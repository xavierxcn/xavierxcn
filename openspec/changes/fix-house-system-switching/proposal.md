# 提案：修复本命星盘宫位制切换功能

## 问题描述

用户在本命星盘工具中切换宫位制（如从 Placidus 切换到整宫制）时，星盘图形和宫位数据没有任何变化。

### 现象分析

1. 控制台显示宫位制参数已正确传递（如 `House system: W`）
2. WASM 模块返回的 `houses` 数据始终保持不变
3. 即使选择"等宫制(E)"，返回的宫位间距也不是 30°，说明始终使用默认的 Placidus

### 根本原因

通过分析 [astro-sweph](https://github.com/astroahava/astro-sweph) 的 C 源码，发现问题出在 **JavaScript 字符串参数传递给 WASM 的方式**。

#### C 代码分析

```c
// astro.c 中的关键代码
const char *get(int year, int month, int day, int hour, int minute, int second,
                int lonG, int lonM, int lonS, char *lonEW, int latG, int latM,
                int latS, char *latNS, char *iHouse)
{
    // ...
    swe_houses_ex(julian_day, calculation_flags, latitude, longitude,
                  (int)*iHouse, house_cusps, angles);
    // ...
}
```

`(int)*iHouse` 表示：解引用 `char *` 指针获取第一个字符，然后转换为 ASCII 码。

例如：
- "P" → 'P' → 80 (Placidus)
- "W" → 'W' → 87 (Whole Sign)
- "E" → 'E' → 69 (Equal)

#### JavaScript 传参问题

当前代码直接传递 JavaScript 字符串：

```javascript
Module._get(year, month, day, hour, minute, 0,
    lonDeg, lonMin, lonSec, lonDir,  // lonDir = "E" 或 "W"
    latDeg, latMin, latSec, latDir,  // latDir = "N" 或 "S"
    hsys);                           // hsys = "P", "K", "W" 等
```

但 Emscripten WASM 模块对于 `char *` 类型参数，**需要传递指向内存的指针（数字）**，而不是 JavaScript 字符串。

当 JavaScript 传递字符串 "W" 时：
1. 如果 Emscripten 没有正确配置字符串转换，字符串会被强制转换为数字
2. "W" 转换为 NaN 或 0
3. 导致 `*iHouse` 解引用得到错误的值

## 解决方案

### 方案一：使用 Module.ccall（推荐）

使用 Emscripten 提供的 `ccall` 函数，自动处理字符串转换：

```javascript
const result = Module.ccall('get', 'string',
    ['number', 'number', 'number', 'number', 'number', 'number',  // date/time
     'number', 'number', 'number', 'string',   // longitude
     'number', 'number', 'number', 'string',   // latitude
     'string'],                                 // house system
    [year, month, day, hour, minute, 0,
     lonDeg, lonMin, lonSec, lonDir,
     latDeg, latMin, latSec, latDir,
     hsys]
);
const chartData = JSON.parse(result);
```

**优点**：代码简洁，Emscripten 自动处理内存分配和释放
**缺点**：需要确认当前 WASM 编译时包含了 ccall 支持

### 方案二：手动分配内存

手动分配内存并传递指针：

```javascript
function allocateString(str) {
    const len = Module.lengthBytesUTF8(str) + 1;
    const ptr = Module._malloc(len);
    Module.stringToUTF8(str, ptr, len);
    return ptr;
}

function freeString(ptr) {
    Module._free(ptr);
}

// 使用
const lonDirPtr = allocateString(lonDir);
const latDirPtr = allocateString(latDir);
const hsysPtr = allocateString(hsys);

try {
    const resultPtr = Module._get(
        year, month, day, hour, minute, 0,
        lonDeg, lonMin, lonSec, lonDirPtr,
        latDeg, latMin, latSec, latDirPtr,
        hsysPtr
    );
    const resultStr = Module.UTF8ToString(resultPtr);
    const chartData = JSON.parse(resultStr);
} finally {
    freeString(lonDirPtr);
    freeString(latDirPtr);
    freeString(hsysPtr);
}
```

**优点**：不依赖 ccall，兼容性更好
**缺点**：代码较繁琐，需要手动管理内存

### 方案三：传递 ASCII 码（备选）

如果 WASM 函数签名允许，直接传递 ASCII 码：

```javascript
const lonDir = lon >= 0 ? 69 : 87;  // 'E' = 69, 'W' = 87
const latDir = lat >= 0 ? 78 : 83;  // 'N' = 78, 'S' = 83
const hsys = houseSystem.value.charCodeAt(0);
```

**注意**：之前尝试过这个方案，导致 houses 数据为空。这说明 C 函数确实期望 `char *` 指针，不能直接传数字。

## 实施计划

### Phase 1：验证 ccall 可用性

1. 检查当前 WASM 模块是否包含 ccall 支持
2. 在控制台测试 `typeof Module.ccall`

### Phase 2：实现修复

根据 Phase 1 结果选择方案一或方案二实现。

### Phase 3：测试验证

1. 测试 Placidus (P) - 默认系统，宫位间距不等
2. 测试 Whole Sign (W) - 每个宫位精确 30°，从 ASC 所在星座 0° 开始
3. 测试 Equal (E) - 每个宫位精确 30°，从 ASC 开始
4. 测试 Koch (K) - 与 Placidus 类似但计算方式不同

### Phase 4：清理

1. 移除调试日志
2. 更新文档

## 验收标准

1. 切换到整宫制(W)时，每个宫位角度相差精确 30°
2. 切换到等宫制(E)时，每个宫位角度相差精确 30°
3. 切换宫位制后，星盘图形的宫位线位置随之变化
4. 切换宫位制后，宫头位置表格数据随之更新
5. 设置在刷新后保持

## 参考资料

- [astro-sweph GitHub](https://github.com/astroahava/astro-sweph)
- [Emscripten 字符串处理文档](https://emscripten.org/docs/porting/connecting_cpp_and_javascript/Interacting-with-code.html#interacting-with-code-ccall-cwrap)
- [Swiss Ephemeris 宫位系统文档](https://www.astro.com/swisseph/swephprg.htm#_Toc112948985)

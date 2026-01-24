# Proposal: improve-natal-chart-ux

## Why

当前本命星盘工具需要用户手动填写所有信息后点击"生成星盘"按钮，这增加了使用门槛。用户体验可以通过以下方式显著改善：

1. 默认显示当前时间和位置的星盘，让用户立即看到结果
2. 实时更新星盘，减少操作步骤
3. 允许用户自定义显示哪些天体，避免界面过于拥挤

## What Changes

### 1. 实时生成星盘

- 移除"生成星盘"按钮
- 任何输入变化时自动重新计算并渲染星盘
- 添加防抖（debounce）避免频繁计算

### 2. 智能默认值

- **当前时间**: 默认使用用户浏览器的当前日期和时间
- **当前时区**: 自动检测浏览器时区
- **当前位置**: 通过 IP 地理定位 API 获取用户大致位置
  - 使用免费 API（如 ip-api.com）
  - 作为备选默认值，用户仍可手动搜索

### 3. 天体选择器

添加可折叠的天体选择面板：

**默认显示（主要行星）：**
- 太阳、月亮、水星、金星、火星
- 木星、土星、天王星、海王星、冥王星

**可选显示（小行星和特殊点）：**
- 凯龙星 (Chiron)
- 莉莉丝/黑月 (Lilith)
- 北交点 (North Node)
- 南交点 (South Node)
- 福点 (Part of Fortune)

### 4. 界面调整

- 所有天体名称使用中文显示
- 添加天体选择区域（可折叠面板或复选框组）
- 优化加载状态（实时更新时使用轻量级指示器）

## Technical Approach

### IP 地理定位

```javascript
// 使用免费 API 获取位置
async function getLocationByIP() {
    const response = await fetch('http://ip-api.com/json/?fields=lat,lon,city,timezone');
    return response.json();
}
```

### 实时更新（防抖）

```javascript
let updateTimeout;
function scheduleUpdate() {
    clearTimeout(updateTimeout);
    updateTimeout = setTimeout(generateChart, 500);
}

// 所有输入添加事件监听
inputs.forEach(el => el.addEventListener('input', scheduleUpdate));
```

### 天体选择器数据

```javascript
const CELESTIAL_BODIES = {
    main: [
        { id: 'Sun', cn: '太阳', default: true },
        { id: 'Moon', cn: '月亮', default: true },
        // ...
    ],
    asteroids: [
        { id: 'Chiron', cn: '凯龙星', default: false },
        { id: 'Lilith', cn: '莉莉丝', default: false },
        // ...
    ]
};
```

## Scope

- 修改 `content/tools/natal-chart/template.html`
- 添加 IP 地理定位功能
- 添加天体选择 UI
- 实现实时更新逻辑
- 更新星盘渲染以支持可选天体

## Out of Scope

- 精确 GPS 定位（需要用户授权，复杂度高）
- 天体图标自定义
- 多语言支持（保持中文）

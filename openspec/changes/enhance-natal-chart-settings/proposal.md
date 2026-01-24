# Proposal: enhance-natal-chart-settings

## Why

本命星盘工具存在以下用户体验问题：

1. **地名检索速度慢**：当前使用 Nominatim API，响应时间较长（1-3秒），影响用户输入流畅度
2. **经纬度不可见**：用户选择城市后无法看到具体的经纬度数值，缺乏透明度
3. **设置面板布局不合理**：天体选择放在左侧输入面板，占用空间且与星盘分离；相位设置完全缺失

## What Changes

### 1. 优化地名检索性能

- 切换到更快的地理编码 API（如 Photon API 或添加本地缓存）
- 添加常用中国城市的本地数据库作为优先匹配
- 保留 Nominatim 作为备选

### 2. 显示经纬度

- 在城市输入框下方添加经纬度显示
- 格式：`经度: 116.4074° E  纬度: 39.9042° N`
- 支持用户直接编辑经纬度

### 3. 重构星盘设置面板

将天体和相位设置从左侧面板移到星盘区域：

**UI 设计：**
- 在星盘图上方或下方添加"设置"按钮（齿轮图标）
- 点击打开模态弹窗/抽屉面板
- 弹窗包含三个标签页：
  - **天体选择**：复选框选择要显示的天体
  - **相位选择**：复选框选择要显示的相位类型
  - **容许度设置**：滑块或数字输入调整各相位的容许度

**相位选项：**
| 相位 | 中文名 | 角度 | 默认容许度 | 默认显示 |
|------|--------|------|-----------|---------|
| Conjunction | 合相 | 0° | 8° | ✓ |
| Opposition | 冲相 | 180° | 8° | ✓ |
| Trine | 三分相 | 120° | 8° | ✓ |
| Square | 四分相 | 90° | 8° | ✓ |
| Sextile | 六分相 | 60° | 6° | ✓ |
| Quincunx | 梅花相 | 150° | 3° | ✗ |

## Technical Approach

### 地名检索优化

```javascript
// 本地城市数据（常用中国城市）
const LOCAL_CITIES = [
    { name: '北京', lat: 39.9042, lon: 116.4074 },
    { name: '上海', lat: 31.2304, lon: 121.4737 },
    { name: '广州', lat: 23.1291, lon: 113.2644 },
    // ... 更多城市
];

// 优先本地匹配
function searchCity(query) {
    const localMatches = LOCAL_CITIES.filter(c =>
        c.name.includes(query)
    );
    if (localMatches.length > 0) {
        return localMatches; // 即时返回
    }
    // 降级到 API
    return fetchFromAPI(query);
}
```

### 设置弹窗结构

```html
<div class="settings-modal">
    <div class="settings-tabs">
        <button class="tab active" data-tab="planets">天体</button>
        <button class="tab" data-tab="aspects">相位</button>
        <button class="tab" data-tab="orbs">容许度</button>
    </div>
    <div class="settings-content">
        <!-- 各标签页内容 -->
    </div>
</div>
```

## Scope

- 修改 `content/tools/natal-chart/template.html`
- 添加本地城市数据
- 重构 CSS 和 JavaScript

## Out of Scope

- 后端 API 实现（保持纯前端）
- 完整的全球城市数据库
- 用户账户系统保存设置

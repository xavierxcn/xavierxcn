# Proposal: add-timezone-selection

## Why

本命星盘工具目前使用 UTC 时间进行计算，但用户输入的通常是本地时间。大多数用户位于东八区（中国），需要手动进行时区转换才能得到准确的星盘。这增加了使用门槛，容易导致计算错误。

## What Changes

为本命星盘工具添加时区选择功能，支持用户选择出生地的时区，系统自动将本地时间转换为 UTC 时间进行星历计算。

### 功能特性

1. **时区选择器**
   - 在出生地点下方添加时区选择下拉框
   - 默认选择 UTC+8 (中国标准时间)
   - 支持常用时区列表（UTC-12 到 UTC+14）
   - 选择城市后自动推断时区（可选优化）

2. **时间转换**
   - 用户输入本地出生时间
   - 系统根据选择的时区自动转换为 UTC
   - UTC 时间用于 Swiss Ephemeris 计算

3. **显示优化**
   - 在结果中显示输入的本地时间和对应的 UTC 时间
   - 便于用户确认时间转换正确

## Technical Approach

### 时区处理

使用 JavaScript 内置的时区偏移量计算：
- 时区以 UTC 偏移量表示（如 +8、-5）
- 转换公式：`UTC时间 = 本地时间 - 时区偏移`

### 实现方案

```javascript
// 时区选择器数据
const TIMEZONES = [
  { value: 8, label: 'UTC+8 (中国/新加坡/台湾)' },
  { value: 9, label: 'UTC+9 (日本/韩国)' },
  { value: 0, label: 'UTC+0 (英国/格林尼治)' },
  // ... 更多时区
];

// 时间转换
function localToUTC(localHour, localMinute, tzOffset) {
  let utcHour = localHour - tzOffset;
  let dayOffset = 0;

  if (utcHour < 0) {
    utcHour += 24;
    dayOffset = -1;
  } else if (utcHour >= 24) {
    utcHour -= 24;
    dayOffset = 1;
  }

  return { hour: utcHour, minute: localMinute, dayOffset };
}
```

### UI 变更

在出生时间输入区域后添加：
```html
<div class="input-section">
    <label class="input-label">时区</label>
    <select id="timezone" class="house-select">
        <option value="8" selected>UTC+8 (中国/新加坡/台湾)</option>
        <!-- 更多选项 -->
    </select>
</div>
```

## Scope

- 修改 `content/tools/natal-chart/template.html`
- 添加时区选择 UI
- 添加时间转换逻辑
- 更新结果显示

## Out of Scope

- 夏令时自动处理（用户需手动选择）
- 根据城市自动获取时区（需要额外 API）
- 历史时区变更处理

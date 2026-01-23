# Change: 重新设计占星板块

## Why

当前占星板块的设计过于简陋，使用 emoji 表情显得不够专业，内容也不够完整。需要：
1. 采用现代杂志风格重新设计视觉呈现
2. 使用正规的 Unicode 占星符号替代 emoji
3. 补充完整的占星基础内容，并改善行文风格

## What Changes

### 视觉设计
- 采用现代杂志风格：大图留白、清晰的版式层级、优雅的卡片设计
- 使用 Unicode 占星符号（♈♉♊♋♌♍♎♏♐♑♒♓）
- 移除分类页面中使用的 emoji 图标
- 添加精致的视觉细节：渐变、阴影、微动效

### 内容完善
- 12 星座完整内容
- 10 行星完整内容（含日月）
- 12 宫位完整内容
- 主要相位完整内容
- 行文风格更加人性化、有温度

## Impact

- Affected specs: render
- Affected files:
  - `themes/default/templates/astrology.html`
  - `themes/default/templates/astrology-category.html`
  - `themes/default/static/css/style.css`
  - `content/astrology/**/*.md` - 所有内容文件

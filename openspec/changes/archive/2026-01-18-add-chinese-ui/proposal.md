# Change: 中文优先 UI

## Why

博客主要面向中文读者，UI 文本应优先使用中文，提供更好的用户体验。后续会添加国际化支持，但当前阶段中文优先。

## What Changes

- **导航文本**: 首页、归档、标签、分类等导航文本使用中文
- **分页导航**: 上一页、下一页、页码等使用中文
- **搜索提示**: 搜索框占位符和提示使用中文
- **日期格式**: 使用中文日期格式

## Impact

- Affected specs: `render` - UI 文本显示规则
- Affected code:
  - `themes/default/templates/` - 所有模板中的 UI 文本
  - `themes/default/templates/partials/` - 导航、分页等组件

# Change: 添加静态搜索功能

## Why

随着文章数量增加，用户需要快速找到相关内容。静态搜索提供无需服务端的全文搜索能力。

## What Changes

- **搜索索引生成**: 构建时生成 `search-index.json`，包含文章标题、摘要、标签等
- **搜索 UI 组件**: 导航栏搜索框 + 搜索结果页面
- **客户端搜索**: JavaScript 实现的即时搜索，支持标题、内容、标签匹配

## Impact

- Affected specs: `build` - 添加搜索索引生成逻辑
- Affected code:
  - `generator/src/build.rs` - 生成搜索索引
  - `themes/default/templates/` - 添加搜索 UI
  - `themes/default/static/js/search.js` - 客户端搜索逻辑
  - `themes/default/static/css/style.css` - 搜索样式

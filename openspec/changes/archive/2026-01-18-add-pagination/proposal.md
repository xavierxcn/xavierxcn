# Change: 添加分页功能

## Why

随着文章数量增加，首页和列表页面会变得过长，需要分页显示以提升用户体验。

## What Changes

- **首页分页**: 首页显示最新 N 篇文章，超出部分通过 `/page/2/`, `/page/3/` 访问
- **归档页分页**: 归档页面支持分页 `/archive/page/{n}/`
- **标签页分页**: 每个标签页面支持分页 `/tags/{tag}/page/{n}/`
- **分类页分页**: 每个分类页面支持分页 `/categories/{cat}/page/{n}/`
- **分页导航组件**: 显示上一页/下一页和页码

## Impact

- Affected specs: `build` - 修改页面生成逻辑以支持分页
- Affected code:
  - `generator/src/build.rs` - 添加分页逻辑
  - `themes/default/templates/` - 更新模板添加分页导航
  - `themes/default/static/css/style.css` - 分页样式
- 配置: `pagination.posts_per_page` (已存在，将被启用)

# Change: 新增占星板块

## Why

博客需要一个专门的占星知识库板块，用于展示星座、行星、宫位、相位等占星学基础知识。这个板块将作为一个独立的内容分类，与文章、工具板块并列。

## What Changes

- 新增 `content/astrology/` 目录结构，包含四个子分类：
  - `signs/` - 12 星座
  - `planets/` - 行星
  - `houses/` - 12 宫位
  - `aspects/` - 相位
- 每个分类使用 Markdown 文件存储内容，支持 front matter 元数据
- 新增占星板块的模板：
  - `astrology.html` - 占星首页（四个分类的入口）
  - `astrology-category.html` - 分类汇总页（展示该分类下所有条目）
- 添加客户端筛选/搜索功能
- 在导航栏添加占星入口

## Impact

- Affected specs: content, build, render, config
- Affected code:
  - `generator/src/content/mod.rs` - 新增占星内容加载逻辑
  - `generator/src/build.rs` - 新增占星页面生成逻辑
  - `themes/default/templates/` - 新增占星相关模板
  - `config.yaml` - 添加导航项

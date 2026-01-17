# Change: 实现静态网站生成核心功能

## Why

当前生成器只有一个空壳框架，无法完成博客内容到静态网站的转换。需要实现完整的静态页面生成流程，将 Markdown 内容转换为可部署的 HTML 网站。

## What Changes

- **配置模块**: 读取和解析 `config.yaml` 站点配置
- **内容模块**: 解析 Markdown 文件，提取 YAML front matter 和正文内容
- **渲染模块**: 将 Markdown 转换为 HTML，支持代码语法高亮
- **模板模块**: 加载 Tera 模板，渲染最终 HTML 页面
- **构建模块**: 协调整个构建流程，生成完整静态网站
  - 文章页面 (posts)
  - 独立页面 (pages)
  - 首页文章列表
  - 归档页面
  - 标签/分类页面
  - 静态资源复制

## Impact

- Affected specs: 新增 `config`, `content`, `render`, `build` 四个能力规范
- Affected code:
  - `generator/src/main.rs` - CLI 入口重构
  - `generator/src/config.rs` - 新增配置模块
  - `generator/src/content/` - 新增内容解析模块
  - `generator/src/render/` - 新增渲染模块
  - `generator/src/build.rs` - 新增构建协调模块

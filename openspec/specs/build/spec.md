# build Specification

## Purpose
TBD - created by archiving change add-static-generation. Update Purpose after archive.
## Requirements
### Requirement: Build Orchestration

系统 SHALL 协调完整的静态网站构建流程。

#### Scenario: 执行完整构建

- **WHEN** 用户运行 `xavierxcn-generator build` 命令
- **THEN** 系统依次执行: 读取配置 → 加载内容 → 渲染页面 → 输出文件
- **AND** 在控制台输出构建进度

#### Scenario: 清理输出目录

- **WHEN** 构建开始
- **THEN** 系统清空 `output/` 目录 (如存在)
- **AND** 重新创建目录结构

### Requirement: Post Page Generation

系统 SHALL 为每篇文章生成独立的 HTML 页面。

#### Scenario: 生成文章页面

- **WHEN** 文章列表包含文章
- **THEN** 系统为每篇文章生成 `output/posts/{slug}/index.html`

#### Scenario: 文章按日期排序

- **WHEN** 生成文章相关页面 (首页、归档)
- **THEN** 文章按发布日期降序排列 (最新在前)

### Requirement: Page Generation

系统 SHALL 为每个独立页面生成 HTML 页面。

#### Scenario: 生成独立页面

- **WHEN** 页面列表包含页面
- **THEN** 系统为每个页面生成 `output/{slug}/index.html`

### Requirement: Index Page Generation

系统 SHALL 生成首页，支持分页显示文章列表。

#### Scenario: 首页文章列表

- **WHEN** 生成首页
- **THEN** 系统在 `output/index.html` 生成首页
- **AND** 显示最新的 N 篇文章（N 由 `pagination.posts_per_page` 配置）

#### Scenario: 首页分页

- **WHEN** 文章总数超过每页显示数量
- **THEN** 系统生成分页页面 `output/page/{n}/index.html`
- **AND** 每个分页页面包含分页导航（上一页/下一页/页码）

### Requirement: Archive Page Generation

系统 SHALL 生成归档页面，支持分页。

#### Scenario: 归档页面

- **WHEN** 生成归档页面
- **THEN** 系统在 `output/archive/index.html` 生成归档页
- **AND** 按日期列出文章，支持分页

#### Scenario: 归档页分页

- **WHEN** 归档文章总数超过每页显示数量
- **THEN** 系统生成分页页面 `output/archive/page/{n}/index.html`

### Requirement: Tag Page Generation

系统 SHALL 为每个标签生成聚合页面，支持分页。

#### Scenario: 标签页面

- **WHEN** 文章包含标签
- **THEN** 系统为每个唯一标签生成 `output/tags/{tag}/index.html`
- **AND** 页面列出该标签下的文章，支持分页

#### Scenario: 标签页分页

- **WHEN** 某标签下文章数超过每页显示数量
- **THEN** 系统生成分页页面 `output/tags/{tag}/page/{n}/index.html`

### Requirement: Category Page Generation

系统 SHALL 为每个分类生成聚合页面，支持分页。

#### Scenario: 分类页面

- **WHEN** 文章包含分类
- **THEN** 系统为每个唯一分类生成 `output/categories/{category}/index.html`
- **AND** 页面列出该分类下的文章，支持分页

#### Scenario: 分类页分页

- **WHEN** 某分类下文章数超过每页显示数量
- **THEN** 系统生成分页页面 `output/categories/{category}/page/{n}/index.html`

### Requirement: Static Asset Copying

系统 SHALL 复制静态资源到输出目录。

#### Scenario: 复制主题静态资源

- **WHEN** 构建完成页面生成
- **THEN** 系统复制 `themes/{theme}/static/` 下所有文件到 `output/static/`

#### Scenario: 复制全局静态资源

- **WHEN** 根目录存在 `static/` 文件夹
- **THEN** 系统复制其内容到 `output/static/` (覆盖同名文件)


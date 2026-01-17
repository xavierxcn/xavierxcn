## MODIFIED Requirements

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

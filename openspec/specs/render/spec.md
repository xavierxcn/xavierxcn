# render Specification

## Purpose
TBD - created by archiving change add-static-generation. Update Purpose after archive.
## Requirements
### Requirement: Markdown Rendering

系统 SHALL 将 Markdown 内容转换为 HTML。

#### Scenario: 基础 Markdown 渲染

- **WHEN** 内容包含标准 Markdown 语法 (标题、段落、列表、链接、图片等)
- **THEN** 系统使用 pulldown-cmark 渲染为对应的 HTML

#### Scenario: GFM 扩展支持

- **WHEN** 内容包含 GFM 扩展语法 (表格、任务列表、删除线)
- **THEN** 系统正确渲染这些扩展语法

### Requirement: Syntax Highlighting

系统 SHALL 对代码块进行服务端语法高亮。

#### Scenario: 代码块高亮

- **WHEN** Markdown 包含带语言标识的代码块 (如 \`\`\`rust)
- **THEN** 系统使用 syntect 进行语法高亮
- **AND** 生成带有颜色样式的 HTML

#### Scenario: 无语言标识的代码块

- **WHEN** 代码块没有指定语言
- **THEN** 系统作为纯文本渲染，不进行高亮

#### Scenario: Mermaid 代码块

- **WHEN** 代码块语言为 `mermaid`
- **THEN** 系统保留原始代码块内容 (客户端渲染)
- **AND** 添加 `class="mermaid"` 供前端 JS 处理

### Requirement: Template Rendering

系统 SHALL 使用 Tera 模板引擎渲染最终 HTML 页面。

#### Scenario: 加载主题模板

- **WHEN** 构建开始
- **THEN** 系统从 `themes/{theme}/templates/` 加载所有模板文件

#### Scenario: 渲染文章页面

- **WHEN** 渲染单篇文章
- **THEN** 系统使用 `post.html` 模板
- **AND** 提供 `site`, `config`, `post`, `nav` 变量

#### Scenario: 渲染独立页面

- **WHEN** 渲染独立页面
- **THEN** 系统使用 `page.html` 模板
- **AND** 提供 `site`, `config`, `page`, `nav` 变量

#### Scenario: 渲染首页

- **WHEN** 渲染首页
- **THEN** 系统使用 `index.html` 模板
- **AND** 提供 `site`, `config`, `posts`, `nav` 变量

#### Scenario: 渲染归档页

- **WHEN** 渲染归档页面
- **THEN** 系统使用 `archive.html` 模板
- **AND** 提供按日期排序的完整文章列表

### Requirement: Astrology Template Rendering

系统 SHALL 提供占星板块的模板渲染支持，采用现代杂志风格设计。

#### Scenario: 渲染占星首页

- **WHEN** 渲染 `astrology.html` 模板
- **THEN** 模板可访问 `site`、`nav`、`categories` 变量
- **AND** 页面采用现代杂志风格布局
- **AND** 使用 Unicode 占星符号而非 emoji

#### Scenario: 渲染分类汇总页

- **WHEN** 渲染 `astrology-category.html` 模板
- **THEN** 模板可访问 `site`、`nav`、`category`、`items` 变量
- **AND** 页面采用卡片网格布局，具有优雅的视觉层级
- **AND** 使用 Unicode 占星符号展示星座/行星标识

### Requirement: Astrology Client-Side Filtering

系统 SHALL 提供客户端筛选功能。

#### Scenario: 按关键词筛选

- **WHEN** 用户在分类页输入搜索关键词
- **THEN** 实时筛选显示匹配的条目
- **AND** 匹配标题、符号、关键词字段

#### Scenario: 按属性筛选

- **WHEN** 分类为星座（signs）
- **THEN** 提供按元素（火、土、风、水）筛选的选项
- **AND** 提供按模式（本位、固定、变动）筛选的选项


# content Specification

## Purpose
TBD - created by archiving change add-static-generation. Update Purpose after archive.
## Requirements
### Requirement: Front Matter Parsing

系统 SHALL 从 Markdown 文件头部解析 YAML front matter。

#### Scenario: 成功解析 front matter

- **WHEN** Markdown 文件以 `---` 分隔的 YAML 块开头
- **THEN** 系统提取并解析 YAML 内容
- **AND** 返回 front matter 数据和正文内容

#### Scenario: 无 front matter

- **WHEN** Markdown 文件不包含 front matter
- **THEN** 系统使用默认元数据 (标题从文件名生成)
- **AND** 整个文件内容作为正文

### Requirement: Post Loading

系统 SHALL 从 `content/posts/` 目录加载所有文章。

#### Scenario: 加载文章列表

- **WHEN** 构建过程扫描 `content/posts/` 目录
- **THEN** 系统读取所有 `.md` 文件
- **AND** 解析每个文件的 front matter 和正文
- **AND** 返回 Post 对象列表

#### Scenario: 文章元数据

- **WHEN** 文章包含完整 front matter
- **THEN** 系统解析 `title`, `date`, `updated`, `tags`, `categories`, `summary`, `draft` 字段

#### Scenario: 跳过草稿

- **WHEN** 文章 front matter 中 `draft: true`
- **THEN** 系统跳过该文章，不包含在生成列表中

#### Scenario: 文章解析失败

- **WHEN** 单篇文章解析失败 (front matter 格式错误等)
- **THEN** 系统记录警告日志
- **AND** 跳过该文章继续处理其他文章

### Requirement: Page Loading

系统 SHALL 从 `content/pages/` 目录加载所有独立页面。

#### Scenario: 加载页面列表

- **WHEN** 构建过程扫描 `content/pages/` 目录
- **THEN** 系统读取所有 `.md` 文件
- **AND** 解析每个文件的 front matter 和正文
- **AND** 返回 Page 对象列表

#### Scenario: 页面元数据

- **WHEN** 页面包含 front matter
- **THEN** 系统解析 `title` 字段 (页面不需要 date、tags 等)

### Requirement: Slug Generation

系统 SHALL 从文件名生成 URL slug。

#### Scenario: 从文件名生成 slug

- **WHEN** 文件名为 `hello-world.md`
- **THEN** 生成的 slug 为 `hello-world`

#### Scenario: 中文文件名

- **WHEN** 文件名包含中文字符
- **THEN** 系统保留中文字符作为 slug (URL 编码在输出时处理)

### Requirement: Astrology Item Loading

系统 SHALL 从 `content/astrology/` 目录加载占星内容，支持四个分类：signs（星座）、planets（行星）、houses（宫位）、aspects（相位）。

#### Scenario: 加载占星内容

- **WHEN** `content/astrology/{category}/` 目录存在
- **THEN** 加载该目录下所有 `.md` 文件作为占星条目
- **AND** 按分类组织返回

#### Scenario: 占星条目元数据

- **WHEN** 解析占星条目的 Markdown 文件
- **THEN** 提取 front matter 中的字段：
  - `title`（必需）- 标题
  - `symbol`（可选）- 符号
  - `element`（可选）- 元素（火、土、风、水）
  - `modality`（可选）- 模式（本位、固定、变动）
  - `ruling_planet`（可选）- 守护星
  - `keywords`（可选）- 关键词列表
  - `summary`（可选）- 简短描述
- **AND** 生成 slug 和 URL

#### Scenario: 占星目录不存在

- **WHEN** `content/astrology/` 目录不存在
- **THEN** 返回空的占星数据集合
- **AND** 不产生错误


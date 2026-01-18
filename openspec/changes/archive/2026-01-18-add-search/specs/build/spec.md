## NEW Requirements

### Requirement: Search Index Generation

系统 SHALL 在构建时生成搜索索引文件。

#### Scenario: 生成搜索索引

- **WHEN** 执行构建命令
- **THEN** 系统在 `output/search-index.json` 生成搜索索引
- **AND** 索引包含所有文章的 slug、title、summary、tags、date 字段

#### Scenario: 搜索索引内容

- **GIVEN** 文章包含标题、摘要和标签
- **WHEN** 生成搜索索引
- **THEN** 每篇文章的索引条目包含：
  - `slug`: 文章 URL 路径
  - `title`: 文章标题
  - `summary`: 文章摘要（前 200 字符或自定义摘要）
  - `tags`: 文章标签数组
  - `date`: 发布日期

### Requirement: Search UI

系统 SHALL 提供搜索用户界面。

#### Scenario: 搜索输入框

- **WHEN** 用户访问任何页面
- **THEN** 导航栏显示搜索输入框
- **AND** 输入框支持即时搜索

#### Scenario: 搜索结果展示

- **WHEN** 用户输入搜索关键词
- **THEN** 系统实时显示匹配结果
- **AND** 结果包含文章标题、摘要和日期
- **AND** 关键词在结果中高亮显示

### Requirement: Client-side Search

系统 SHALL 实现客户端搜索功能。

#### Scenario: 搜索匹配

- **WHEN** 用户输入搜索关键词
- **THEN** 系统在以下字段中搜索匹配：
  - 文章标题
  - 文章摘要
  - 文章标签
- **AND** 匹配不区分大小写

#### Scenario: 搜索结果排序

- **WHEN** 搜索返回多个结果
- **THEN** 结果按相关性排序（标题匹配优先于内容匹配）

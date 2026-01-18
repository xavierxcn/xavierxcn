## NEW Requirements

### Requirement: Chinese Navigation Text

系统 SHALL 使用中文导航文本。

#### Scenario: 主导航

- **WHEN** 渲染页面导航栏
- **THEN** 导航链接使用中文文本：
  - "首页" (Home)
  - "归档" (Archive)
  - "标签" (Tags)
  - "分类" (Categories)
  - "关于" (About)

### Requirement: Chinese Pagination Text

系统 SHALL 使用中文分页导航文本。

#### Scenario: 分页导航

- **WHEN** 渲染分页组件
- **THEN** 分页使用中文文本：
  - "上一页" (Previous)
  - "下一页" (Next)
  - "第 N 页，共 M 页" (Page N of M)

### Requirement: Chinese Search Text

系统 SHALL 使用中文搜索提示文本。

#### Scenario: 搜索框

- **WHEN** 渲染搜索输入框
- **THEN** 占位符显示 "搜索文章..."
- **AND** 无结果时显示 "未找到相关文章"

### Requirement: Chinese Date Format

系统 SHALL 使用中文友好的日期格式。

#### Scenario: 日期显示

- **WHEN** 渲染文章日期
- **THEN** 使用格式 "YYYY年MM月DD日" 或 "YYYY-MM-DD"

### Requirement: Chinese UI Text

系统 SHALL 使用中文其他 UI 文本。

#### Scenario: 通用文本

- **WHEN** 渲染页面
- **THEN** 使用中文文本：
  - "阅读全文" (Read more)
  - "返回顶部" (Back to top)
  - "文章标签" (Tags)
  - "相关文章" (Related posts)

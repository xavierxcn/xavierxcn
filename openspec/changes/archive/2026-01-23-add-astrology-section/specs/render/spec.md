## ADDED Requirements

### Requirement: Astrology Template Rendering

系统 SHALL 提供占星板块的模板渲染支持。

#### Scenario: 渲染占星首页

- **WHEN** 渲染 `astrology.html` 模板
- **THEN** 模板可访问 `site`、`nav`、`categories` 变量
- **AND** `categories` 包含四个分类及其条目数量

#### Scenario: 渲染分类汇总页

- **WHEN** 渲染 `astrology-category.html` 模板
- **THEN** 模板可访问 `site`、`nav`、`category`、`items` 变量
- **AND** `items` 为该分类下所有占星条目的列表

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

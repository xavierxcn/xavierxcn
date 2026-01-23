## MODIFIED Requirements

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

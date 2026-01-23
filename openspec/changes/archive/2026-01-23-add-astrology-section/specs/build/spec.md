## ADDED Requirements

### Requirement: Astrology Index Page Generation

系统 SHALL 生成占星首页，展示四个分类的入口卡片。

#### Scenario: 生成占星首页

- **WHEN** 执行构建
- **THEN** 生成 `/astrology/index.html`
- **AND** 页面包含四个分类卡片（星座、行星、宫位、相位）
- **AND** 每个卡片链接到对应的分类页面

### Requirement: Astrology Category Page Generation

系统 SHALL 为每个占星分类生成汇总页面。

#### Scenario: 生成分类汇总页

- **WHEN** 执行构建
- **AND** 存在占星内容
- **THEN** 为每个分类生成 `/astrology/{category}/index.html`
- **AND** 页面展示该分类下所有条目的标题、符号和简介

#### Scenario: 分类无内容

- **WHEN** 某个分类目录为空
- **THEN** 仍然生成该分类页面
- **AND** 显示"暂无内容"提示

### Requirement: Astrology Search Index Integration

系统 SHALL 将占星内容纳入搜索索引。

#### Scenario: 占星内容可搜索

- **WHEN** 生成搜索索引
- **THEN** 包含所有占星条目
- **AND** 每个条目包含 title、summary、keywords 字段

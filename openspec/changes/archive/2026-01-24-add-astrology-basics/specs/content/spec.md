# Content Spec Delta

## MODIFIED Requirements

### Requirement: Astrology Item Loading

系统 SHALL 从 `content/astrology/` 目录加载占星内容，支持五个分类：basics（基础入门）、signs（星座）、planets（行星）、houses（宫位）、aspects（相位）。

#### Scenario: 加载占星内容

- **WHEN** `content/astrology/{category}/` 目录存在
- **THEN** 加载该目录下所有 `.md` 文件作为占星条目
- **AND** 按分类组织返回

#### Scenario: 基础入门分类

- **WHEN** `content/astrology/basics/` 目录存在
- **THEN** 加载该目录下所有 `.md` 文件作为入门文章
- **AND** 在占星页面中显示为第一个分类（order: 0）

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
  - `order`（可选）- 排序权重
- **AND** 生成 slug 和 URL

#### Scenario: 占星目录不存在

- **WHEN** `content/astrology/` 目录不存在
- **THEN** 返回空的占星数据集合
- **AND** 不产生错误

## ADDED Requirements

### Requirement: Configuration Loading

系统 SHALL 从 `config.yaml` 文件读取站点配置。

#### Scenario: 成功加载配置

- **WHEN** 用户运行构建命令且 `config.yaml` 存在
- **THEN** 系统读取并解析配置文件
- **AND** 验证必填字段 (`site.title`, `site.url`, `build.content_dir`, `build.output_dir`)
- **AND** 返回配置对象供后续模块使用

#### Scenario: 配置文件缺失

- **WHEN** 用户运行构建命令但 `config.yaml` 不存在
- **THEN** 系统终止构建并输出错误消息 "Configuration file not found: config.yaml"

#### Scenario: 配置格式错误

- **WHEN** `config.yaml` 存在但 YAML 格式无效
- **THEN** 系统终止构建并输出 YAML 解析错误详情

### Requirement: Configuration Schema

配置文件 SHALL 支持以下配置项:

#### Scenario: 站点配置

- **WHEN** 配置包含 `site` 段
- **THEN** 系统解析 `title`, `description`, `author`, `url`, `language` 字段

#### Scenario: 构建配置

- **WHEN** 配置包含 `build` 段
- **THEN** 系统解析 `content_dir`, `output_dir`, `theme` 字段

#### Scenario: Markdown 配置

- **WHEN** 配置包含 `markdown` 段
- **THEN** 系统解析 `syntax_highlight`, `syntax_theme`, `mermaid`, `math` 字段

#### Scenario: 导航配置

- **WHEN** 配置包含 `nav` 段
- **THEN** 系统解析导航项列表，每项包含 `title` 和 `url`

# tools Specification

## Purpose
TBD - created by archiving change add-tools-section. Update Purpose after archive.
## Requirements
### Requirement: Tool Metadata Loading

系统 SHALL 从 `content/tools/_meta.yaml` 文件加载工具元数据。

#### Scenario: 加载工具元数据

- **WHEN** 构建过程读取 `content/tools/_meta.yaml`
- **THEN** 系统解析 YAML 文件中的 `tools` 数组
- **AND** 返回 ToolMeta 对象列表

#### Scenario: 元数据字段

- **WHEN** 解析单个工具元数据
- **THEN** 系统读取 `slug`, `title`, `description`, `icon` 字段
- **AND** `description` 和 `icon` 为可选字段

#### Scenario: 元数据文件不存在

- **WHEN** `content/tools/_meta.yaml` 文件不存在
- **THEN** 系统返回空的工具列表
- **AND** 不生成工具相关页面

### Requirement: Tool Page Copying

系统 SHALL 将工具目录整体复制到输出目录。

#### Scenario: 复制工具页面

- **WHEN** 工具元数据中包含 slug 为 `{slug}` 的工具
- **THEN** 系统将 `content/tools/{slug}/` 目录整体复制到 `output/tools/{slug}/`
- **AND** 保留目录结构和所有文件

#### Scenario: 工具目录不存在

- **WHEN** 工具元数据中包含工具但对应目录不存在
- **THEN** 系统记录警告日志
- **AND** 跳过该工具，继续处理其他工具

### Requirement: Tools List Page Generation

系统 SHALL 生成工具列表页面。

#### Scenario: 生成工具列表页

- **WHEN** 存在至少一个工具
- **THEN** 系统生成 `output/tools/index.html` 工具列表页
- **AND** 使用 `tools.html` 模板渲染
- **AND** 列表中包含所有工具的标题、描述、图标

### Requirement: Markdown to WeChat Tool

系统 SHALL 提供 Markdown 转微信公众号 HTML 格式的工具页面。

#### Scenario: 工具页面结构

- **WHEN** 访问 `/tools/md2wechat/`
- **THEN** 显示完整的工具页面
- **AND** 页面包含导航栏、工具内容、页脚
- **AND** 页面布局应尽可能宽，充分利用屏幕空间

#### Scenario: Markdown 输入转换

- **WHEN** 用户在输入框输入 Markdown 文本
- **THEN** 系统实时解析 Markdown
- **AND** 在预览区域显示微信公众号样式的 HTML

#### Scenario: 代码高亮

- **WHEN** Markdown 包含代码块（\`\`\`language）
- **THEN** 系统使用 highlight.js 进行语法高亮
- **AND** 样式通过内联 CSS 应用（微信兼容）

#### Scenario: Mermaid 图表渲染

- **WHEN** Markdown 包含 mermaid 代码块
- **THEN** 系统使用 Mermaid.js 渲染图表
- **AND** 输出为内嵌 SVG 格式

#### Scenario: 复制到剪贴板

- **WHEN** 用户点击"复制 HTML"按钮
- **THEN** 系统将带有内联样式的 HTML 复制到剪贴板
- **AND** 显示复制成功提示

#### Scenario: 样式内联

- **WHEN** 生成最终 HTML 输出
- **THEN** 所有 CSS 样式必须内联到元素的 style 属性
- **AND** 不依赖外部样式表或 CSS 类

### Requirement: Navigation Integration

系统 SHALL 在导航栏中提供工具入口。

#### Scenario: 导航栏显示工具链接

- **WHEN** 配置文件中包含工具导航项
- **THEN** 导航栏显示"工具"链接
- **AND** 链接指向 `/tools/`


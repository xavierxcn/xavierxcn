# Tools Spec Delta

## MODIFIED Requirements

### Requirement: Tool Page Copying

系统 SHALL 将工具目录中的静态资源复制到输出目录，并渲染工具模板。

#### Scenario: 渲染工具模板

- **WHEN** 工具目录包含 `template.html` 文件
- **THEN** 系统使用 Tera 引擎渲染该模板
- **AND** 输出到 `output/tools/{slug}/index.html`
- **AND** 渲染上下文包含 `site`、`nav`、`config`、`tool` 变量

#### Scenario: 复制工具静态资源

- **WHEN** 工具目录包含非模板文件（如 assets 目录）
- **THEN** 系统将这些文件复制到 `output/tools/{slug}/`
- **AND** 保留目录结构

#### Scenario: 兼容旧版工具页面

- **WHEN** 工具目录不包含 `template.html` 但包含 `index.html`
- **THEN** 系统将整个目录复制到输出目录（保持向后兼容）

### Requirement: Markdown to WeChat Tool

系统 SHALL 提供 Markdown 转微信公众号 HTML 格式的工具页面。

#### Scenario: 工具页面模板继承

- **WHEN** 渲染 md2wechat 工具页面
- **THEN** 使用 `template.html` 模板
- **AND** 模板继承 `base.html`
- **AND** 自动获得主站的 header、footer、搜索功能

#### Scenario: 工具页面导航一致性

- **WHEN** 访问 `/tools/md2wechat/`
- **THEN** 页面头部导航栏与主站完全一致
- **AND** 显示正确的站点标题
- **AND** 包含搜索输入框
- **AND** 包含完整的导航链接（首页、占星、工具、归档、关于）

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

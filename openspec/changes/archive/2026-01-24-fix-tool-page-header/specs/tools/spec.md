# Tools Spec Delta

## MODIFIED Requirements

### Requirement: Markdown to WeChat Tool

系统 SHALL 提供 Markdown 转微信公众号 HTML 格式的工具页面。

#### Scenario: 工具页面导航一致性

- **WHEN** 访问 `/tools/md2wechat/`
- **THEN** 页面头部导航栏与主站完全一致
- **AND** 显示正确的站点标题
- **AND** 包含搜索输入框
- **AND** 包含完整的导航链接（首页、占星、工具、归档、关于）

#### Scenario: 工具页面全局功能

- **WHEN** 访问工具页面
- **THEN** 页面包含 Google Analytics 跟踪代码
- **AND** 搜索功能正常工作
- **AND** 支持主站的暗色模式

#### Scenario: 工具页面样式继承

- **WHEN** 渲染工具页面
- **THEN** 页面引入主站 CSS 文件
- **AND** 导航栏使用主站样式
- **AND** 工具特有样式（编辑器、预览等）保持不变

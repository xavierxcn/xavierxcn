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

### Requirement: Tools List Page Generation

系统 SHALL 生成工具列表页面。

#### Scenario: 生成工具列表页

- **WHEN** 存在至少一个工具
- **THEN** 系统生成 `output/tools/index.html` 工具列表页
- **AND** 使用 `tools.html` 模板渲染
- **AND** 列表中包含所有工具的标题、描述、图标

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

### Requirement: Navigation Integration

系统 SHALL 在导航栏中提供工具入口。

#### Scenario: 导航栏显示工具链接

- **WHEN** 配置文件中包含工具导航项
- **THEN** 导航栏显示"工具"链接
- **AND** 链接指向 `/tools/`

### Requirement: Decoration Budget Display Modes

装修预算助手 SHALL 根据装修方式显示不同的信息详细程度。

#### Scenario: 清包/半包模式显示详细信息

- **WHEN** 用户选择清包或半包装修方式
- **THEN** 预算项目显示详细模式
- **AND** 包含：项目名称、数量输入、单价输入、品牌/备注、小计
- **AND** 显示参考价格范围

#### Scenario: 全包/全案模式显示简化信息

- **WHEN** 用户选择全包或全案装修方式
- **THEN** 预算项目显示简化模式
- **AND** 包含：项目名称、工艺说明输入、总价输入
- **AND** 不显示数量和单价字段

### Requirement: Decoration Budget Custom Items

装修预算助手 SHALL 支持用户在各分类下灵活添加项目，无预设项目。

#### Scenario: 分类默认为空

- **WHEN** 首次渲染预算列表
- **THEN** 各分类下无预设项目
- **AND** 用户需手动添加所需项目

#### Scenario: 添加项目时显示建议

- **WHEN** 用户点击"添加项目"按钮
- **THEN** 显示项目名称输入框
- **AND** 输入时显示常用项目建议列表
- **AND** 用户可选择建议或输入自定义名称

#### Scenario: 删除项目

- **WHEN** 用户点击项目的删除按钮
- **THEN** 系统从列表中移除该项目
- **AND** 更新分类小计和总预算

#### Scenario: 项目数据持久化

- **WHEN** 用户添加、修改或删除项目
- **THEN** 系统将数据保存到 localStorage
- **AND** 页面刷新后项目被恢复

### Requirement: Decoration Budget Design Fee Category

装修预算助手 SHALL 在所有装修方式下显示设计分类。

#### Scenario: 设计分类始终显示

- **WHEN** 渲染预算列表
- **THEN** 设计分类始终显示在最顶部
- **AND** 适用于所有装修方式（清包、半包、全包、全案）

#### Scenario: 设计费可为零

- **WHEN** 用户不需要付费设计
- **THEN** 用户可输入 0 元
- **AND** 设计分类仍然显示

### Requirement: Decoration Budget Custom Furniture Category

装修预算助手 SHALL 包含全屋定制分类。

#### Scenario: 显示全屋定制分类

- **WHEN** 渲染预算列表
- **THEN** 显示"全屋定制"分类
- **AND** 包含：衣柜、橱柜、书柜/书桌、榻榻米、鞋柜等项目

#### Scenario: 全屋定制计入汇总

- **WHEN** 计算预算总计
- **THEN** 全屋定制费用计入总预算
- **AND** 汇总区域单独显示"全屋定制"金额

### Requirement: Decoration Budget Quick Quote

装修预算助手 SHALL 支持快捷报价功能，用户可输入总单价快速计算预算。

#### Scenario: 输入快捷报价

- **WHEN** 用户在设置面板输入单价（元/平米）
- **THEN** 系统自动计算：面积 × 单价 = 总价
- **AND** 显示该报价覆盖的工程阶段

#### Scenario: 清包模式快捷报价覆盖范围

- **WHEN** 用户选择清包并输入快捷报价
- **THEN** 报价覆盖基装人工（拆改、水电、瓦工、木工、油漆）

#### Scenario: 半包模式快捷报价覆盖范围

- **WHEN** 用户选择半包并输入快捷报价
- **THEN** 报价覆盖基装部分（拆改、水电、瓦工、木工、油漆）

#### Scenario: 全包模式快捷报价覆盖范围

- **WHEN** 用户选择全包并输入快捷报价
- **THEN** 报价覆盖硬装部分（拆改、水电、瓦工、木工、油漆、主材）

#### Scenario: 全案模式快捷报价覆盖范围

- **WHEN** 用户选择全案并输入快捷报价
- **THEN** 报价覆盖全案部分（设计、拆改、水电、瓦工、木工、油漆、主材、全屋定制）

#### Scenario: 快捷报价与明细共存

- **WHEN** 用户同时使用快捷报价和明细输入
- **THEN** 汇总区域分别显示"快捷报价"和"明细合计"
- **AND** 总预算为两者之和

### Requirement: Decoration Budget Item Suggestions

装修预算助手 SHALL 为各分类提供常用项目建议列表。

#### Scenario: 设计分类建议

- **WHEN** 用户在设计分类添加项目
- **THEN** 建议列表包含：设计费、效果图、施工图、量房费

#### Scenario: 拆改工程建议

- **WHEN** 用户在拆改工程分类添加项目
- **THEN** 建议列表包含：拆墙、砌墙、铲墙皮、垃圾清运、拆除门窗等

#### Scenario: 水电工程建议

- **WHEN** 用户在水电工程分类添加项目
- **THEN** 建议列表包含：电路改造、水路改造、开关插座安装、弱电布线等

#### Scenario: 建议列表支持扩展

- **WHEN** 用户输入建议列表中没有的项目名称
- **THEN** 系统允许添加该自定义项目
- **AND** 可选择将其保存为新的建议项（未来使用）


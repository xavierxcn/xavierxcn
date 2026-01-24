# tools Specification Delta

## MODIFIED Requirements

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

## ADDED Requirements

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

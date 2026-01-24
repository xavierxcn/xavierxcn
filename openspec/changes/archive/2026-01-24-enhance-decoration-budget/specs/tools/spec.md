# tools Specification Delta

## ADDED Requirements

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

装修预算助手 SHALL 支持用户在各分类下添加自定义项目（增项）。

#### Scenario: 添加自定义项目

- **WHEN** 用户点击分类下的"添加项目"按钮
- **THEN** 系统显示新项目输入行
- **AND** 用户可输入项目名称和相关费用信息
- **AND** 自定义项目根据当前显示模式显示对应字段

#### Scenario: 删除自定义项目

- **WHEN** 用户点击自定义项目的删除按钮
- **THEN** 系统从列表中移除该项目
- **AND** 更新分类小计和总预算

#### Scenario: 自定义项目持久化

- **WHEN** 用户添加或修改自定义项目
- **THEN** 系统将数据保存到 localStorage
- **AND** 页面刷新后自定义项目被恢复

### Requirement: Decoration Budget Design Fee Category

装修预算助手 SHALL 在全案模式下显示设计费分类。

#### Scenario: 全案模式显示设计费

- **WHEN** 用户选择全案装修方式
- **THEN** 预算列表显示"设计费"分类
- **AND** 包含：设计费、效果图、施工图等项目

#### Scenario: 非全案模式隐藏设计费

- **WHEN** 用户选择清包、半包或全包装修方式
- **THEN** 预算列表不显示"设计费"分类

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

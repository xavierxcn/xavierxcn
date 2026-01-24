# Spec Delta: tools/decoration-budget Budget Target

## ADDED Requirements

### Requirement: Budget Target Input
**Priority**: High

装修预算助手 SHALL 支持用户设置目标预算金额。目标预算 MUST 持久化保存到 localStorage。

#### Scenario: User sets budget target
- Given 用户在设置面板
- When 用户在"目标预算"输入框中输入金额（如 400000）
- Then 系统保存该目标预算
- And 页面刷新后目标预算被恢复

#### Scenario: Budget target is optional
- Given 用户未设置目标预算
- When 查看汇总区域
- Then 不显示剩余预算相关信息

### Requirement: Remaining Budget Display
**Priority**: High

装修预算助手 SHALL 在汇总卡片中显示剩余预算。剩余预算 MUST 根据目标预算和已支出总计实时计算。

#### Scenario: Display remaining budget
- Given 用户设置了目标预算为 400000
- And 已支出总计为 150000
- When 查看汇总卡片
- Then 显示剩余预算为 250000

#### Scenario: Overspending warning
- Given 用户设置了目标预算为 400000
- And 已支出总计为 450000
- When 查看汇总卡片
- Then 剩余预算显示为 -50000
- And 使用红色警示样式

### Requirement: Budget Progress Bar
**Priority**: Medium

装修预算助手 SHALL 显示预算使用进度条。进度条颜色 MUST 随使用比例变化。

#### Scenario: Progress bar color changes
- Given 用户设置了目标预算
- When 预算使用比例 ≤60%
- Then 进度条显示绿色

#### Scenario: Progress bar yellow warning
- Given 用户设置了目标预算
- When 预算使用比例 >60% 且 ≤90%
- Then 进度条显示黄色

#### Scenario: Progress bar red warning
- Given 用户设置了目标预算
- When 预算使用比例 >90%
- Then 进度条显示红色

# Spec Delta: tools/decoration-budget UX Improvements

## ADDED Requirements

### Requirement: Add Item Interaction
**Priority**: High

用户点击"添加项目"按钮后，系统 SHALL 弹出模态框供用户选择或输入项目，而非直接添加空行。模态框 MUST 显示该分类的所有建议项目，并支持多选批量添加。

#### Scenario: User adds items via modal
- Given 用户在某个分类下点击"添加项目"按钮
- When 模态框弹出
- Then 显示该分类的所有建议项目作为可选复选框
- And 提供自定义项目名称输入框
- And 用户可以多选后一次性添加

#### Scenario: User adds custom item
- Given 用户在模态框中输入自定义项目名称
- When 用户点击确认
- Then 自定义项目被添加到分类中
- And 输入框清空以便继续添加

### Requirement: Delete Confirmation
**Priority**: Medium

删除项目时系统 SHALL 提供确认机制，防止误操作。删除动画 MUST 平滑过渡。

#### Scenario: User deletes an item
- Given 用户点击删除按钮
- When 删除按钮变为确认状态
- Then 用户需要再次点击确认才能删除
- And 删除时有平滑的淡出动画

### Requirement: Fullscreen Mode
**Priority**: Medium

系统 SHALL 提供全屏模式以最大化工作区空间。全屏模式 MUST 支持 ESC 键退出。

#### Scenario: User enters fullscreen
- Given 用户点击全屏按钮
- When 进入全屏模式
- Then 工具占满整个视口
- And 页面其他元素被隐藏
- And 显示退出全屏按钮

#### Scenario: User exits fullscreen
- Given 用户在全屏模式下
- When 用户点击退出按钮或按 ESC 键
- Then 恢复正常视图

### Requirement: Mobile Interaction
**Priority**: Medium

移动端的删除按钮 MUST 始终可见，不依赖 hover 状态。

#### Scenario: Mobile delete button visibility
- Given 用户在移动设备上使用工具
- When 查看项目列表
- Then 删除按钮始终可见（使用紧凑图标）

### Requirement: Collapse State Persistence
**Priority**: Low

分类的折叠/展开状态 SHALL 持久化保存到 localStorage。

#### Scenario: Collapse state is remembered
- Given 用户折叠了某些分类
- When 用户刷新页面
- Then 折叠状态被恢复

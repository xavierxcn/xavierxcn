# Tasks: add-budget-target

## Implementation Checklist

### Phase 1: 目标预算输入
- [x] 在设置面板添加"目标预算"输入框（在"基础信息"区块）
- [x] 添加 budgetTarget 状态到 DecorationBudget 类
- [x] 实现 budgetTarget 的 localStorage 持久化
- [x] 绑定输入事件更新状态

### Phase 2: 剩余预算显示
- [x] 在汇总卡片中添加"剩余预算"行
- [x] 实现剩余预算计算（目标 - 已支出）
- [x] 超支时显示红色警示样式
- [x] 未设置目标时隐藏剩余预算行

### Phase 3: 进度条可视化
- [x] 添加进度条 HTML 结构
- [x] 添加进度条 CSS 样式
- [x] 实现进度百分比计算
- [x] 根据使用比例动态改变颜色（≤60% 绿色，≤90% 黄色，>90% 红色）

### Bonus
- [x] Excel 导出包含目标预算和剩余预算信息

### Validation
- [ ] 测试目标预算输入和持久化
- [ ] 测试剩余预算计算和显示
- [ ] 测试超支警示样式
- [ ] 测试进度条颜色变化

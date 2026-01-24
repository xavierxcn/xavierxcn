# Tasks: improve-decoration-budget-ux

## Implementation Checklist

### Phase 1: 添加项目弹窗
- [x] 创建模态框 HTML 结构
- [x] 实现模态框 CSS 样式（含动画）
- [x] 实现 `showAddItemModal(categoryKey)` 方法
- [x] 渲染建议项目列表，支持多选复选框
- [x] 添加自定义项目输入框
- [x] 实现批量添加选中项目的逻辑
- [x] 添加键盘支持（Enter 确认，ESC 取消）

### Phase 2: 删除确认与动画
- [x] 实现删除确认逻辑（内联确认按钮）
- [x] 添加删除过渡动画
- [x] 移动端删除按钮始终可见

### Phase 3: 全屏模式
- [x] 添加全屏切换按钮到工具栏
- [x] 实现全屏 CSS 样式
- [x] 实现 `toggleFullscreen()` 方法
- [x] 添加 ESC 键退出全屏
- [x] 持久化全屏偏好设置

### Phase 4: 状态持久化
- [x] 保存分类折叠状态到 localStorage
- [x] 页面加载时恢复折叠状态

### Phase 5: 视觉美化
- [x] 统一按钮样式和配色
- [x] 优化表格和卡片间距
- [x] 添加微交互动画（hover、focus 状态）
- [x] 确保深色模式一致性
- [x] 移动端响应式优化

### Validation
- [x] 测试添加项目弹窗流程
- [x] 测试删除确认和动画
- [x] 测试全屏模式切换
- [x] 测试折叠状态持久化
- [x] 测试移动端交互
- [x] 验证 localStorage 兼容性

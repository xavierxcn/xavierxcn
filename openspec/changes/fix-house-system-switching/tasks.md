# Tasks: fix-house-system-switching

## Implementation Checklist

### Phase 1: 验证 ccall 可用性
- [ ] 检查 Module.ccall 是否可用
- [ ] 检查 Module.cwrap 是否可用
- [ ] 检查 Module._malloc 和 Module._free 是否可用

### Phase 2: 实现修复
- [ ] 根据可用性选择实现方案
- [ ] 实现字符串参数正确传递
- [ ] 测试基本功能是否正常

### Phase 3: 测试验证
- [ ] 测试 Placidus (P) 宫位制
- [ ] 测试 Whole Sign (W) 整宫制
- [ ] 测试 Equal (E) 等宫制
- [ ] 测试 Koch (K) 宫位制
- [ ] 验证星盘图形更新
- [ ] 验证宫头位置表格更新

### Phase 4: 清理
- [ ] 移除调试日志
- [ ] 代码整理

## Dependencies

```
Phase 1 → Phase 2 → Phase 3 → Phase 4
```

## Acceptance Criteria

1. 切换到整宫制(W)时，每个宫位角度相差精确 30°
2. 切换到等宫制(E)时，每个宫位角度相差精确 30°
3. 切换宫位制后，星盘图形的宫位线位置随之变化
4. 切换宫位制后，宫头位置表格数据随之更新
5. 设置在刷新后保持

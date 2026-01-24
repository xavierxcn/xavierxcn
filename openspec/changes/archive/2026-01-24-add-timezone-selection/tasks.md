# Tasks: add-timezone-selection

## Implementation Checklist

### Phase 1: 时区选择器
- [x] 添加时区数据常量（常用时区列表）
- [x] 添加时区选择下拉框 UI
- [x] 设置默认时区为 UTC+8

### Phase 2: 时间转换逻辑
- [x] 实现本地时间到 UTC 的转换函数
- [x] 处理跨日期的情况（如东八区 01:00 = UTC 前一天 17:00）
- [x] 在生成星盘时应用时区转换

### Phase 3: 结果显示优化
- [x] 在结果区域显示输入的本地时间
- [x] 显示计算使用的 UTC 时间
- [x] 显示所选时区信息

### Phase 4: 本地存储
- [x] 将时区选择保存到 localStorage
- [x] 页面加载时恢复上次选择的时区

### Validation
- [x] 测试东八区时间转换（如北京时间 08:00 = UTC 00:00）
- [x] 测试跨日期转换（如 UTC+8 的 01:00 = UTC 前一天 17:00）
- [x] 测试负时区（如 UTC-5 美东时间）
- [x] 验证星盘计算结果与专业软件一致

## Dependencies

```
Phase 1 → Phase 2 → Phase 3
             ↓
          Phase 4
```

## Acceptance Criteria

1. 用户可以选择时区，默认为 UTC+8
2. 输入的时间被正确识别为本地时间
3. 系统正确转换为 UTC 进行计算
4. 跨日期转换正确处理日期变化
5. 结果显示本地时间和 UTC 时间

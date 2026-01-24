# Tasks: add-natal-chart-tool

## Implementation Checklist

### Phase 1: 准备工作
- [x] 下载并集成 astro-sweph WASM 库
- [x] 在 _meta.yaml 中注册新工具
- [x] 创建 template.html 基础结构

### Phase 2: 用户输入界面
- [x] 实现出生日期选择器
- [x] 实现出生时间选择器
- [x] 实现城市搜索输入框（带自动补全）
- [x] 调用 Nominatim API 获取城市经纬度
- [x] 处理时区转换

### Phase 3: 星历计算
- [x] 初始化 astro-sweph WASM 模块
- [x] 实现行星位置计算函数
- [x] 实现宫位计算函数
- [x] 将计算结果转换为 AstrologyChart2 格式

### Phase 4: 星盘渲染
- [x] 使用 AstrologyChart2 渲染星盘
- [x] 显示相位线
- [x] 添加星盘样式（适配深色模式）

### Phase 5: 信息展示
- [x] 显示行星位置表格（行星、星座、度数、宫位）
- [x] 显示宫头位置表格
- [x] 显示相位列表

### Phase 6: 用户体验优化
- [x] 添加加载状态指示
- [x] 添加输入验证和错误提示
- [x] 移动端适配
- [x] 本地存储上次输入

### Validation
- [x] 测试不同日期的计算精度
- [x] 测试城市搜索功能
- [x] 测试深色模式显示
- [x] 测试移动端布局

## Dependencies

```
Phase 1 → Phase 2 → Phase 3 → Phase 4 → Phase 5 → Phase 6
                ↘           ↗
                  (并行)
```

## Acceptance Criteria

1. 用户可以输入出生日期、时间、地点
2. 城市搜索支持中英文，自动补全
3. 星盘计算精度与专业软件一致
4. 星盘图美观、清晰、支持深色模式
5. 显示完整的行星、宫位、相位信息

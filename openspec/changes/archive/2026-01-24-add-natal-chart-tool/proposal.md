# Proposal: add-natal-chart-tool

## Why

占星板块目前只有静态的教学内容，缺少交互式工具。用户无法根据自己的出生信息绘制本命星盘，这是占星学习和应用的核心需求。

## What Changes

添加一个"本命星盘"工具，允许用户输入出生日期、时间和地点，生成专业级的本命星盘图。

### 功能特性

1. **用户输入表单**
   - 出生日期选择器（年月日）
   - 出生时间选择器（时分）
   - 出生地点搜索（城市自动补全）

2. **星历计算**
   - 使用 `astro-sweph` WebAssembly 库（基于 Swiss Ephemeris）
   - 支持所有十大行星 + 凯龙星、莉莉丝、北交点
   - 支持多种宫位制（默认 Placidus）
   - 弧秒级精度，1800-2400年范围

3. **星盘展示**
   - 使用已集成的 AstrologyChart2 库渲染
   - 显示行星位置、宫位、相位线
   - 显示行星落座、落宫信息表格

4. **城市搜索**
   - 使用 OpenStreetMap Nominatim API（免费，无需 API Key）
   - 自动获取经纬度和时区

## Technical Approach

### 依赖库

| 库 | 用途 | 大小 |
|---|---|---|
| astro-sweph | Swiss Ephemeris WASM | ~1.9MB |
| AstrologyChart2 | 星盘渲染 | 已集成 |
| Nominatim API | 城市地理编码 | 外部 API |

### 架构

```
用户输入 → 城市搜索(Nominatim) → 获取经纬度/时区
         ↓
      astro-sweph WASM 计算行星位置
         ↓
      AstrologyChart2 渲染星盘
         ↓
      显示详细信息表格
```

### 文件结构

```
content/tools/natal-chart/
├── template.html     # 完整的工具页面
themes/default/static/js/
├── astro-sweph.js    # Swiss Ephemeris WASM 库
```

## Scope

- 添加新工具页面，不修改现有功能
- 复用已有的 AstrologyChart2 和 Astronomicon 字体
- 纯前端实现，无需后端支持

## Out of Scope

- 星盘解读/报告生成
- 合盘、推运等高级功能
- 用户数据存储

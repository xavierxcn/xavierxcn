# Proposal: 为占星板块添加星盘图可视化

## 背景

当前占星板块的文章都是纯文字内容，缺少直观的星盘图展示。对于占星内容来说，星盘是最核心的可视化工具，能够帮助读者直观理解星座、行星、宫位、相位等概念。

## 目标

在占星板块的文章页面中集成 JavaScript 星盘绘制库，通过配置参数动态渲染星盘图，使内容更加直观易懂。

## 方案概述

### 选用库

采用 **[@astrodraw/astrochart](https://github.com/AstroDraw/AstroChart)** 库：

- 纯 TypeScript/JavaScript 实现，零依赖
- 生成 SVG 格式图表，清晰可缩放
- 支持展示行星、宫位、星座、相位等
- 开源免费，MIT 许可证
- 通过 CDN 引入，无需构建工具

### 实现方式

1. **模板层面**：在 `astrology-item.html` 模板中引入 AstroChart 库
2. **数据驱动**：通过 front matter 中的配置参数控制星盘显示内容
3. **按需渲染**：只在配置了星盘参数的页面渲染星盘图

### 星盘类型

根据不同的占星内容类型，展示不同的星盘配置：

| 内容类型 | 星盘展示 |
|---------|---------|
| 星座 (signs) | 高亮对应星座区域 |
| 行星 (planets) | 高亮对应行星符号 |
| 宫位 (houses) | 高亮对应宫位 |
| 相位 (aspects) | 展示相位连线示例 |
| 常识 (basics) | 展示完整星盘或特定元素 |

### Front Matter 扩展

在占星文章的 front matter 中添加 `chart` 配置：

```yaml
---
title: 白羊座
chart:
  type: sign           # sign | planet | house | aspect | full
  highlight: aries     # 高亮的元素
  show_houses: true    # 是否显示宫位
  show_planets: false  # 是否显示行星
---
```

## 范围

### 包含

- 引入 AstroChart 库（通过 CDN）
- 修改 `astrology-item.html` 模板添加星盘容器和渲染脚本
- 扩展 front matter 支持 `chart` 配置
- 为现有占星文章添加合适的星盘配置

### 不包含

- 交互式星盘（拖拽、缩放等）
- 真实出生星盘计算（需要精确时间地点）
- 自定义星盘样式主题

## 风险与考量

1. **渐进增强**：星盘作为可视化增强，不影响文字内容的可读性
2. **性能**：AstroChart 库体积约 50KB，通过 CDN 加载，对首屏影响小
3. **兼容性**：SVG 在现代浏览器中有良好支持

## 参考资源

- [AstroChart 官网](https://astrodraw.github.io/)
- [AstroChart GitHub](https://github.com/AstroDraw/AstroChart)
- [AstroChart npm](https://www.npmjs.com/package/@astrodraw/astrochart)

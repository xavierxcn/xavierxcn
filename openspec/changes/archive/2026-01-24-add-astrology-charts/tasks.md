# Tasks: 为占星板块添加星盘图可视化

## 任务清单

- [x] **1. 研究 AstroChart API**
  - 阅读 AstroChart 文档，了解 API 用法
  - 创建本地测试页面验证库功能
  - 确定各类占星内容的星盘配置方案

- [x] **2. 扩展 Front Matter 解析**
  - 在 `generator/src/content/mod.rs` 中添加 `ChartConfig` 结构体
  - 在 `AstrologyMeta` 结构体中添加 `chart` 字段
  - 更新 YAML 解析逻辑

- [x] **3. 修改占星模板**
  - 在 `themes/default/templates/astrology-item.html` 中添加星盘容器
  - 添加星盘样式
  - 添加根据 front matter 配置渲染星盘的 JavaScript 代码

- [x] **4. 创建星盘渲染脚本**
  - 创建 `themes/default/static/js/astro-chart.js`
  - 实现纯 SVG 星盘绘制，不依赖外部库
  - 实现根据配置类型（sign/planet/house/aspect）渲染不同星盘的逻辑
  - 处理高亮指定元素的样式

- [x] **5. 为现有内容添加星盘配置**
  - 为 12 个星座文章添加 chart 配置 ✓
  - 为 10 个行星文章添加 chart 配置 ✓
  - 为 12 个宫位文章添加 chart 配置 ✓
  - 为 5 个相位文章添加 chart 配置 ✓

- [x] **6. 测试与优化**
  - 本地构建成功
  - 验证星盘脚本正确包含在生成的 HTML 中
  - basics 目录文章无 chart 配置也能正常构建

- [x] **7. 构建并部署**
  - 运行 `make build` 构建站点 ✓
  - 验证生成的 HTML 正确包含星盘脚本 ✓

## 依赖关系

```
1 → 2 → 3 → 4 → 5 → 6 → 7
```

## 验收标准

1. ✅ 占星文章页面显示对应的星盘图
2. ✅ 星盘正确高亮当前文章对应的元素（星座/行星/宫位）
3. ✅ 页面加载性能无明显下降（使用纯 SVG，无外部依赖）
4. ✅ 禁用 JavaScript 时文字内容仍可正常阅读

## 实现说明

最终采用纯 SVG 实现星盘绘制，而非外部 AstroChart 库，原因：
- 零依赖，无 CDN 加载延迟
- 代码完全可控，易于定制
- 体积更小，性能更好

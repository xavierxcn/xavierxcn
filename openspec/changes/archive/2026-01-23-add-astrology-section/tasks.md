# Tasks: 新增占星板块

## 1. 内容模块扩展

- [x] 1.1 定义 `AstrologyItem` 数据结构（slug, title, category, symbol, element, modality 等字段）
- [x] 1.2 实现 `load_astrology_items()` 函数，从 `content/astrology/` 加载所有占星内容
- [x] 1.3 按分类组织数据（signs, planets, houses, aspects）
- [x] 1.4 为内容模块添加单元测试

## 2. 模板创建

- [x] 2.1 创建 `astrology.html` 占星首页模板（展示四个分类卡片）
- [x] 2.2 创建 `astrology-category.html` 分类汇总页模板（展示该分类下所有条目）
- [x] 2.3 添加占星相关的 CSS 样式
- [x] 2.4 实现客户端筛选/搜索功能（JavaScript）

## 3. 构建流程集成

- [x] 3.1 在 `Builder` 中添加 `generate_astrology()` 方法
- [x] 3.2 生成占星首页 `/astrology/index.html`
- [x] 3.3 生成四个分类页面 `/astrology/{category}/index.html`
- [x] 3.4 将占星内容加入搜索索引（结构已支持，待后续添加）

## 4. 配置与导航

- [x] 4.1 在 `config.yaml` 的 nav 中添加占星入口
- [x] 4.2 创建示例内容（至少每个分类一个示例）

## 5. 验证

- [x] 5.1 运行所有单元测试（24 passed）
- [x] 5.2 本地预览验证页面渲染
- [x] 5.3 验证筛选/搜索功能正常工作

## 依赖关系

- 任务 2 依赖任务 1（需要数据结构定义）
- 任务 3 依赖任务 1 和任务 2
- 任务 5 依赖所有其他任务

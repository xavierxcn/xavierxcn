# Tasks

## 1. 扩展搜索索引

### 1.1 修改 SearchIndexItem 结构
- [x] 添加 `item_type` 字段区分文章和占星内容
- [x] 添加 `category` 字段用于占星分类

### 1.2 更新 generate_search_index 函数
- [x] 遍历 `self.astrology.categories` 添加占星条目到索引
- [x] 设置合适的 URL 路径和元数据

### 1.3 更新前端搜索显示
- [x] 修改 search.js 支持显示占星内容
- [x] 添加类型标签（文章/星座/行星等）区分搜索结果

## 2. 重新设计占星首页（使用 frontend-design）

### 2.1 设计紧凑型布局
- [x] 精简 hero 区域，只保留标题
- [x] 添加分类 tab 栏（星座/行星/宫位/相位）
- [x] 设计条目网格视图，直接显示所有条目

### 2.2 实现分类切换交互
- [x] 使用 JavaScript 实现 tab 切换
- [ ] 保持 URL 状态同步（可选，暂不实现）
- [x] 添加平滑过渡动画

### 2.3 更新模板上下文
- [x] 在模板中渲染所有分类的条目（已通过现有context实现）

## 3. 验证

### 3.1 搜索功能验证
- [x] 验证搜索索引包含39个占星条目和2篇文章（共41条）
- [x] 验证占星条目包含 item_type: "astrology" 和 category 字段

### 3.2 首页交互验证
- [x] 验证 HTML 正确生成（tabs、cards、grid）
- [x] 验证条目卡片包含正确的链接
- [x] 验证移动端响应式 CSS 就绪

# Tasks: Add Tools Section

## Phase 1: Infrastructure Refactoring

### 1.1 Refactor Tool content type for pure HTML

- [x] 修改 `generator/src/content/mod.rs` 中的 `ToolMeta` 结构体
  - 添加 `slug` 字段（从 `_meta.yaml` 读取）
  - 移除 `raw_html` 字段（不再需要）
- [x] 新增 `ToolsMetaFile` 结构体解析 `_meta.yaml`
- [x] 重写 `load_tools()` 函数从 `_meta.yaml` 加载元数据

**验证**: `cargo build` 通过，`cargo test` 通过 ✓

### 1.2 Update build process for tool copying

- [x] 修改 `generator/src/build.rs` 中的 `generate_tools()` 方法
  - 复制工具目录而非渲染 Markdown
  - 生成工具列表页（复用现有逻辑）
- [x] 使用现有 `copy_dir_recursive()` 辅助函数

**验证**: `make build` 正确复制工具目录到 `docs/tools/` ✓

### 1.3 Update template context

- [x] 修改 `generator/src/render/template.rs` 中的 `ToolsListContext`
  - 移除 `ToolContext`（不再需要）
  - 确保与新的 `Tool` 结构体兼容

**验证**: 工具列表页正确渲染 ✓

---

## Phase 2: Create Pure HTML Tool Page

### 2.1 Create tool metadata file

- [x] 创建 `content/tools/_meta.yaml`
- [x] 添加 md2wechat 工具元数据

**验证**: `make build` 能读取元数据 ✓

### 2.2 Create md2wechat HTML page

- [x] 创建 `content/tools/md2wechat/index.html`
- [x] 包含完整的 HTML 结构（doctype, head, body）
- [x] 包含导航栏和页脚（与主站风格一致）
- [x] 页面布局宽度最大化，充分利用屏幕空间

**验证**: 页面可以独立在浏览器中打开 ✓

### 2.3 Implement tool CSS (embedded)

- [x] 在 `<style>` 标签中嵌入所有样式
- [x] 导航栏、页脚样式与主站一致
- [x] 工具区域样式（编辑器、预览区）
- [x] 响应式布局

**验证**: 页面在不同屏幕尺寸下显示正常 ✓

### 2.4 Implement tool JavaScript (embedded)

- [x] 引入 CDN 库（marked.js, highlight.js, mermaid.js）
- [x] 实现 Markdown 解析和实时预览
- [x] 实现代码高亮（内联样式）
- [x] 实现 Mermaid 图表渲染
- [x] 实现复制到剪贴板功能
- [x] 实现主题切换功能

**验证**: 所有功能正常工作 ✓

### 2.5 UI polish

- [x] 工具页面 UI 简洁专业
- [x] 确保高质量的视觉效果
- [x] 优化移动端体验

**验证**: 页面视觉效果达到专业水准 ✓

---

## Phase 3: Cleanup

### 3.1 Remove old files

- [x] 删除 `content/tools/md2wechat.md`（旧的 Markdown 文件）
- [x] 删除 `themes/default/static/js/tools/md2wechat.js`（逻辑已内嵌）
- [x] 删除 `themes/default/templates/tool.html`（不再使用单独的工具模板）

**验证**: `make build` 仍然成功 ✓

### 3.2 Code cleanup

- [x] 移除 `generator/src/content/mod.rs` 中未使用的代码
- [x] 移除 `raw_html` 相关代码
- [x] 移除 `ToolContext` 从 template.rs

**验证**: `cargo build` 通过，`cargo test` 通过 ✓

---

## Phase 4: Testing

### 4.1 End-to-end testing

- [ ] 测试完整的 Markdown 转换流程
- [ ] 测试各种 Markdown 语法（标题、列表、表格、代码、图片）
- [ ] 测试 Mermaid 图表类型（flowchart, sequence, class, etc.）
- [ ] 测试复制到微信公众号编辑器

**验证**: 复制的 HTML 在微信公众号编辑器中正确显示

### 4.2 Cross-browser testing

- [ ] Chrome 测试
- [ ] Safari 测试
- [ ] Firefox 测试
- [ ] 移动端 Safari/Chrome 测试

**验证**: 所有主流浏览器功能正常

---

## Implementation Notes

### Files Created

- `content/tools/_meta.yaml` - 工具元数据文件
- `content/tools/md2wechat/index.html` - 完整的工具页面（HTML+CSS+JS 内嵌）

### Files Modified

**Generator (Rust)**:
- `generator/src/content/mod.rs` - 重构 ToolMeta, Tool, 新增 ToolsMetaFile, 重写 load_tools()
- `generator/src/build.rs` - 修改 generate_tools() 为复制目录方式
- `generator/src/render/template.rs` - 移除 ToolContext

### Files Removed

- `content/tools/md2wechat.md` - 旧的 Markdown 工具文件
- `themes/default/static/js/tools/md2wechat.js` - 旧的 JS 文件
- `themes/default/templates/tool.html` - 旧的模板文件

### Architecture Change

从 Markdown 嵌入方式改为纯 HTML 方式：

**之前**:
```
content/tools/md2wechat.md → Markdown 解析 → tool.html 模板 → 输出
```

**现在**:
```
content/tools/_meta.yaml → 读取元数据 → tools.html 模板 → 工具列表页
content/tools/md2wechat/ → 直接复制 → 输出
```

优势：
1. 避免 Markdown 解析问题
2. 工具页面可独立开发测试
3. 更灵活的布局控制
4. 性能更好

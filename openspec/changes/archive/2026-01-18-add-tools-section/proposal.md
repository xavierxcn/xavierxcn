# Proposal: Add Tools Section

## Summary

为博客添加工具页面区域，提供实用的在线工具。第一个工具是 **Markdown 转微信公众号 HTML 格式转换器**，支持代码高亮和 Mermaid 图表。

## Problem Statement

当前博客仅支持文章和静态页面，缺少交互式工具页面的能力。用户需要一个地方来托管实用的在线小工具，如：
- Markdown 转微信公众号格式
- 计算器
- 单位转换
- 时间戳转换
- 等等

这些工具页面需要：
1. 独立的 URL 路径 (`/tools/{tool-name}/`)
2. 客户端交互能力 (JavaScript)
3. 与博客主题风格统一
4. 在导航中可访问

## Proposed Solution

### 1. 工具页面架构（纯 HTML 方式）

工具页面采用**纯 HTML+CSS+JS** 的方式，不经过 Markdown 处理。每个工具是一个独立的 HTML 文件：

```
content/tools/
├── _meta.yaml           # 工具元数据（用于生成列表页）
└── md2wechat/
    └── index.html       # 完整的工具页面（HTML+CSS+JS 内嵌）
```

工具元数据文件 `_meta.yaml` 格式：

```yaml
tools:
  - slug: md2wechat
    title: Markdown 转微信公众号
    description: 将 Markdown 转换为微信公众号兼容的 HTML 格式
    icon: 📝
  - slug: calculator
    title: 计算器
    description: 简单的在线计算器
    icon: 🧮
```

### 2. 构建流程

1. **加载工具元数据**：从 `content/tools/_meta.yaml` 读取工具列表
2. **复制工具页面**：将 `content/tools/{slug}/` 目录整体复制到 `output/tools/{slug}/`
3. **生成列表页**：使用 `tools.html` 模板生成 `/tools/index.html`
4. **导航栏**：添加"工具"入口

### 3. Markdown 转微信公众号工具

**功能需求：**
- 左侧输入 Markdown 文本
- 右侧实时预览微信公众号样式
- 支持代码高亮（内联样式，微信兼容）
- 支持 Mermaid 图表（转换为 SVG 内嵌）
- 一键复制 HTML 到剪贴板
- 可选主题风格

**技术方案：**
- 纯 HTML 文件，CSS 和 JS 内嵌在页面中
- 使用 CDN 引入第三方库（marked.js, highlight.js, mermaid.js）
- 所有输出样式内联（微信公众号不支持外部 CSS）

### 4. 工具页面 HTML 结构

每个工具页面是一个完整的 HTML 文件，包含：

```html
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>工具名称 - Xavier's Blog</title>
    <style>
        /* 页面样式内嵌 */
    </style>
</head>
<body>
    <!-- 页面导航（可选：通过 JS 动态加载或静态写入） -->
    <header>...</header>

    <!-- 工具内容 -->
    <main>...</main>

    <!-- 页脚 -->
    <footer>...</footer>

    <script>
        /* 工具逻辑内嵌 */
    </script>
    <!-- 或引用 CDN -->
    <script src="https://cdn.jsdelivr.net/..."></script>
</body>
</html>
```

## Design Considerations

### 为什么选择纯 HTML 而非 Markdown 嵌入

1. **避免解析问题**：Markdown 解析器对 HTML 内容有特殊处理规则（如缩进导致代码块），纯 HTML 更可控
2. **更灵活**：工具页面通常需要复杂的 HTML 结构和大量 JS，纯 HTML 更自然
3. **性能**：不需要 Markdown 到 HTML 的转换步骤
4. **可移植性**：工具页面可以独立测试，不依赖生成器

### UI 设计要求

使用 `frontend-design` 技能确保：
- 工具页面与博客主题风格一致
- 响应式布局，支持移动端
- 良好的交互体验（实时预览、复制反馈）
- 高质量的视觉效果

### 微信公众号兼容性

微信公众号编辑器的限制：
- 不支持外部 CSS，所有样式必须内联
- 不支持 `<script>`，交互在复制前完成
- 图片需要使用 data URI 或微信图床
- 部分 HTML 标签被过滤

## Scope

**In Scope:**
- 工具页面基础架构（元数据加载、复制、列表页生成）
- 工具列表页面
- Markdown 转微信公众号工具（代码高亮 + Mermaid）
- 导航栏更新

**Out of Scope (Future):**
- 其他工具（计算器、单位转换等）
- 工具页面的 SEO 优化
- 工具使用统计

## Success Criteria

1. 可以通过 `/tools/` 访问工具列表页
2. 可以通过 `/tools/md2wechat/` 访问 Markdown 转换工具
3. 转换后的 HTML 可以直接粘贴到微信公众号编辑器
4. 代码块正确高亮显示
5. Mermaid 图表正确渲染为 SVG

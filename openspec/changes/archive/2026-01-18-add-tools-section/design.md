# Design: Tools Section Architecture

## Overview

本文档描述工具页面的技术架构设计。

## Architecture Decision

### 选项 1: 工具作为 Markdown 页面 (废弃)

将工具视为一种特殊的 Page 类型，在 Markdown 中嵌入 HTML/CSS/JS。

**缺点：**
- Markdown 解析器对 HTML 有特殊处理（如缩进会变成代码块）
- 需要添加 `raw_html` 选项绕过解析
- 复杂 HTML 在 Markdown 中不自然

### 选项 2: 纯 HTML 工具页面 (采用)

工具页面是独立的 HTML 文件，不经过 Markdown 处理。

```
content/tools/
├── _meta.yaml           # 工具元数据
└── md2wechat/
    └── index.html       # 完整 HTML 页面
```

**优点：**
- 完全控制 HTML 结构，无解析问题
- 工具页面可独立开发和测试
- 性能更好（无需转换）
- 更自然的开发体验

**缺点：**
- 需要手动保持导航/页脚与主站一致
- 元数据需要单独维护

### 决策: 选项 2 (纯 HTML)

采用选项 2，理由：
1. 工具页面主要是交互式应用，HTML/CSS/JS 是原生格式
2. 避免 Markdown 解析带来的问题
3. 可以独立开发测试，更灵活

## Data Flow

```
content/tools/_meta.yaml → 解析元数据 → Tool 对象列表
content/tools/{slug}/    → 整体复制   → output/tools/{slug}/
Tool 对象列表            → 模板渲染   → output/tools/index.html
```

## File Structure

### 源文件结构

```
content/tools/
├── _meta.yaml              # 工具元数据列表
└── md2wechat/
    └── index.html          # 完整的工具页面
```

### 元数据格式 (_meta.yaml)

```yaml
tools:
  - slug: md2wechat
    title: Markdown 转微信公众号
    description: 将 Markdown 转换为微信公众号兼容的 HTML 格式
    icon: 📝
```

### 工具页面结构 (index.html)

每个工具页面是完整的 HTML 文件：

```html
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Markdown 转微信公众号 - Xavier's Blog</title>

    <!-- 样式内嵌 -->
    <style>
        /* 导航、页脚样式 */
        /* 工具特有样式 */
    </style>
</head>
<body>
    <!-- 导航栏（与主站保持一致） -->
    <header class="site-header">
        <nav class="nav">
            <a href="/xavierxcn/" class="site-title">Xavier's Blog</a>
            <ul class="nav-links">
                <li><a href="/xavierxcn/">首页</a></li>
                <li><a href="/xavierxcn/tools/">工具</a></li>
                <li><a href="/xavierxcn/archive/">归档</a></li>
                <li><a href="/xavierxcn/about/">关于</a></li>
            </ul>
        </nav>
    </header>

    <!-- 工具内容 -->
    <main class="main-content">
        <!-- 工具 UI -->
    </main>

    <!-- 页脚 -->
    <footer class="site-footer">
        <p>&copy; 2026 Xavier Fan. 保留所有权利。</p>
    </footer>

    <!-- 第三方库（CDN） -->
    <script src="https://cdn.jsdelivr.net/npm/marked/marked.min.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/highlight.js/lib/core.min.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js"></script>

    <!-- 工具逻辑内嵌 -->
    <script>
        // 工具功能实现
    </script>
</body>
</html>
```

## Generator Changes

### 新增数据结构

```rust
/// Tool metadata from _meta.yaml
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToolMeta {
    pub slug: String,
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
}

/// Tools metadata file
#[derive(Debug, Clone, Deserialize)]
pub struct ToolsMetaFile {
    pub tools: Vec<ToolMeta>,
}
```

### 加载流程

```rust
pub fn load_tools_meta<P: AsRef<Path>>(content_dir: P) -> Result<Vec<ToolMeta>> {
    let meta_path = content_dir.as_ref().join("tools/_meta.yaml");
    if !meta_path.exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(&meta_path)?;
    let meta_file: ToolsMetaFile = serde_yaml::from_str(&content)?;
    Ok(meta_file.tools)
}
```

### 构建流程

```rust
fn generate_tools(&self, output_dir: &Path) -> Result<()> {
    let tools_dir = self.config.build.content_dir.join("tools");
    let output_tools = output_dir.join("tools");

    // 1. 复制每个工具目录（排除 _meta.yaml）
    for tool in &self.tools {
        let src = tools_dir.join(&tool.slug);
        let dst = output_tools.join(&tool.slug);
        copy_dir_all(&src, &dst)?;
    }

    // 2. 生成工具列表页
    let context = ToolsListContext { tools: &self.tools, ... };
    let html = self.tera.render("tools.html", &context)?;
    write_file(output_tools.join("index.html"), &html)?;

    Ok(())
}
```

## Template Structure

### tools.html (工具列表页)

```html
{% extends "base.html" %}

{% block content %}
<section class="tools-section">
    <header class="tools-header">
        <h1>工具</h1>
        <p class="tools-intro">一些实用的在线小工具</p>
    </header>

    <div class="tools-grid">
        {% for tool in tools %}
        <a href="{{ config.base_path }}/tools/{{ tool.slug }}/" class="tool-card">
            {% if tool.icon %}
            <span class="tool-card-icon">{{ tool.icon }}</span>
            {% endif %}
            <h2 class="tool-card-title">{{ tool.title }}</h2>
            {% if tool.description %}
            <p class="tool-card-desc">{{ tool.description }}</p>
            {% endif %}
        </a>
        {% endfor %}
    </div>
</section>
{% endblock %}
```

## Markdown 转微信公众号工具设计

### 技术栈

| 功能 | 库 | 说明 |
|-----|-----|------|
| Markdown 解析 | marked.js | 轻量、可扩展 |
| 代码高亮 | highlight.js | 支持多语言，易于自定义主题 |
| Mermaid 渲染 | mermaid.js | 官方库，渲染为 SVG |
| 剪贴板操作 | 原生 API | navigator.clipboard |

### UI 布局

```
+------------------------------------------+
|  Header: 导航栏                            |
+------------------------------------------+
|  工具栏: [主题选择] [复制 HTML] [清空]       |
+-------------------+----------------------+
|                   |                      |
|   Markdown 输入    |   微信样式预览        |
|   (textarea)      |   (预览区域)          |
|                   |                      |
+-------------------+----------------------+
|  Footer: 页脚                             |
+------------------------------------------+
```

### 微信样式内联策略

由于微信公众号不支持外部 CSS，所有样式需要内联：

```javascript
// 将 CSS 类转换为内联样式
function inlineStyles(html) {
    const styleMap = {
        'h1': 'font-size: 24px; font-weight: bold; margin: 20px 0 10px;',
        'h2': 'font-size: 20px; font-weight: bold; margin: 18px 0 8px;',
        'p': 'margin: 10px 0; line-height: 1.8;',
        'code': 'background: #f6f8fa; padding: 2px 4px; border-radius: 3px;',
        'pre': 'background: #282c34; padding: 16px; border-radius: 5px; overflow-x: auto;',
        // ...
    };
    // 遍历 DOM，为每个元素添加 style 属性
}
```

### 性能考虑

- 使用 debounce 延迟预览更新（300ms）
- Mermaid 图表异步渲染
- 大文本分块处理

## Output Structure

```
docs/
└── tools/
    ├── index.html           # 工具列表（模板生成）
    └── md2wechat/
        └── index.html       # 转换工具（直接复制）
```

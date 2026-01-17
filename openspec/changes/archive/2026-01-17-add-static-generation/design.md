## Context

本项目是一个专为 xavierxcn 博客定制的 Rust 静态网站生成器。不同于 Hugo/Hexo 等通用工具，本生成器专注于简洁和高效，不追求插件化或通用性。

**约束条件**:
- 仅需支持本博客的特定需求
- 性能优先，构建速度要快
- 代码简洁，易于维护

## Goals / Non-Goals

**Goals**:
- 实现完整的 Markdown → HTML 转换流程
- 支持 YAML front matter 解析
- 支持代码语法高亮 (服务端渲染)
- 生成首页、归档、标签/分类页面
- 复制静态资源到输出目录

**Non-Goals**:
- 不支持自定义插件系统
- 不支持主题热切换 (固定使用 `themes/default`)
- 不实现分页功能 (首期)
- 不实现 RSS/Sitemap 生成 (后续迭代)

## Decisions

### 1. 模块划分

采用按职责划分的模块结构:

```
generator/src/
├── main.rs         # CLI 入口，仅包含命令解析
├── config.rs       # 配置读取和验证
├── content/
│   ├── mod.rs      # 内容模块入口
│   ├── frontmatter.rs  # YAML front matter 解析
│   └── markdown.rs     # Markdown 解析
├── render/
│   ├── mod.rs      # 渲染模块入口
│   ├── syntax.rs   # 语法高亮
│   └── template.rs # Tera 模板渲染
└── build.rs        # 构建协调
```

**理由**: 职责清晰，每个模块独立可测试，符合项目简洁的目标。

### 2. 数据结构设计

```rust
// 文章数据结构
struct Post {
    slug: String,           // URL slug (从文件名生成)
    meta: PostMeta,         // front matter 数据
    content: String,        // 原始 Markdown 内容
    html_content: String,   // 渲染后的 HTML
}

struct PostMeta {
    title: String,
    date: NaiveDate,
    updated: Option<NaiveDate>,
    tags: Vec<String>,
    categories: Vec<String>,
    summary: Option<String>,
    draft: bool,
}

// 独立页面数据结构 (简化版 Post)
struct Page {
    slug: String,
    title: String,
    content: String,
    html_content: String,
}
```

**理由**: 结构简单直接，Post 和 Page 分开处理，符合实际使用场景。

### 3. 构建流程

```
1. 读取 config.yaml
2. 扫描 content/posts/*.md，解析为 Post 列表
3. 扫描 content/pages/*.md，解析为 Page 列表
4. 对每个 Post/Page 进行 Markdown → HTML 渲染
5. 加载 Tera 模板
6. 生成页面:
   a. 每篇文章 → output/posts/{slug}/index.html
   b. 每个独立页面 → output/{slug}/index.html
   c. 首页 → output/index.html (显示最新文章列表)
   d. 归档页 → output/archive/index.html
   e. 每个标签 → output/tags/{tag}/index.html
   f. 每个分类 → output/categories/{category}/index.html
7. 复制静态资源 themes/default/static/ → output/static/
```

### 4. URL 结构

采用 "pretty URLs" 模式:
- 文章: `/posts/{slug}/` (使用文件名作为 slug)
- 页面: `/{slug}/`
- 归档: `/archive/`
- 标签: `/tags/{tag}/`
- 分类: `/categories/{category}/`

**理由**: 简洁、SEO 友好、与 config.yaml 中 nav 配置一致。

### 5. 错误处理策略

- 配置文件缺失/格式错误: 终止构建并提示
- 单篇文章解析失败: 跳过该文章，记录警告，继续构建
- 模板渲染失败: 终止构建并提示

**理由**: 容错性与可靠性的平衡，避免单篇文章问题影响整站构建。

## Risks / Trade-offs

| Risk | Impact | Mitigation |
|------|--------|------------|
| 大量文章时构建慢 | 低 (个人博客文章量有限) | 后续可添加增量构建 |
| 模板语法错误难以定位 | 中 | 使用 Tera 的错误消息，构建时输出详细信息 |
| 内存占用 (一次性加载所有文章) | 低 | 个人博客规模不会有问题 |

## Migration Plan

不适用 - 这是全新功能，不涉及迁移。

## Open Questions

1. ~~是否需要支持草稿预览 (`draft: true` 的文章)？~~ → 首期不支持，草稿文章不生成
2. 是否需要生成 404 页面？ → 后续迭代考虑

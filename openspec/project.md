# Project Context

## Purpose

xavierxcn 是一个个人博客静态网站项目，包含一个使用 Rust 开发的定制化静态网站生成器。与 Hexo、Hugo 等通用静态网站生成器不同，本项目的生成器专门为此博客定制，不追求通用性，而是追求简洁、高效和可维护性。

### 核心目标

1. **简洁高效**: 生成器代码简洁，构建速度快
2. **Markdown 支持完善**: 支持丰富的 Markdown 扩展语法
3. **插件式主题**: 主题与内容分离，易于更换和定制
4. **现代化体验**: 支持代码高亮、Mermaid 图表、LaTeX 公式等

## Tech Stack

### 生成器 (generator/)

- **语言**: Rust 2021 Edition
- **Markdown 解析**: pulldown-cmark (支持 CommonMark + 扩展)
- **模板引擎**: Tera (Jinja2 风格)
- **语法高亮**: syntect
- **序列化**: serde + serde_yaml + serde_json
- **CLI**: clap
- **开发服务器**: axum + tower-http
- **文件监听**: notify

### 前端

- **Mermaid**: 客户端渲染图表
- **KaTeX/MathJax**: LaTeX 公式渲染
- **Prism.js/highlight.js**: 可选的客户端代码高亮
- **CSS**: 原生 CSS 或 Tailwind CSS

## Project Conventions

### Code Style

#### Rust

- 遵循 Rust 官方风格指南
- 使用 `rustfmt` 格式化代码
- 使用 `clippy` 进行代码检查
- 命名规范:
  - 模块/文件: `snake_case`
  - 类型/Trait: `PascalCase`
  - 函数/变量: `snake_case`
  - 常量: `SCREAMING_SNAKE_CASE`
- 错误处理: 使用 `anyhow` 处理应用错误，`thiserror` 定义库错误类型
- 文档: 公共 API 必须有文档注释

#### 配置文件

- 项目配置使用 YAML 格式
- 文章 front matter 使用 YAML 格式

### Architecture Patterns

```
xavierxcn/
├── generator/              # Rust 静态网站生成器
│   ├── src/
│   │   ├── main.rs        # CLI 入口
│   │   ├── lib.rs         # 库入口
│   │   ├── config.rs      # 配置管理
│   │   ├── content/       # 内容处理模块
│   │   │   ├── mod.rs
│   │   │   ├── post.rs    # 文章解析
│   │   │   ├── page.rs    # 页面解析
│   │   │   └── markdown.rs # Markdown 处理
│   │   ├── theme/         # 主题处理模块
│   │   │   ├── mod.rs
│   │   │   └── template.rs
│   │   ├── render/        # 渲染模块
│   │   │   ├── mod.rs
│   │   │   ├── syntax.rs  # 语法高亮
│   │   │   └── html.rs
│   │   ├── serve/         # 开发服务器
│   │   │   └── mod.rs
│   │   └── utils/         # 工具函数
│   └── Cargo.toml
├── content/               # 博客内容
│   ├── posts/            # 文章目录
│   │   └── *.md
│   └── pages/            # 独立页面
│       └── *.md
├── themes/               # 主题目录
│   └── default/          # 默认主题
│       ├── templates/    # Tera 模板
│       │   ├── base.html
│       │   ├── index.html
│       │   ├── post.html
│       │   ├── page.html
│       │   ├── archive.html
│       │   └── partials/
│       └── static/       # 主题静态资源
│           ├── css/
│           └── js/
├── static/               # 全局静态资源
├── output/               # 生成的静态网站
├── config.yaml           # 站点配置
└── openspec/             # 项目规范
```

#### 核心模块职责

1. **config**: 读取和验证 `config.yaml`，管理站点配置
2. **content**: 解析 Markdown 文件，提取 front matter 和内容
3. **theme**: 加载主题模板，管理模板上下文
4. **render**: 将 Markdown 渲染为 HTML，处理语法高亮
5. **serve**: 提供开发服务器，支持热重载

#### 数据流

```
Markdown 文件 → 解析 front matter → 解析 Markdown → 语法高亮 → 模板渲染 → HTML 输出
```

### Testing Strategy

- **单元测试**: 每个模块的核心函数必须有单元测试
- **集成测试**: 测试完整的构建流程
- **测试目录**: `generator/tests/`
- **测试数据**: 使用 `tempfile` 创建临时测试目录
- 运行测试: `cargo test`

### Git Workflow

- **主分支**: `main`
- **功能分支**: `feature/<feature-name>`
- **修复分支**: `fix/<bug-description>`
- **提交信息**: 使用中文或英文，清晰描述变更内容
- **提交前**: 运行 `cargo fmt` 和 `cargo clippy`

## Domain Context

### Markdown 扩展支持

生成器需要支持以下 Markdown 扩展:

1. **标准 CommonMark**: 基础 Markdown 语法
2. **GFM 表格**: GitHub 风格表格
3. **代码块语法高亮**: 使用 syntect 服务端渲染
4. **Mermaid 图表**: 识别 `mermaid` 代码块，客户端渲染
5. **LaTeX 公式**: 识别 `$...$` 和 `$$...$$`，客户端渲染
6. **任务列表**: `- [ ]` 和 `- [x]`
7. **脚注**: `[^1]` 语法
8. **删除线**: `~~text~~`
9. **自动链接**: URL 自动转换为链接

### 文章 Front Matter 格式

```yaml
---
title: 文章标题
date: 2024-01-01
updated: 2024-01-02  # 可选
tags:
  - Rust
  - Blog
categories:
  - 技术
draft: false  # 可选，默认 false
summary: 文章摘要  # 可选
---
```

### 主题结构

主题必须包含以下模板文件:

- `base.html`: 基础布局模板
- `index.html`: 首页模板
- `post.html`: 文章详情模板
- `page.html`: 独立页面模板
- `archive.html`: 归档页面模板

模板使用 Tera 语法，可用变量:

- `site`: 站点配置
- `post`: 当前文章 (在 post.html 中)
- `posts`: 文章列表 (在 index.html、archive.html 中)
- `page`: 当前页面 (在 page.html 中)

## Important Constraints

1. **仅限本项目使用**: 生成器不需要考虑通用性，可以做特定假设
2. **性能优先**: 构建速度要快，避免不必要的文件操作
3. **零 JavaScript 依赖** (生成器侧): 生成器本身不依赖 Node.js
4. **渐进增强**: 网站在禁用 JavaScript 时仍可阅读基本内容
5. **SEO 友好**: 生成的 HTML 应该对搜索引擎友好

## External Dependencies

### 构建依赖

- Rust toolchain (1.75+)
- Cargo

### 运行时依赖 (前端)

- Mermaid.js (CDN)
- KaTeX 或 MathJax (CDN)

### 开发工具

- rustfmt
- clippy

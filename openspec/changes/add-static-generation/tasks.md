## 1. 配置模块

- [ ] 1.1 创建 `config.rs`，定义配置数据结构 (`SiteConfig`, `BuildConfig`, `MarkdownConfig` 等)
- [ ] 1.2 实现 `config.yaml` 读取和解析
- [ ] 1.3 添加配置验证逻辑 (必填字段检查)
- [ ] 1.4 编写配置模块单元测试

## 2. 内容解析模块

- [ ] 2.1 创建 `content/mod.rs` 模块结构
- [ ] 2.2 实现 `frontmatter.rs` - YAML front matter 解析
- [ ] 2.3 定义 `Post` 和 `Page` 数据结构
- [ ] 2.4 实现 `markdown.rs` - 基础 Markdown 解析 (使用 pulldown-cmark)
- [ ] 2.5 实现文章/页面扫描和加载 (`load_posts`, `load_pages`)
- [ ] 2.6 编写内容模块单元测试

## 3. 渲染模块

- [ ] 3.1 创建 `render/mod.rs` 模块结构
- [ ] 3.2 实现 `syntax.rs` - 代码语法高亮 (使用 syntect)
- [ ] 3.3 实现 `template.rs` - Tera 模板加载和渲染
- [ ] 3.4 将语法高亮集成到 Markdown 渲染流程
- [ ] 3.5 编写渲染模块单元测试

## 4. 构建模块

- [ ] 4.1 创建 `build.rs` 构建协调模块
- [ ] 4.2 实现文章页面生成 (`/posts/{slug}/index.html`)
- [ ] 4.3 实现独立页面生成 (`/{slug}/index.html`)
- [ ] 4.4 实现首页生成 (文章列表)
- [ ] 4.5 实现归档页面生成
- [ ] 4.6 实现标签页面生成
- [ ] 4.7 实现分类页面生成
- [ ] 4.8 实现静态资源复制
- [ ] 4.9 编写构建模块集成测试

## 5. CLI 集成

- [ ] 5.1 重构 `main.rs`，添加 `build` 子命令
- [ ] 5.2 添加命令行参数 (`--config`, `--output`)
- [ ] 5.3 添加日志输出 (构建进度、错误信息)

## 6. 验收测试

- [ ] 6.1 使用现有 `content/` 内容运行完整构建
- [ ] 6.2 验证所有生成的 HTML 页面可正常渲染
- [ ] 6.3 验证代码高亮正常工作
- [ ] 6.4 验证静态资源正确复制

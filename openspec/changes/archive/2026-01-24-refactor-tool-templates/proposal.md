# Proposal: refactor-tool-templates

## Why

当前工具页面（如 md2wechat）使用独立的 HTML 文件，通过 `copy_dir_recursive` 直接复制到输出目录。这导致：

1. **维护困难**: 每次修改站点头部/导航栏，都需要手动更新每个工具页面
2. **一致性问题**: 工具页面容易与主站风格脱节
3. **重复代码**: 每个工具页面都要复制相同的 header/footer/GA 代码
4. **容易遗漏**: 新增全站功能（如搜索）时容易忘记更新工具页面

## What Changes

将工具页面从"纯 HTML 复制"改为"Tera 模板渲染"，使其复用主站的 base.html：

1. **创建 tool-page.html 模板** - 继承 base.html，定义工具页面专用的 block
2. **工具内容放在 content block 中** - 工具特有的 HTML/CSS/JS 通过 Tera block 注入
3. **修改构建流程** - 将工具的 template.html 作为 Tera 模板渲染，而非直接复制
4. **保留静态资源复制** - 工具目录中的非模板文件（图片等）仍然直接复制

### 新的工具目录结构

```
content/tools/
├── _meta.yaml           # 工具元数据
└── md2wechat/
    ├── template.html    # Tera 模板（继承 tool-page.html）
    └── assets/          # 可选：工具专用静态资源
```

### 模板继承关系

```
base.html
└── tool-page.html (可选中间层)
    └── content/tools/{slug}/template.html
```

## Benefits

- **自动同步**: 修改 base.html 的 header/footer，所有工具页面自动更新
- **一致体验**: 工具页面与主站共享相同的导航、搜索、暗色模式
- **减少重复**: 不再需要在每个工具页面复制 GA/CSS/header 代码
- **易于扩展**: 添加新工具只需关注工具本身的功能

## Scope

- 修改 generator 的构建逻辑
- 将现有 md2wechat 工具迁移到模板方式
- 保持工具功能完全不变

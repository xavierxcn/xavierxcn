# Tasks

## 1. 修改构建流程

### 1.1 更新工具渲染逻辑
- [x] 修改 `generate_tools` 函数，识别 `template.html` 文件
- [x] 如果存在 `template.html`，使用 Tera 渲染；否则保持原有复制行为
- [x] 渲染时传入 site、nav、tool 等上下文变量
- [x] 复制工具目录中的非模板文件（assets 等）

### 1.2 添加 Tool 上下文
- [x] 在渲染工具模板时，传入当前工具的 meta 信息
- [x] 传入 `site`、`nav`、`config` 等标准上下文

## 2. 迁移 md2wechat 工具

### 2.1 创建 template.html
- [x] 将现有 index.html 转换为 Tera 模板
- [x] 使用 `{% extends "base.html" %}` 继承基础模板
- [x] 将工具特有内容放入 `{% block content %}`
- [x] 将工具特有 CSS 放入 `{% block head %}`
- [x] 将工具特有 JS 放入 `{% block scripts %}`

### 2.2 清理冗余代码
- [x] 移除原 index.html 中的 header/footer HTML
- [x] 移除重复的 GA 代码
- [x] 移除重复的 CSS 引用
- [x] 保留工具专用的 CSS 和 JS

## 3. 验证

### 3.1 功能验证
- [x] 构建成功
- [x] 工具页面正确显示主站 header/footer
- [x] 搜索功能正常工作
- [x] 工具核心功能（Markdown 转换、复制等）正常
- [x] 暗色模式正常

### 3.2 回归测试
- [x] 工具列表页 `/tools/` 正常显示
- [x] 主站其他页面不受影响

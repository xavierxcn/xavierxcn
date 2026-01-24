# Proposal: fix-tool-page-header

## Why

工具页面（如 md2wechat）的头部导航栏与主站不一致：
- 使用了旧的站点标题 "Xavier's Blog"
- 没有搜索功能
- 没有 Google Analytics
- 导航栏样式与主站不同
- 没有使用主站的 CSS 变量和暗色模式支持

## What Changes

更新工具页面的 HTML 结构，使其头部导航栏与主站保持一致：

1. **更新头部 HTML 结构** - 使用与 base.html 相同的 header 结构
2. **引入主站 CSS** - 链接到 `/xavierxcn/static/css/style.css`
3. **添加 Google Analytics** - 添加 GA 跟踪代码
4. **添加搜索功能** - 添加搜索输入框和相关脚本
5. **更新站点标题** - 使用新标题 "xavier的即兴发挥"

由于工具页面是独立的 HTML 文件（直接复制到输出目录），需要手动更新每个工具页面的 HTML。

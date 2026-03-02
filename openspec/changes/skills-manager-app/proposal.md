## Why

目前 AI 编辑器（Cursor、Claude Code 等）的 skills 管理分散且缺乏统一的可视化工具。用户需要手动浏览文件系统来查看、启用或禁用 skills，操作繁琐且容易出错。尤其在 symlinks 模式下，skills 的启用/禁用涉及文件系统软链接操作，用户需要一个直观的桌面应用来统一管理这些 skills，降低操作门槛。

## What Changes

- 新建基于 Tauri v2 的跨平台桌面应用（支持 macOS 和 Windows）
- 前端使用 Vue 3 + TypeScript，采用新拟态（Neumorphism）UI 风格
- 实现左右分栏布局（3:7 比例），左侧为编辑器筛选与 skills 列表，右侧为 skills 详情面板
- 支持多编辑器筛选（Cursor、Claude Code），可多选过滤
- 展示 skills 详情：版本、元信息（作者、许可证等）、指令内容
- 实现 skills 开关功能，兼容 symlinks 模式和 copy 模式：
  - symlinks 模式：skills 存放在 `~/.agents/skills/` 下，通过软链接到各编辑器 skills 目录；禁用时移除软链接并将 skills 转移到临时隐藏目录
  - copy 模式：skills 直接安装到编辑器目录；禁用时移到临时隐藏目录
- 开关操作需处理 IO 状态管理（加载、禁用状态），防止用户快速连续操作导致文件系统竞态

## Capabilities

### New Capabilities
- `skill-discovery`: 扫描和发现本地已安装的 skills，解析 SKILL.md 元信息（name、description、version、author、license 等）
- `editor-detection`: 检测系统中已安装的 AI 编辑器及其 skills 目录路径，支持 macOS 和 Windows 路径差异
- `skill-toggle`: skills 启用/禁用功能，支持 symlinks 和 copy 两种模式的正确处理，包含并发操作保护
- `skill-detail-view`: skills 详情展示面板，渲染版本信息、元数据、指令内容及 Markdown 格式化显示
- `editor-filter`: 编辑器多选筛选组件，根据选中的编辑器过滤 skills 列表
- `neumorphic-ui`: 新拟态风格 UI 组件库，包含按钮、卡片、开关、选择器等基础组件

### Modified Capabilities

（无已有能力需要修改，这是全新项目）

## Impact

- **技术栈**：Tauri v2 (Rust 后端) + Vue 3 + TypeScript + Vite（前端）
- **文件系统**：需要读写 `~/.agents/skills/`、`~/.cursor/skills/`、`~/.claude/skills/` 等目录，涉及软链接的创建和删除
- **平台差异**：macOS 和 Windows 下 skills 目录路径不同，symlink 行为也有差异（Windows 需要管理员权限或开发者模式）
- **依赖**：Tauri v2、Vue 3、Vite、可能需要 markdown 解析库（如 markdown-it）用于渲染 SKILL.md 内容
- **权限**：Windows 下创建 symlinks 可能需要特殊权限处理

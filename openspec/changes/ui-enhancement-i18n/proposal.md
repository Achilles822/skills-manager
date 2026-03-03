## Why

当前 Skills Manager 存在以下体验不足：

1. **Skill 详情展示过于单一**：选中 skill 后，右侧只展示 SKILL.md 的渲染内容，但实际上一个 skill 可能包含 `scripts/`、`references/`、`assets/`、`templates/` 等子目录和文件。用户无法在应用内浏览和编辑这些关联文件，必须切换到文件管理器操作，体验割裂。

2. **编辑模式按钮顺序不合理**：编辑模式下保存按钮在左、取消按钮在右，不符合多数桌面应用的惯例（通常取消在左、确认/保存在右），容易误操作。

3. **缺少多语言支持**：当前所有 UI 文本均为硬编码中文，无法服务英文用户。需要引入国际化（i18n）机制，并在设置界面中提供语言切换入口。同时设置界面需要重构，新增保存按钮让配置变更可控。

## What Changes

### 1. Skill 目录浏览器（Skill File Explorer）

- 选中 skill 后，右侧面板采用左右分栏布局：
  - **左侧**：展示当前 skill 的目录树（文件/文件夹层级结构）
  - **右侧**：文件内容查看/编辑区
- 默认自动打开并展示 `SKILL.md`
- 支持浏览 skill 目录下的所有文件和子目录（`scripts/`、`references/`、`assets/`、`templates/` 等）
- 用户可点击目录树中的文件查看内容
- 编辑模式下可编辑选中的文件
- 对于脚本文件（`.sh`、`.py`、`.js`、`.ts` 等），查看和编辑时需支持代码语法高亮

### 2. 编辑模式按钮顺序调整

- 将编辑模式下的"取消"按钮移到左侧，"保存"按钮移到右侧
- 符合桌面应用通用交互规范

### 3. 多语言（i18n）支持

- 在设置界面弹窗（由齿轮图标触发）顶部新增"界面语言"（Display Language）选择器
- 支持中文简体和英文两种界面语言
- 设置弹窗底部新增"保存"按钮，点击后应用设置变更
- 引入前端国际化框架（vue-i18n），将所有硬编码 UI 文本提取为翻译 key
- 语言偏好持久化存储（localStorage 或 Tauri Store）

## Capabilities

### New Capabilities

- `skill-file-explorer`: Skill 目录树浏览器，展示选中 skill 的完整文件结构，支持文件选择、内容查看和编辑
- `code-syntax-highlight`: 代码语法高亮，为脚本文件（.sh、.py、.js、.ts 等）提供语法着色显示
- `i18n-support`: 国际化支持，界面语言可切换（中文简体/英文），所有 UI 文本支持翻译
- `settings-panel`: 设置面板重构，新增语言选择和保存功能

### Modified Capabilities

- `skill-detail-view`: 重构 SkillDetail 组件，从单一 Markdown 查看/编辑变为目录树 + 文件查看/编辑分栏布局
- `neumorphic-ui`: 可能需要新增文件树组件样式

## Impact

- **前端组件**：需要重构 `SkillDetail.vue`，新增 `SkillFileTree.vue`、`FileViewer.vue`、`SettingsDialog.vue` 等组件；新增 `src/i18n/` 国际化模块
- **后端 (Rust)**：需要新增 Tauri Command 来列出 skill 目录文件结构、读取任意文件内容、保存任意文件内容
- **依赖**：需要新增 `vue-i18n`（国际化）、`highlight.js` 或 `shiki`（代码高亮）
- **存储**：语言偏好需要持久化，可使用 localStorage 或 tauri-plugin-store
- **兼容性**：所有改动均为前端 + 少量后端扩展，不影响现有 skill 管理核心逻辑

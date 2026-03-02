## 1. 项目脚手架与基础配置

- [ ] 1.1 使用 `pnpm create tauri-app` 初始化 Tauri v2 项目，前端选择 Vue 3 + TypeScript + Vite 模板
- [ ] 1.2 配置 Tauri 权限：在 `src-tauri/capabilities/` 中声明所需的文件系统访问权限和 shell 执行权限
- [ ] 1.3 配置 Vite 和 TypeScript：设置路径别名（`@/`）、启用严格模式
- [ ] 1.4 添加前端依赖：`markdown-it`（Markdown 渲染）
- [ ] 1.5 创建前端项目目录结构：`src/components/`、`src/views/`、`src/composables/`、`src/styles/`、`src/types/`

## 2. Rust 后端 - 编辑器注册表与平台模块

- [ ] 2.1 定义 `EditorDefinition` trait：包含 `id()`、`display_name()`、`icon()`、`skills_dir()`、`config_dir()` 方法
- [ ] 2.2 创建 `EditorRegistry` 结构体：管理编辑器注册、遍历和查找
- [ ] 2.3 实现 `CursorEditor` 和 `ClaudeCodeEditor` 两个内置编辑器定义，包含跨平台路径逻辑
- [ ] 2.4 创建 `platform` 模块：实现跨平台用户主目录获取
- [ ] 2.5 实现 Windows symlink 权限检测函数
- [ ] 2.6 创建 `detect_editors` Tauri Command：遍历注册表检测已安装编辑器，返回编辑器列表及其 skills 目录路径

## 3. Rust 后端 - Skill 发现与解析

- [ ] 3.1 创建 `skill` 模块：定义 `Skill`、`SkillMeta`、`InstallMode`（Symlink/Copy）等数据结构，派生 Serialize
- [ ] 3.2 实现 `SKILL.md` 解析函数：读取文件、提取 YAML front matter 元信息和 Markdown 正文内容
- [ ] 3.3 实现 `scan_skills` 函数：利用 `EditorRegistry` 扫描中心目录和各编辑器 skills 目录，自动检测安装模式
- [ ] 3.4 实现 skills 去重合并逻辑：以中心目录为主记录，关联各编辑器链接状态
- [ ] 3.5 创建 `list_skills` Tauri Command：支持按编辑器筛选参数，返回 skills 列表给前端
- [ ] 3.6 创建 `get_skill_detail` Tauri Command：返回指定 skill 的完整详情（元信息 + Markdown 正文原始内容）

## 4. Rust 后端 - Skill 开关功能

- [ ] 4.1 创建 `toggle` 模块：实现操作锁机制（per-skill Mutex），防止同一 skill 的并发操作
- [ ] 4.2 实现 `disable_skill_symlink` 函数：移除 symlinks → 将 skill 目录移到 `~/.agents/.disabled-skills/`
- [ ] 4.3 实现 `enable_skill_symlink` 函数：从隐藏目录移回 → 重建 symlinks 到各关联编辑器
- [ ] 4.4 实现 `disable_skill_copy` 函数：将 skill 目录移到 `<editor-dir>/.disabled/`
- [ ] 4.5 实现 `enable_skill_copy` 函数：从隐藏目录移回编辑器 skills 目录
- [ ] 4.6 实现错误回滚逻辑：操作失败时尝试恢复原始状态
- [ ] 4.7 实现隐藏目录自动创建逻辑
- [ ] 4.8 创建 `toggle_skill` Tauri Command：接收 skill 名称和目标状态，根据安装模式调用对应函数

## 5. Rust 后端 - Skill 内容编辑

- [ ] 5.1 创建 `save_skill_content` Tauri Command：接收 skill 路径和新内容，写回 SKILL.md 文件
- [ ] 5.2 实现写入前的基本校验（文件存在性、权限检查）
- [ ] 5.3 保存时先写入临时文件再重命名，确保原子性

## 6. 前端 - 新拟态 UI 基础

- [ ] 6.1 创建全局 CSS 变量和新拟态基础样式：定义背景色（`#e0e5ec`）、阴影参数、圆角尺寸等
- [ ] 6.2 实现 `NeuCard` 组件：新拟态卡片，支持 hover 阴影变化和 selected 内凹效果
- [ ] 6.3 实现 `NeuToggle` 组件：新拟态开关，支持 on/off/loading 三种状态和平滑过渡动画
- [ ] 6.4 实现 `NeuSelect` 组件：新拟态多选选择器，选中时内凹效果
- [ ] 6.5 实现 `NeuButton` 组件：新拟态按钮，支持 pressed 效果
- [ ] 6.6 实现 `NeuTag` 组件：新拟态标签，支持不同颜色变体（用于 Symlink/Copy 标识）
- [ ] 6.7 自定义滚动条样式，使其与新拟态风格一致

## 7. 前端 - 主布局与编辑器筛选

- [ ] 7.1 创建 `AppLayout` 组件：实现左右分栏布局（3:7 比例），左侧面板和右侧面板
- [ ] 7.2 创建 `EditorFilter` 组件：使用 `NeuSelect` 实现编辑器多选筛选，动态从 `detect_editors` 获取可用编辑器列表
- [ ] 7.3 创建 `useEditorFilter` composable：管理筛选状态，提供响应式的当前筛选条件

## 8. 前端 - Skills 列表

- [ ] 8.1 创建 `useSkills` composable：调用 `list_skills` Command，管理 skills 列表数据和加载状态，响应筛选条件变化
- [ ] 8.2 创建 `SkillListItem` 组件：使用 `NeuCard` 展示 skill 摘要（名称、描述截断、编辑器图标、开关）
- [ ] 8.3 创建 `SkillList` 组件：渲染 skills 列表，处理空状态和加载状态
- [ ] 8.4 实现列表中 skill 选中逻辑，选中时通知详情面板

## 9. 前端 - Skill 详情面板

- [ ] 9.1 创建 `SkillDetail` 组件：展示 skill 基础信息（名称、版本、作者、许可证、关联编辑器）
- [ ] 9.2 使用 `NeuTag` 展示安装模式标签（Symlink 蓝色 / Copy 绿色），放置在 skill 名称旁
- [ ] 9.3 集成 `markdown-it` 渲染 SKILL.md 正文内容，配置代码高亮
- [ ] 9.4 实现"在文件管理器中打开"功能：调用 Tauri shell open API
- [ ] 9.5 在详情面板中集成 `NeuToggle` 启用/禁用开关，与列表中的开关状态同步
- [ ] 9.6 创建详情面板的占位状态（未选中 skill 时的提示界面）

## 10. 前端 - Skill 内容编辑功能

- [ ] 10.1 实现编辑模式切换：详情面板增加"编辑"开关，切换 Markdown 渲染区为 textarea 编辑器
- [ ] 10.2 textarea 使用等宽字体，显示 SKILL.md 原始内容（含 YAML front matter）
- [ ] 10.3 实现保存和取消按钮：保存调用 `save_skill_content` Command，取消恢复原始内容
- [ ] 10.4 Symlinks 模式 skill 进入编辑时显示警告提示
- [ ] 10.5 创建 `useSkillEditor` composable：管理编辑状态、内容暂存、保存/取消逻辑

## 11. 前端 - Skill 开关集成

- [ ] 11.1 创建 `useSkillToggle` composable：调用 `toggle_skill` Command，管理每个 skill 的加载状态
- [ ] 11.2 将 toggle composable 集成到 `SkillListItem` 和 `SkillDetail` 组件，确保状态双向同步
- [ ] 11.3 实现错误处理 UI：操作失败时显示错误提示信息
- [ ] 11.4 实现 Windows 无 symlink 权限时的降级提示

## 12. 整体打磨与收尾

- [ ] 12.1 统一处理应用窗口配置：设置合理的默认窗口大小和最小窗口大小
- [ ] 12.2 处理应用加载时的初始化流程：启动时扫描 skills → 填充列表
- [ ] 12.3 添加应用图标和窗口标题
- [ ] 12.4 端到端功能自测：验证 skills 扫描、筛选、详情查看、编辑、开关功能的表现
- [ ] 12.5 优化性能：确保大量 skills 时列表渲染流畅

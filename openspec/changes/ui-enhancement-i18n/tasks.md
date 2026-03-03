## 1. 国际化（i18n）基础设施

- [x] 1.1 安装 `vue-i18n` 依赖
- [x] 1.2 创建 `src/i18n/zh-CN.ts` 中文翻译文件，提取所有现有硬编码中文文本为翻译 key
- [x] 1.3 创建 `src/i18n/en.ts` 英文翻译文件
- [x] 1.4 创建 `src/i18n/index.ts`，初始化 vue-i18n 实例，从 localStorage 读取语言偏好（默认 `zh-CN`）
- [x] 1.5 在 `src/main.ts` 中注册 vue-i18n 插件
- [x] 1.6 创建 `src/composables/useI18nPreference.ts`，封装语言偏好读取/保存逻辑

## 2. 全局组件 i18n 适配

- [x] 2.1 改造 `AppLayout.vue`：将所有硬编码文本替换为 `t()` 翻译调用
- [x] 2.2 改造 `SkillDetail.vue`：替换"版本"、"作者"、"编辑"、"保存"、"取消"等文本
- [x] 2.3 改造 `SkillList.vue`：替换加载提示、空状态文本
- [x] 2.4 改造 `SkillListItem.vue`：替换相关文本
- [x] 2.5 改造 `EditorFilter.vue`：替换 placeholder 等文本
- [x] 2.6 改造 `StatusFilter.vue`：替换"全部"、"已启用"、"已禁用"
- [x] 2.7 改造 `AboutDialog.vue`：替换"关于"弹窗中的文本
- [x] 2.8 改造 `ConfirmDialog.vue`：替换按钮文本

## 3. 设置弹窗

- [x] 3.1 创建 `SettingsDialog.vue` 设置弹窗组件，顶部为"界面语言"下拉选择器，底部为"保存"按钮
- [x] 3.2 实现语言切换预览：选择语言后界面立即切换，但未保存时关闭弹窗恢复原语言
- [x] 3.3 在 `AppLayout.vue` 中将齿轮图标按钮改为触发设置弹窗（保留关于弹窗独立入口）
- [x] 3.4 保存按钮逻辑：将语言偏好写入 localStorage 并确认 vue-i18n locale 更新

## 4. Rust 后端 — 文件操作扩展

- [x] 4.1 在 `commands.rs` 中新增 `list_skill_files` Command：接收 skill 目录路径，递归返回 `Vec<FileEntry>` 文件树结构
- [x] 4.2 定义 `FileEntry` 结构体：`{ name: String, path: String, is_dir: bool, children: Option<Vec<FileEntry>> }`
- [x] 4.3 在 `commands.rs` 中新增 `read_file_content` Command：读取指定文件内容，返回 `{ content: String, is_binary: bool }`
- [x] 4.4 实现二进制文件检测逻辑（检查文件头部字节是否包含非文本字符）
- [x] 4.5 扩展 `save_skill_content` 或新增通用 `save_file_content` Command，支持保存任意文件
- [x] 4.6 在 `lib.rs` 中注册新增的 Command

## 5. 前端 — Skill 目录树组件

- [x] 5.1 创建 `src/composables/useSkillFiles.ts`：调用 `list_skill_files` Command，管理文件树数据和当前选中文件
- [x] 5.2 创建 `SkillFileTree.vue`：递归渲染文件树，支持展开/收起目录、点击选择文件
- [x] 5.3 实现文件类型图标区分（文件夹、Markdown、脚本、配置文件、其他）
- [x] 5.4 实现 SKILL.md 默认选中逻辑
- [x] 5.5 实现文件切换时的未保存修改检测和提示

## 6. 前端 — 文件查看/编辑组件

- [x] 6.1 安装 `highlight.js` 依赖
- [x] 6.2 创建 `FileViewer.vue`：根据文件类型选择渲染方式（Markdown 渲染 / 代码高亮 / 纯文本）
- [x] 6.3 实现 Markdown 文件渲染（复用现有 markdown-it 逻辑）
- [x] 6.4 实现脚本文件代码高亮（highlight.js），支持 JavaScript、TypeScript、Python、Shell、JSON、YAML
- [x] 6.5 实现编辑模式：切换为 textarea 编辑器，保存调用 `save_file_content` Command
- [x] 6.6 调整编辑模式按钮顺序：取消按钮在左，保存按钮在右

## 7. 前端 — SkillDetail 重构

- [x] 7.1 重构 `SkillDetail.vue`：header + info 区域保持不变，下方改为左右分栏（SkillFileTree + FileViewer）
- [x] 7.2 实现智能分栏：当 skill 只有 SKILL.md 且无子目录时，隐藏目录树，保持原单文件查看体验
- [x] 7.2.1 实现目录树折叠/展开按钮：在文件树分栏边缘提供折叠/展开按钮，用户可手动收起目录树以获得更大内容区
- [x] 7.3 集成 useSkillFiles composable，skill 切换时自动加载文件树
- [x] 7.4 将编辑模式逻辑从 SkillDetail 下沉到 FileViewer，SkillDetail 只控制编辑模式开关

## 8. 样式与整合打磨

- [x] 8.1 为目录树组件编写新拟态风格样式（内凹面板、文件列表项 hover/active 效果）
- [x] 8.2 为代码高亮选择并适配新拟态配色 highlight.js 主题
- [x] 8.3 确保设置弹窗样式与 AboutDialog 风格一致
- [x] 8.4 测试中英文切换：验证所有组件文本正确切换
- [x] 8.5 测试目录树：验证多层级 skill 的文件树展示和文件查看/编辑功能
- [x] 8.6 测试按钮顺序：验证编辑模式取消在左、保存在右
- [x] 8.7 响应式检查：确保目录树分栏在窄窗口下不出现布局问题

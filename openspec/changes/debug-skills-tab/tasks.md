## 1. 后端基础设施

- [ ] 1.1 添加 `tauri-plugin-dialog` 依赖到 `Cargo.toml` 和 `tauri.conf.json`，在 `lib.rs` 中注册插件
- [ ] 1.2 新建 `src-tauri/src/debug_store.rs` 模块，定义 `DebugSkillsStore` 数据结构（`DebugFolder`、`LinkedSkill`），实现 `debug-skills.json` 的读写、添加文件夹、添加/移除 skill 记录方法
- [ ] 1.3 扩展 `Skill` 结构体（`skill.rs`），新增 `is_debug: bool`、`debug_status: Option<String>`（"normal"/"abnormal"）、`debug_source_path: Option<String>` 字段
- [ ] 1.4 在 `scan_skills` 函数中加载 `debug-skills.json`，对匹配的 center skills 设置调试相关字段，执行一致性检查（软链是否完好、指向是否正确）

## 2. 后端命令（IPC 接口）

- [ ] 2.1 新增 `scan_external_folder` 命令：接收文件夹路径，扫描直接子目录中含 SKILL.md 的 skills，返回列表（名称、描述、目录名、是否与全局冲突）
- [ ] 2.2 新增 `link_debug_skills` 命令：接收文件夹路径和选中的 skill 目录名列表、冲突覆盖策略，创建软链到 `~/.agents/skills/`，写入持久化记录
- [ ] 2.3 新增 `uninstall_debug_skill` 命令：根据 skill 的 debug_status 决定是仅移除记录（异常）还是删除软链+移除记录（正常），不删除源文件
- [ ] 2.4 新增 `list_debug_skills` 命令：读取 `debug-skills.json`，返回所有调试文件夹和 skills 记录
- [ ] 2.5 在 `lib.rs` 的 `invoke_handler` 中注册所有新增命令

## 3. 前端类型和数据层

- [ ] 3.1 更新 `src/types/index.ts`，为 `Skill` 接口新增 `is_debug`、`debug_status`、`debug_source_path` 字段
- [ ] 3.2 新增 `src/composables/useDebugSkills.ts` composable，封装调试 skills 的数据获取、文件夹扫描、链接导入、卸载等 invoke 调用

## 4. Tab 切换布局

- [ ] 4.1 修改 `AppLayout.vue`，将品牌文字替换为 Tab 切换组件（管理/调试），添加 `activeTab` 状态管理
- [ ] 4.2 根据 `activeTab` 条件渲染管理 Tab 内容或调试 Tab 内容，调试 Tab 复用 `EditorFilter`、`StatusFilter`、`SkillList`、`SkillDetail`
- [ ] 4.3 调试 Tab 在筛选区域上方添加"添加文件夹"按钮
- [ ] 4.4 Tab 切换时的样式设计，保持 Neumorphic UI 风格一致

## 5. 调试功能 UI 组件

- [ ] 5.1 新增 `SkillSelectDialog.vue` 弹窗组件：展示扫描到的 skills 列表，支持 checkbox 多选、冲突标记、确认/取消按钮
- [ ] 5.2 弹窗中对冲突的 skills 显示警告图标和提示文案，确认时二次确认覆盖
- [ ] 5.3 在调试 Tab 中集成添加文件夹 → 扫描 → 弹窗选择 → 链接的完整流程

## 6. 调试标签和详情展示

- [ ] 6.1 修改 `SkillListItem.vue`，当 `skill.is_debug` 为 true 时显示"本地调试"标签（使用独特颜色）
- [ ] 6.2 当 `skill.debug_status === 'abnormal'` 时，额外显示"异常"警告标签
- [ ] 6.3 修改 `SkillDetail.vue`，当 `skill.is_debug` 为 true 时在路径字段下方显示"实际路径"（`debug_source_path`）
- [ ] 6.4 异常状态 skill 的详情页显示警告提示信息

## 7. 卸载逻辑适配

- [ ] 7.1 修改卸载流程：调试 skill 卸载时调用 `uninstall_debug_skill` 而非 `uninstall_skill`
- [ ] 7.2 调试 skill 的卸载确认弹窗文案调整，说明仅删除软链不删除源文件
- [ ] 7.3 异常状态 skill 的卸载确认弹窗文案调整，说明仅移除调试记录

## 8. 国际化

- [ ] 8.1 在 `zh-CN.ts` 和 `en.ts` 中新增调试相关的 i18n key：Tab 文案（管理、调试）、添加文件夹、skills 选择弹窗、本地调试标签、异常标签、实际路径、卸载提示等

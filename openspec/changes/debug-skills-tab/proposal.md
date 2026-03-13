## Why

用户在其他项目文件夹中开发 skills，开发完成后需要将它们快速安装到全局 (`~/.agents/skills/`) 进行测试和调试。目前没有便捷的方式从外部项目文件夹软链 skills 到全局目录，用户只能手动执行命令行操作。新增"调试"功能可以让用户在 GUI 中完成这一流程。

## What Changes

- 顶部保留 "Skills Manager" 品牌文字，标题栏操作区新增 "+" 按钮用于导入外部调试 skills
- 所有 skills（普通 + 调试）在同一个列表中展示，调试 skills 通过"本地调试"标签区分
- 点击 "+" 按钮，通过系统文件选择器选择包含多个 skills 的父文件夹，展示可导入的 skills 列表供用户勾选，同时选择目标编辑器
- 确认后将选中的 skills 软链到 `~/.agents/skills/`（center），并根据用户选择的编辑器在对应编辑器 skills 目录中创建指向 center 的软链
- 同一个 skill 在多个编辑器中软链时，列表中只显示一条记录，卡片上通过小按钮（如 C / CC）切换各编辑器的关联状态
- 调试列表持久化存储（`app_data_dir` 下的 `debug-skills.json`）
- 搜索框位于标题栏与列表之间，右侧有折叠按钮（默认收起），展开后显示编辑器筛选和状态筛选
- skills 详情页新增"实际路径"显示（软链目标路径），编辑器关联通过 checkbox 直接管理
- 卸载调试 skill 时仅删除软链和调试记录，不删除源文件
- 冲突检测：导入时同名 skill 提示是否覆盖；扫描时检测调试软链是否被外部覆盖（非软链），标记异常状态提示用户
- 异常状态下点击卸载仅移除调试记录，不执行文件删除操作
- 编辑器关联检测使用 `link_points_to_center` 辅助函数，同时比较原始路径和规范化路径，兼容 Windows 的 `\\?\` 前缀和 reparse point

## Capabilities

### New Capabilities
- `debug-skills-link`: 调试 skills 的核心功能——扫描外部文件夹、软链到全局及编辑器目录、持久化记录、冲突检测、异常状态检测、安全卸载
- `debug-skill-tag`: 在所有 skill 列表卡片上标识调试 skills（"本地调试"标签），详情页显示实际路径
- `search-filter-ui`: 搜索框 + 可折叠筛选面板，支持按名称搜索、按编辑器和状态筛选
- `editor-toggle-per-skill`: 每个 center skill 卡片上显示各已安装编辑器的关联开关，详情页通过 checkbox 管理

### Modified Capabilities
- `skill-list`: 合并展示所有 skills（普通和调试），不再使用 Tab 切换
- `editor-detection`: `link_points_to_center` 函数取代 `canonicalize` 比较，正确处理多层软链和 Windows junction

## Impact

- **前端**：`AppLayout.vue` 新增搜索框和可折叠筛选；`SkillListItem.vue` 新增编辑器按钮和调试标签；`SkillDetail.vue` 新增编辑器 checkbox 和实际路径
- **后端（Rust）**：新增 `link_points_to_center` 辅助函数修复路径比较；新增扫描外部文件夹、创建软链、读写调试记录等 IPC 命令；引入 `tauri-plugin-dialog` 和 `chrono` 依赖
- **数据类型**：`Skill` 结构体新增 `is_debug`、`debug_status`、`debug_source_path` 字段
- **i18n**：新增调试、搜索、筛选相关的中英文翻译
- **持久化**：`app_data_dir` 下新增 `debug-skills.json` 配置文件

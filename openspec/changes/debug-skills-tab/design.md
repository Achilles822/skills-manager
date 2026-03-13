## Context

Skills Manager 是一个基于 Tauri + Vue 3 的桌面应用，用于管理 AI 编辑器（Cursor、Claude Code）的 skills。当前支持查看、启用/禁用、卸载全局 center skills 和编辑器级别的 copy/symlink skills。

用户在外部项目文件夹中开发 skills，完成后需要软链到全局 `~/.agents/skills/` 进行调试。当前缺乏 GUI 支持，只能通过命令行手动操作。

**现有技术栈**：Tauri 2（Rust 后端）、Vue 3 Composition API + `<script setup>`、vue-i18n、Neumorphic UI 设计系统。

## Goals / Non-Goals

**Goals:**
- 在现有布局中新增 Tab 切换，将品牌文字替换为"管理"/"调试"两个 Tab
- 提供从外部文件夹批量导入 skills 到全局的软链功能
- 持久化记录调试 skills 的来源信息，支持冲突检测和异常状态标识
- 调试 skills 在所有列表中通过"本地调试"标签可视化区分
- 卸载调试 skill 时安全地仅删除软链，保留源文件

**Non-Goals:**
- 不实现自动文件监听/热重载（skill 源文件变更后不自动同步）
- 不实现调试 skill 向编辑器特定目录的单独链接管理（统一软链到全局 `~/.agents/skills/`）
- 不实现远程/网络 skill 的导入

## Decisions

### 1. 持久化方案：Tauri app_data_dir 下的 JSON 文件

**选择**：在 Tauri 的 `app_data_dir`（各平台标准位置）下存储 `debug-skills.json`。

**数据结构**：
```json
{
  "folders": [
    {
      "path": "D:\\projects\\my-skills",
      "linked_skills": [
        {
          "dir_name": "vue",
          "linked_at": "2026-03-11T10:00:00Z"
        }
      ]
    }
  ]
}
```

**替代方案**：localStorage（不适合，因为 Rust 后端也需要读写）、软件目录下写文件（不符合操作系统规范）。

**理由**：`app_data_dir` 是 Tauri 推荐的持久化位置，跨平台一致；JSON 格式简单，Rust 原生 serde 支持好；后端和前端都能方便地读写。

### 2. 文件选择器：tauri-plugin-dialog

**选择**：使用 `tauri-plugin-dialog` 的文件夹选择对话框。

**理由**：Tauri 官方插件，跨平台原生对话框，无需自行实现。需要在 `Cargo.toml` 和 `tauri.conf.json` 中添加依赖。

### 3. 调试 skill 标识方式：通过持久化记录关联

**选择**：扫描 skills 时，将 `~/.agents/skills/` 中的 center skills 与 `debug-skills.json` 中的记录做匹配。如果某个 skill 的 `dir_name` 出现在调试记录中，则标记为调试 skill。

**替代方案**：在 skill 文件中嵌入标记（侵入性太强）、通过软链目标路径推断（不够可靠，因为软链可能被覆盖）。

**理由**：非侵入性，不修改 skill 文件本身；可靠检测异常状态（记录中有但文件系统上不是软链 → 异常）。

### 4. 冲突与异常检测逻辑

**导入时冲突**：创建软链前检查 `~/.agents/skills/<dir_name>` 是否已存在。若存在则提示用户是否覆盖（先删除再创建软链）。

**运行时异常检测**：扫描时对照 `debug-skills.json` 记录，检查 `~/.agents/skills/<dir_name>` 是否仍为指向记录中原始路径的软链：
- 是软链且指向正确 → 正常
- 不存在 → 可能被手动删除 → 标记异常
- 存在但不是软链 → 被外部覆盖 → 标记异常
- 是软链但指向不同路径 → 标记异常

**异常状态下卸载**：仅从 `debug-skills.json` 中移除记录，不执行任何文件系统删除操作。

### 5. Tab 布局实现：AppLayout 内部状态切换

**选择**：在 `AppLayout.vue` 中使用 `ref<'manage' | 'debug'>` 控制当前 Tab，两个 Tab 共享顶部区域（Tab 切换 + 刷新/设置按钮），下方内容区根据 Tab 切换显示不同组件。

**理由**：改动最小，不引入路由，保持单页应用的简洁性。

### 6. 调试 Tab 的 UI 复用策略

**选择**：调试 Tab 复用现有的 `EditorFilter`、`StatusFilter`、`SkillList`、`SkillDetail` 组件，新增"添加文件夹"按钮和 skills 选择弹窗。调试 Tab 的数据流与管理 Tab 类似，但数据源过滤为仅包含调试记录中的 skills。

**理由**：最大化代码复用，保持 UI 一致性；用户体验无缝衔接。

## Risks / Trade-offs

- **[风险] 权限问题**：Windows 上创建 symlink 可能需要开发者模式或管理员权限 → 已有处理（现有 toggle.rs 中已使用 `symlink_dir`），创建失败时返回明确错误信息。

- **[风险] debug-skills.json 与文件系统不同步**：用户可能在外部操作文件系统 → 每次扫描时做一致性检查，标记异常状态。

- **[风险] 调试文件夹被删除或移动**：软链目标不存在 → 软链变为 dangling link，扫描时检测并标记异常。

- **[权衡] 调试 Tab 保留编辑器筛选**：当前调试 skills 统一软链到全局 `.agents/skills/`，编辑器筛选的实际意义有限，但保留以便未来扩展。

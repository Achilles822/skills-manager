## Context

当前 AI 编辑器的 skills 管理依赖文件系统手动操作。`vercel-labs/skills` 工具支持两种安装模式：

- **symlinks 模式**：skills 安装到 `~/.agents/skills/` 中心目录，然后创建软链接到各编辑器的 skills 目录（如 `~/.cursor/skills/`、`~/.claude/skills/`）
- **copy 模式**：skills 直接复制到对应编辑器的 skills 目录中

本项目需要构建一个跨平台桌面应用，让用户能够可视化地管理这些 skills。项目面向技术用户（AI 编辑器使用者），但仍需提供优秀的用户体验。

**当前约束**：
- macOS 和 Windows 的文件路径、symlink 行为不同
- Windows 创建 symlinks 需要管理员权限或开启开发者模式
- skills 的 SKILL.md 文件是自描述的元信息来源

## Goals / Non-Goals

**Goals:**
- 提供统一的 GUI 管理 Cursor 和 Claude Code 的 skills
- 支持 macOS 和 Windows 双平台
- 正确处理 symlinks 和 copy 两种模式的 skills 启用/禁用
- 新拟态 UI 风格，界面清新有质感
- 安全的文件系统操作，防止竞态条件

**Non-Goals:**
- 不实现 skills 的在线搜索和下载（仅管理本地已安装的 skills）
- 不支持 Linux 系统（可后续扩展）
- 不实现 skills 的创建功能（仅支持编辑已有 skills）
- 不处理编辑器本身的安装或配置
- 不实现 skills 的版本更新功能

## Decisions

### 1. 技术栈：Tauri v2 + Vue 3 + TypeScript

**选择**：使用 Tauri v2 作为桌面框架，Vue 3 + Composition API + TypeScript 作为前端栈，Vite 作为构建工具。

**理由**：Tauri v2 体积小、性能好，Rust 后端天然适合处理文件系统操作和跨平台兼容性。Vue 3 的 Composition API 配合 TypeScript 提供良好的类型安全和代码组织能力。

**备选方案**：Electron（体积过大）、Flutter（生态不够成熟）。

### 2. 后端架构：Rust Tauri Commands

**选择**：所有文件系统操作（扫描 skills、创建/删除 symlinks、移动文件）在 Rust 后端通过 Tauri Commands 实现。

**理由**：
- Rust 处理文件 IO 性能优异且类型安全
- 可以利用 Rust 的错误处理机制确保操作的原子性
- Tauri Commands 提供了前后端的类型安全通信

**备选方案**：直接在前端通过 Tauri 的 fs plugin 操作（缺乏事务性保障）。

### 2.5. 编辑器注册表：可扩展的编辑器支持架构

**选择**：使用 `EditorRegistry` trait + 注册表模式。每个编辑器实现统一的 trait 接口，通过注册表动态管理。新增编辑器只需添加一个新的 struct 实现 trait 并注册即可。

```rust
trait EditorDefinition {
    fn id(&self) -> &str;           // "cursor", "claude-code"
    fn display_name(&self) -> &str; // "Cursor", "Claude Code"
    fn skills_dir(&self, home: &Path) -> PathBuf;
    fn config_dir(&self, home: &Path) -> PathBuf;
    fn icon(&self) -> &str;         // 前端图标标识
}
```

**理由**：
- 后续扩展新编辑器（如 Windsurf、Cline 等）只需新增一个实现文件，不需修改核心扫描/切换逻辑
- 统一接口使得 skills 扫描、开关、路径解析等操作对所有编辑器行为一致
- 前端也可以通过 `detect_editors` Command 获取动态编辑器列表，无需硬编码 UI 选项

**备选方案**：硬编码每个编辑器的路径（不利于扩展，新增编辑器需要改多处代码）。

### 3. Skills 禁用策略：隐藏目录 + 状态文件

**选择**：
- 在 `~/.agents/.disabled-skills/` 目录存放被禁用的 skills
- 禁用时：将 skills 移动到隐藏目录，移除相关 symlinks
- 启用时：从隐藏目录移回并重建 symlinks
- 使用操作锁（Mutex）防止并发操作同一个 skill

**理由**：文件移动是原子操作（同一分区内），比标记文件更可靠。使用 Mutex 避免用户快速连续点击导致的竞态。

**备选方案**：使用 JSON 状态文件标记禁用（但 symlinks 仍存在，编辑器仍会加载）。

### 4. 安装模式检测

**选择**：通过文件系统状态自动检测 skills 的安装模式：
- 如果编辑器 skills 目录下的条目是 symlink → symlinks 模式
- 如果是普通目录 → copy 模式

**理由**：自动检测无需用户配置，减少使用门槛。

### 5. UI 框架：纯 CSS 新拟态 + CSS 变量

**选择**：不引入第三方 UI 组件库，使用纯 CSS 实现新拟态风格，通过 CSS 变量管理主题。

**理由**：新拟态风格有独特的 shadow 和层次表现，市面上没有成熟的新拟态组件库，自定义实现可以获得更精确的视觉控制。

**备选方案**：基于 Element Plus 等库做主题定制（风格差异过大，定制成本高）。

### 6. 编辑器 Skills 路径映射

**选择**：每个 `EditorDefinition` 实现自身的路径逻辑，通过 trait 方法返回。内置编辑器的默认路径如下：

| 编辑器 | Windows | macOS |
|--------|---------|-------|
| Cursor | `%USERPROFILE%\.cursor\skills\` | `~/.cursor/skills/` |
| Claude Code | `%USERPROFILE%\.claude\skills\` | `~/.claude/skills/` |
| 中心目录 | `%USERPROFILE%\.agents\skills\` | `~/.agents/skills/` |

支持通过配置文件覆盖默认路径。扩展新编辑器时只需在其 struct 中实现 `skills_dir` 和 `config_dir` 方法。

**理由**：路径与编辑器定义绑定，新增编辑器时路径信息自包含，无需修改全局映射表。

### 7. Skill 内容编辑

**选择**：在详情面板提供编辑开关，开启后将 Markdown 渲染区切换为文本编辑器（使用 `<textarea>` + 等宽字体），用户修改后保存回 `SKILL.md` 文件。

**理由**：
- 用户可能需要微调 skill 的指令内容以适应自身工作流
- 直接编辑 `SKILL.md` 原文件（而非副本），确保编辑器下次加载时生效
- 使用简单的 textarea 而非富文本编辑器，保持 Markdown 源码的直接控制

**风险**：编辑 symlinks 模式的 skill 会影响所有链接到该 skill 的编辑器。在 UI 中需明确提示此行为。

## Risks / Trade-offs

- **[Windows Symlink 权限]** → Windows 创建 symlinks 需要特殊权限。**缓解**：检测权限状态，无权限时提示用户启用开发者模式，或降级为 copy 模式操作。
- **[文件系统竞态]** → 用户快速连续操作可能导致文件状态不一致。**缓解**：每个 skill 的操作使用独立锁（Mutex），操作进行中禁用 UI 交互按钮。
- **[路径变化]** → 编辑器更新可能改变 skills 目录位置。**缓解**：支持配置文件覆盖默认路径。
- **[大量 Skills]** → 如果安装了非常多 skills，扫描可能变慢。**缓解**：使用异步扫描 + 加载状态指示，后续可增加缓存。
- **[Symlink 检测失败]** → 某些情况下 symlink 检测可能不准确。**缓解**：在 UI 中标识检测到的模式，允许用户手动指定。

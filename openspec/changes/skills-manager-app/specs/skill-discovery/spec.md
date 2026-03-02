## ADDED Requirements

### Requirement: 扫描已安装的 Skills
系统 SHALL 扫描以下目录以发现已安装的 skills：
- 中心目录：`~/.agents/skills/`
- Cursor 目录：`~/.cursor/skills/`
- Claude Code 目录：`~/.claude/skills/`

每个包含 `SKILL.md` 文件的子目录 SHALL 被识别为一个有效的 skill。

#### Scenario: 发现 symlinks 模式下的 skills
- **WHEN** `~/.agents/skills/` 下存在 skill 目录，且编辑器 skills 目录中存在指向该 skill 的软链接
- **THEN** 系统识别该 skill 为 symlinks 模式安装，关联所有链接到它的编辑器

#### Scenario: 发现 copy 模式下的 skills
- **WHEN** 编辑器 skills 目录下存在非软链接的 skill 目录且包含 `SKILL.md`
- **THEN** 系统识别该 skill 为 copy 模式安装，归属到对应编辑器

#### Scenario: 目录不存在
- **WHEN** 某个扫描目录不存在
- **THEN** 系统 SHALL 跳过该目录而非报错，并在日志中记录

### Requirement: 解析 Skills 元信息
系统 SHALL 解析每个 skill 的 `SKILL.md` 文件，提取 YAML front matter 中的元信息，至少包括：
- `name`：skill 名称
- `description`：skill 描述
- `version`：版本号
- `metadata.author`：作者
- `license`：许可证

#### Scenario: 完整的 SKILL.md
- **WHEN** `SKILL.md` 包含完整的 YAML front matter
- **THEN** 系统成功解析所有字段并展示

#### Scenario: 缺少部分元信息字段
- **WHEN** `SKILL.md` 的 YAML front matter 缺少某些可选字段
- **THEN** 系统 SHALL 将缺失字段显示为"未知"或留空，不影响其他字段的展示

### Requirement: 去重合并 Skills 列表
系统 SHALL 对来自不同目录的同名 skill 进行去重合并，以中心目录（`~/.agents/skills/`）的 skill 为主记录，关联它在各编辑器中的链接状态。

#### Scenario: 同一 skill 在多个编辑器中安装
- **WHEN** `my-skill` 在 `~/.agents/skills/` 中存在，且被软链接到 Cursor 和 Claude Code
- **THEN** 系统显示为一条 skill 记录，标记其关联的编辑器为 Cursor 和 Claude Code

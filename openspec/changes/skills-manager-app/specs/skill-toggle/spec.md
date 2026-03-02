## ADDED Requirements

### Requirement: 启用 Skill（Symlinks 模式）
系统 SHALL 支持在 symlinks 模式下启用被禁用的 skill：
1. 将 skill 目录从 `~/.agents/.disabled-skills/<skill-name>/` 移回 `~/.agents/skills/<skill-name>/`
2. 重新创建到各关联编辑器 skills 目录的软链接

#### Scenario: 成功启用 symlinks 模式 skill
- **WHEN** 用户将禁用状态的 symlinks 模式 skill 的开关切换为启用
- **THEN** 系统将 skill 移回中心目录，重建所有关联编辑器的 symlinks，UI 显示启用状态

#### Scenario: 启用过程中 skill 目标位置已存在同名目录
- **WHEN** 启用操作发现目标位置已有同名目录
- **THEN** 系统 SHALL 提示用户存在冲突，并提供覆盖或取消选项

### Requirement: 禁用 Skill（Symlinks 模式）
系统 SHALL 支持在 symlinks 模式下禁用 skill：
1. 移除所有编辑器 skills 目录中指向该 skill 的软链接
2. 将 skill 目录从 `~/.agents/skills/<skill-name>/` 移动到 `~/.agents/.disabled-skills/<skill-name>/`

#### Scenario: 成功禁用 symlinks 模式 skill
- **WHEN** 用户将启用状态的 symlinks 模式 skill 的开关切换为禁用
- **THEN** 系统移除所有相关 symlinks，将 skill 移到隐藏目录，UI 显示禁用状态

### Requirement: 启用 Skill（Copy 模式）
系统 SHALL 支持在 copy 模式下启用被禁用的 skill：
将 skill 目录从 `<editor-skills-dir>/.disabled/<skill-name>/` 移回 `<editor-skills-dir>/<skill-name>/`

#### Scenario: 成功启用 copy 模式 skill
- **WHEN** 用户将禁用状态的 copy 模式 skill 的开关切换为启用
- **THEN** 系统将 skill 移回编辑器 skills 目录，UI 显示启用状态

### Requirement: 禁用 Skill（Copy 模式）
系统 SHALL 支持在 copy 模式下禁用 skill：
将 skill 目录从 `<editor-skills-dir>/<skill-name>/` 移动到 `<editor-skills-dir>/.disabled/<skill-name>/`

#### Scenario: 成功禁用 copy 模式 skill
- **WHEN** 用户将启用状态的 copy 模式 skill 的开关切换为禁用
- **THEN** 系统将 skill 移到编辑器 skills 隐藏子目录，UI 显示禁用状态

### Requirement: 操作并发保护
系统 SHALL 对每个 skill 的切换操作加锁，防止同一 skill 的并发操作。

#### Scenario: 操作进行中禁止重复操作
- **WHEN** 某个 skill 的启用/禁用操作正在进行中
- **THEN** 该 skill 的开关按钮 SHALL 显示为加载状态（不可点击），直到操作完成

#### Scenario: 操作失败回滚
- **WHEN** 启用或禁用操作中途失败（如权限不足、磁盘空间不足）
- **THEN** 系统 SHALL 尝试回滚到操作前状态，并在 UI 中展示错误信息

### Requirement: 禁用目录自动创建
系统 SHALL 在首次禁用操作时自动创建所需的隐藏目录（`~/.agents/.disabled-skills/` 或 `<editor-skills-dir>/.disabled/`）。

#### Scenario: 隐藏目录不存在时自动创建
- **WHEN** 用户首次禁用一个 skill 且隐藏目录尚不存在
- **THEN** 系统自动创建隐藏目录后继续执行禁用操作

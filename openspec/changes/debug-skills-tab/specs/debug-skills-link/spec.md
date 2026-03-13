## ADDED Requirements

### Requirement: 扫描外部文件夹中的 Skills

系统 SHALL 扫描用户选择的文件夹，识别其中所有包含 SKILL.md 的直接子目录作为有效 skills，解析其 front matter 获取名称和描述信息。

#### Scenario: 扫描包含有效 skills 的文件夹
- **WHEN** 用户选择包含多个 skill 子目录的文件夹
- **THEN** 系统返回所有包含 SKILL.md 的直接子目录列表
- **THEN** 每个 skill 包含解析后的名称和描述

#### Scenario: 扫描空文件夹或无效文件夹
- **WHEN** 用户选择的文件夹不包含任何有效 skill 子目录
- **THEN** 系统返回空列表

### Requirement: 创建调试软链到全局目录

系统 SHALL 将用户选中的外部 skill 目录软链到 `~/.agents/skills/<dir_name>`，使其成为 center skill 被编辑器发现。

#### Scenario: 成功创建调试软链
- **WHEN** 用户确认导入某个 skill 且全局目录中无同名项
- **THEN** 系统在 `~/.agents/skills/` 下创建指向外部 skill 目录的符号链接
- **THEN** 持久化记录该调试信息（来源文件夹路径、skill 目录名、链接时间）

#### Scenario: 导入时发现同名冲突——用户选择覆盖
- **WHEN** `~/.agents/skills/<dir_name>` 已存在且用户选择覆盖
- **THEN** 系统删除已有的目录或软链
- **THEN** 创建新的软链指向外部 skill 目录
- **THEN** 更新持久化记录

#### Scenario: 导入时发现同名冲突——用户选择跳过
- **WHEN** `~/.agents/skills/<dir_name>` 已存在且用户选择不覆盖
- **THEN** 系统跳过该 skill，不创建软链

### Requirement: 持久化调试 Skills 记录

系统 SHALL 在 Tauri `app_data_dir` 下维护 `debug-skills.json` 文件，记录所有通过调试功能导入的文件夹及其链接的 skills 信息。

#### Scenario: 首次添加调试文件夹
- **WHEN** 用户首次通过调试功能导入 skills
- **THEN** 系统创建 `debug-skills.json` 并写入记录

#### Scenario: 追加导入新文件夹
- **WHEN** 用户从新的文件夹导入 skills
- **THEN** 系统在现有记录中追加新的文件夹条目

#### Scenario: 从同一文件夹追加导入
- **WHEN** 用户再次从已记录的文件夹导入新的 skills
- **THEN** 系统在该文件夹条目下追加新的 skill 记录

### Requirement: 冲突与异常状态检测

系统 SHALL 在每次扫描 skills 时检测调试 skills 的状态一致性，标记异常状态。

#### Scenario: 正常状态——软链完好
- **WHEN** `~/.agents/skills/<dir_name>` 是软链且指向记录中的原始路径
- **THEN** 该 skill 标记为正常调试状态

#### Scenario: 异常状态——软链被覆盖为物理目录
- **WHEN** `~/.agents/skills/<dir_name>` 存在但不是软链
- **THEN** 该 skill 标记为"异常"状态
- **THEN** UI 展示警告提示

#### Scenario: 异常状态——软链不存在
- **WHEN** `~/.agents/skills/<dir_name>` 不存在（也不在 `.disabled-skills` 中）
- **THEN** 该 skill 标记为"异常"状态

#### Scenario: 异常状态——软链指向不同路径
- **WHEN** `~/.agents/skills/<dir_name>` 是软链但指向的目标与记录中不一致
- **THEN** 该 skill 标记为"异常"状态

### Requirement: 安全卸载调试 Skill

卸载调试 skill 时系统 SHALL 仅删除 `~/.agents/skills/` 中的软链和编辑器目录中的软链，并移除持久化记录，不得删除用户的源文件目录。

#### Scenario: 正常状态下卸载调试 skill
- **WHEN** 用户对正常状态的调试 skill 点击卸载
- **THEN** 系统删除 `~/.agents/skills/<dir_name>` 软链
- **THEN** 系统删除所有编辑器目录中指向该 center skill 的软链
- **THEN** 系统从 `debug-skills.json` 中移除该 skill 记录
- **THEN** 源文件目录保持不变

#### Scenario: 异常状态下卸载调试 skill
- **WHEN** 用户对异常状态的调试 skill 点击卸载
- **THEN** 系统仅从 `debug-skills.json` 中移除记录
- **THEN** 不执行任何文件系统删除操作（因为软链已不存在或已被替换）

### Requirement: 调试 Skill 的启用/禁用沿用现有逻辑

调试 skills 被软链到全局后成为 center skills，其启用/禁用 SHALL 沿用现有的 center skill 逻辑（移动到 `~/.agents/.disabled-skills/`）。

#### Scenario: 禁用调试 skill
- **WHEN** 用户关闭调试 skill 的开关
- **THEN** 系统将 `~/.agents/skills/<dir_name>` 移动到 `~/.agents/.disabled-skills/<dir_name>`
- **THEN** 移除所有编辑器目录中的软链

#### Scenario: 重新启用调试 skill
- **WHEN** 用户打开已禁用的调试 skill 的开关
- **THEN** 系统将 `~/.agents/.disabled-skills/<dir_name>` 移回 `~/.agents/skills/<dir_name>`
- **THEN** 在选中的编辑器目录中创建软链

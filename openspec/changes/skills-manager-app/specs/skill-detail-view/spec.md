## ADDED Requirements

### Requirement: 展示 Skill 基础信息
系统 SHALL 在右侧详情面板展示选中 skill 的基础信息：
- Skill 名称
- 版本号
- 作者
- 许可证
- 安装模式标签（Tag）
- 关联的编辑器列表
- 启用/禁用状态

#### Scenario: 展示完整的 skill 信息
- **WHEN** 用户在左侧列表中点击选中一个 skill
- **THEN** 右侧面板展示该 skill 的所有基础信息

#### Scenario: 未选中任何 skill
- **WHEN** 没有选中任何 skill
- **THEN** 右侧面板显示占位提示信息（如"请选择一个 Skill 查看详情"）

### Requirement: 展示安装模式标签
系统 SHALL 在详情面板的 skill 名称旁以 Tag（标签）形式展示该 skill 的安装模式。

#### Scenario: Symlinks 模式 skill
- **WHEN** 选中的 skill 为 symlinks 模式安装
- **THEN** 显示 "Symlink" 标签，使用区分色标识（如蓝色调）

#### Scenario: Copy 模式 skill
- **WHEN** 选中的 skill 为 copy 模式安装
- **THEN** 显示 "Copy" 标签，使用区分色标识（如绿色调）

### Requirement: 渲染 Skill 指令内容
系统 SHALL 解析 `SKILL.md` 文件中 YAML front matter 之后的 Markdown 正文内容，并以格式化的 Markdown 形式渲染展示。

#### Scenario: 渲染含代码块的 Markdown 内容
- **WHEN** `SKILL.md` 正文中包含 Markdown 代码块
- **THEN** 系统以语法高亮的方式渲染代码块内容

#### Scenario: SKILL.md 无正文内容
- **WHEN** `SKILL.md` 只有 YAML front matter 而无正文
- **THEN** 系统显示"该 Skill 没有详细指令说明"

### Requirement: 查看 Skill 文件路径
系统 SHALL 展示 skill 在文件系统中的实际路径，并提供"在文件管理器中打开"功能。

#### Scenario: 打开 skill 所在目录
- **WHEN** 用户点击"在文件管理器中打开"按钮
- **THEN** 系统调用操作系统的文件管理器打开该 skill 目录

### Requirement: 详情面板中的 Skill 开关
详情面板 SHALL 包含启用/禁用开关，功能与列表中的开关一致。

#### Scenario: 在详情面板切换 skill 状态
- **WHEN** 用户在详情面板中点击开关
- **THEN** 执行启用或禁用操作，列表中的状态同步更新

### Requirement: 编辑 Skill 内容
系统 SHALL 在详情面板提供"编辑"开关，允许用户直接编辑 `SKILL.md` 文件的完整内容（包括 YAML front matter 和 Markdown 正文）。

#### Scenario: 切换到编辑模式
- **WHEN** 用户点击"编辑"开关
- **THEN** Markdown 渲染区切换为文本编辑器（等宽字体 textarea），显示 `SKILL.md` 原始内容

#### Scenario: 保存编辑内容
- **WHEN** 用户在编辑模式下修改内容并点击"保存"按钮
- **THEN** 系统将修改后的内容写回 `SKILL.md` 文件，并切换回渲染模式展示更新后的内容

#### Scenario: 取消编辑
- **WHEN** 用户在编辑模式下点击"取消"按钮
- **THEN** 放弃所有修改，恢复到编辑前的内容，切换回渲染模式

#### Scenario: 编辑 Symlinks 模式 skill 的警告
- **WHEN** 用户对 symlinks 模式的 skill 进入编辑模式
- **THEN** 系统 SHALL 显示警告提示："此 Skill 为 Symlink 模式，编辑将影响所有链接到此 Skill 的编辑器"

#### Scenario: 保存失败
- **WHEN** 文件写入失败（如权限不足）
- **THEN** 系统显示错误提示信息，保留编辑器中的内容不丢失

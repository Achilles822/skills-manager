## ADDED Requirements

### Requirement: 可扩展编辑器注册表
系统 SHALL 使用注册表模式管理编辑器定义。每个编辑器通过统一的 `EditorDefinition` trait 接口注册，包含编辑器 ID、显示名称、图标标识、skills 目录路径和配置目录路径。新增编辑器支持只需添加一个新的 trait 实现并注册到注册表。

#### Scenario: 内置编辑器注册
- **WHEN** 应用启动
- **THEN** 系统自动注册 Cursor 和 Claude Code 两个内置编辑器定义

#### Scenario: 扩展新编辑器
- **WHEN** 开发者需要添加新编辑器支持（如 Windsurf）
- **THEN** 只需创建一个实现 `EditorDefinition` trait 的新 struct 并注册到注册表，核心扫描和切换逻辑无需修改

### Requirement: 检测已安装的编辑器
系统 SHALL 遍历注册表中的所有编辑器定义，通过检查对应的配置目录是否存在来判断编辑器是否已安装。

#### Scenario: 编辑器已安装
- **WHEN** 对应编辑器的配置目录存在
- **THEN** 系统在编辑器筛选列表中显示该编辑器为可选项

#### Scenario: 编辑器未安装
- **WHEN** 对应编辑器的配置目录不存在
- **THEN** 系统在编辑器筛选列表中隐藏该编辑器，或显示为灰色不可选状态

### Requirement: 跨平台路径解析
系统 SHALL 根据操作系统自动解析正确的 skills 目录路径：
- Windows：使用 `%USERPROFILE%` 作为用户主目录
- macOS：使用 `$HOME` 作为用户主目录

#### Scenario: Windows 系统路径解析
- **WHEN** 应用运行在 Windows 系统上
- **THEN** Cursor skills 路径解析为 `C:\Users\<username>\.cursor\skills\`

#### Scenario: macOS 系统路径解析
- **WHEN** 应用运行在 macOS 系统上
- **THEN** Cursor skills 路径解析为 `/Users/<username>/.cursor/skills/`

### Requirement: 检测 Symlink 支持
系统 SHALL 在 Windows 上检测当前用户是否拥有创建 symlinks 的权限。

#### Scenario: Windows 有 symlink 权限
- **WHEN** 系统为 Windows 且用户具有创建 symlinks 的权限（管理员或开发者模式）
- **THEN** 系统正常支持 symlinks 模式的所有操作

#### Scenario: Windows 无 symlink 权限
- **WHEN** 系统为 Windows 且用户无创建 symlinks 的权限
- **THEN** 系统 SHALL 在 UI 中提示用户启用开发者模式，并对 symlinks 模式的 skill 禁用切换操作或降级为 copy 模式

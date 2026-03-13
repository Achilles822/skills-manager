## ADDED Requirements

### Requirement: 调试 Skills 在列表中显示"本地调试"标签

所有 skill 列表卡片（管理 Tab 和调试 Tab）中，通过调试功能导入的 skills SHALL 显示"本地调试"标签以区分普通 skills。

#### Scenario: 管理 Tab 中显示调试标签
- **WHEN** 管理 Tab 的 skills 列表中包含调试导入的 skill
- **THEN** 该 skill 的卡片上显示"本地调试"标签（使用区别于现有标签的颜色）

#### Scenario: 调试 Tab 中显示调试标签
- **WHEN** 调试 Tab 的 skills 列表中展示调试 skills
- **THEN** 每个 skill 的卡片上均显示"本地调试"标签

#### Scenario: 异常状态显示额外警告标签
- **WHEN** 调试 skill 处于异常状态（软链被覆盖、不存在、或指向不一致）
- **THEN** 除"本地调试"标签外，还显示"异常"警告标签（使用警告色）

### Requirement: 详情页显示实际路径

Skill 详情页 SHALL 在现有"路径"字段下方新增"实际路径"字段，显示软链的目标路径（即用户开发 skill 的外部文件夹路径）。此字段仅对调试 skills 显示。

#### Scenario: 查看正常调试 skill 的详情
- **WHEN** 用户选中一个正常状态的调试 skill
- **THEN** 详情页在"路径"字段下方显示"实际路径"字段
- **THEN** "实际路径"值为调试记录中记录的外部文件夹的完整路径

#### Scenario: 查看非调试 skill 的详情
- **WHEN** 用户选中一个非调试 skill
- **THEN** 详情页不显示"实际路径"字段

### Requirement: 调试 Skills 的 Skill 数据结构扩展

Skill 数据结构 SHALL 新增字段以支持调试标识：`is_debug`（布尔值，是否为调试 skill）、`debug_status`（调试状态：normal/abnormal/null）、`debug_source_path`（调试源路径，即软链原始目标路径）。

#### Scenario: 扫描结果包含调试标识信息
- **WHEN** 后端扫描 skills 并返回列表
- **THEN** 每个与 `debug-skills.json` 记录匹配的 skill 的 `is_debug` 为 true
- **THEN** `debug_status` 根据文件系统一致性检查结果设置为 "normal" 或 "abnormal"
- **THEN** `debug_source_path` 设置为记录中的外部文件夹路径

#### Scenario: 非调试 skill 的字段默认值
- **WHEN** skill 不在调试记录中
- **THEN** `is_debug` 为 false，`debug_status` 为 null，`debug_source_path` 为 null

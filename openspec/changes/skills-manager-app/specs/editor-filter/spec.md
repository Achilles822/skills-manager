## ADDED Requirements

### Requirement: 编辑器多选筛选
系统 SHALL 提供编辑器多选筛选组件，支持以下选项：
- 全部（默认选中）
- Cursor
- Claude Code

筛选组件位于左侧面板顶部。

#### Scenario: 默认显示全部
- **WHEN** 应用启动或用户选择"全部"
- **THEN** skills 列表展示所有编辑器中已安装的 skills

#### Scenario: 单选某个编辑器
- **WHEN** 用户选择"Cursor"
- **THEN** skills 列表仅展示安装到 Cursor 中的 skills（包括 symlinks 和 copy 模式）

#### Scenario: 多选编辑器
- **WHEN** 用户同时选择"Cursor"和"Claude Code"
- **THEN** skills 列表展示安装到 Cursor 或 Claude Code 中的所有 skills（并集）

### Requirement: 筛选结果实时更新
筛选操作 SHALL 立即（无需点击确认按钮）更新 skills 列表。

#### Scenario: 切换筛选条件
- **WHEN** 用户改变编辑器筛选选项
- **THEN** skills 列表在 100ms 内响应更新，无需手动刷新

### Requirement: 列表显示 Skill 摘要
每个 skill 列表条目 SHALL 显示以下摘要信息：
- Skill 名称
- 简短描述（截取 description 的前 80 字符）
- 关联编辑器图标
- 启用/禁用开关

#### Scenario: 展示 skill 列表条目
- **WHEN** 用户查看 skills 列表
- **THEN** 每个条目以新拟态卡片样式展示名称、简述、编辑器图标和状态开关

#### Scenario: 空列表状态
- **WHEN** 筛选结果为空或没有安装任何 skills
- **THEN** 显示空状态提示（如"未发现已安装的 Skills"）

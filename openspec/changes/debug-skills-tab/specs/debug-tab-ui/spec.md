## ADDED Requirements

### Requirement: Tab 切换布局替换品牌文字

系统 SHALL 将左侧面板顶部的品牌文字 "Skills Manager" 替换为 Tab 切换组件，包含"管理"和"调试"两个 Tab。点击品牌文字打开关于弹窗的功能 SHALL 移至其他合适位置（如设置弹窗中或 Tab 区域的辅助元素）。

#### Scenario: 默认显示管理 Tab
- **WHEN** 应用启动
- **THEN** 默认选中"管理"Tab，展示现有的 skills 管理界面（编辑器筛选、状态筛选、skills 列表、详情面板）

#### Scenario: 切换到调试 Tab
- **WHEN** 用户点击"调试"Tab
- **THEN** 左侧面板和右侧详情面板切换为调试 skills 管理界面
- **THEN** 顶部刷新和设置按钮保持可用

#### Scenario: 切换回管理 Tab
- **WHEN** 用户从调试 Tab 点击"管理"Tab
- **THEN** 界面切换回标准 skills 管理视图

### Requirement: 调试 Tab 包含添加文件夹功能

调试 Tab 的左侧面板 SHALL 在筛选区域上方或列表顶部提供"添加文件夹"按钮，点击后打开系统原生文件夹选择对话框。

#### Scenario: 点击添加文件夹按钮
- **WHEN** 用户在调试 Tab 中点击"添加文件夹"按钮
- **THEN** 系统打开原生文件夹选择对话框
- **THEN** 用户选择文件夹后，系统扫描该文件夹下的 skills 子目录

#### Scenario: 取消文件夹选择
- **WHEN** 用户在文件夹选择对话框中点击取消
- **THEN** 不执行任何操作，返回调试 Tab 界面

### Requirement: Skills 选择弹窗

选择文件夹后系统 SHALL 展示一个弹窗，列出扫描到的 skills（包含 SKILL.md 的子目录），用户可勾选要链接的 skills，然后确认导入。

#### Scenario: 文件夹中发现多个 skills
- **WHEN** 选择的文件夹包含多个含 SKILL.md 的子目录
- **THEN** 弹窗列出所有发现的 skills，显示名称和描述
- **THEN** 用户可通过 checkbox 选择要导入的 skills
- **THEN** 用户点击确认后，选中的 skills 被软链到全局

#### Scenario: 文件夹中无有效 skills
- **WHEN** 选择的文件夹下没有包含 SKILL.md 的子目录
- **THEN** 系统提示"未发现有效的 Skills"

#### Scenario: 部分 skills 存在同名冲突
- **WHEN** 待导入的 skill 与全局已有 skill 同名
- **THEN** 该 skill 在列表中标记冲突状态
- **THEN** 用户确认导入时，对冲突的 skills 提示是否覆盖

### Requirement: 调试 Tab 复用现有 UI 组件

调试 Tab SHALL 复用现有的 `EditorFilter`、`StatusFilter`、`SkillList`、`SkillDetail` 组件展示和管理调试 skills。调试 Tab 的 skills 列表 SHALL 仅显示通过调试功能导入的 skills（基于持久化记录过滤）。

#### Scenario: 调试 Tab 展示调试 skills 列表
- **WHEN** 用户切换到调试 Tab
- **THEN** skills 列表仅显示通过调试功能导入的 skills
- **THEN** 编辑器筛选和状态筛选功能正常工作

#### Scenario: 调试 Tab 中查看 skill 详情
- **WHEN** 用户在调试 Tab 中选择一个 skill
- **THEN** 右侧详情面板展示该 skill 的完整信息，与管理 Tab 一致

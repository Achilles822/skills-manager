## ADDED Requirements

### Requirement: 新拟态设计风格
系统 SHALL 使用新拟态（Neumorphism）设计风格，特征为：
- 柔和的外阴影和内阴影组合
- 浅色背景（`#e0e5ec` 或类似色调）
- 低对比度、柔和的光影效果
- 圆角元素
- 清新、有质感的视觉感受

#### Scenario: 基础视觉风格
- **WHEN** 用户打开应用
- **THEN** 整体界面呈现柔和的新拟态风格，包含光影层次感

### Requirement: 左右分栏布局
系统 SHALL 采用左右分栏布局，左侧面板与右侧面板的空间比例为 3:7。

#### Scenario: 正常窗口大小
- **WHEN** 应用窗口宽度 >= 900px
- **THEN** 左右面板按 3:7 比例显示

#### Scenario: 窗口最小宽度
- **WHEN** 应用窗口宽度缩小到最小尺寸
- **THEN** 布局 SHALL 保持 3:7 比例，内容可能出现滚动条但不会重叠

### Requirement: 新拟态开关组件
系统 SHALL 提供新拟态风格的 toggle switch 组件，用于 skills 启用/禁用操作。

#### Scenario: 开关状态切换
- **WHEN** 用户点击开关
- **THEN** 开关以平滑动画（200-300ms）从一个状态过渡到另一个状态，展示内凹/外凸的视觉效果

#### Scenario: 开关加载状态
- **WHEN** 开关对应的操作正在处理中
- **THEN** 开关显示加载动画，不可点击

### Requirement: 新拟态选择器组件
系统 SHALL 提供新拟态风格的多选选择器组件，用于编辑器筛选。

#### Scenario: 选项选中效果
- **WHEN** 用户选中一个筛选选项
- **THEN** 选中的选项显示内凹（pressed）效果，未选中的显示外凸效果

### Requirement: 新拟态卡片组件
系统 SHALL 提供新拟态风格的卡片组件，用于 skills 列表条目。

#### Scenario: 卡片悬停效果
- **WHEN** 用户鼠标悬停在 skill 卡片上
- **THEN** 卡片展示微妙的阴影变化效果

#### Scenario: 卡片选中效果
- **WHEN** 用户点击选中一个 skill 卡片
- **THEN** 卡片显示内凹的选中状态，与未选中卡片有明显区分

### Requirement: 滚动条样式
系统 SHALL 对滚动条进行自定义样式处理，使其与新拟态风格协调。

#### Scenario: 列表内容溢出
- **WHEN** skills 列表内容超出面板高度
- **THEN** 显示自定义样式的滚动条，风格与整体 UI 一致

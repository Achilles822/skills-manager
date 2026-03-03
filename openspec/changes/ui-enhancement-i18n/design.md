## Architecture

本次变更涉及三个独立但可并行推进的功能模块，均以前端改动为主，辅以少量 Rust 后端扩展。

### 整体架构变化

```
src/
├── i18n/                          # [新增] 国际化模块
│   ├── index.ts                   # vue-i18n 初始化配置
│   ├── zh-CN.ts                   # 中文简体翻译
│   └── en.ts                      # 英文翻译
├── components/
│   ├── SkillDetail.vue            # [修改] 重构为目录树 + 文件查看分栏
│   ├── SkillFileTree.vue          # [新增] 目录树组件
│   ├── FileViewer.vue             # [新增] 文件内容查看/编辑组件
│   ├── SettingsDialog.vue         # [新增] 设置弹窗（替代 AboutDialog 中的设置逻辑）
│   └── ... (其他组件加 i18n 翻译)
├── composables/
│   ├── useSkillFiles.ts           # [新增] skill 文件树数据管理
│   └── useI18nPreference.ts       # [新增] 语言偏好管理
src-tauri/src/
│   ├── commands.rs                # [修改] 新增文件相关 Command
│   └── ...
```

---

## Feature 1: Skill 目录浏览器

### 技术方案

**后端扩展（Rust）**：

新增 3 个 Tauri Command：

1. `list_skill_files(skill_dir: String) -> Vec<FileEntry>`
   - 递归扫描 skill 目录，返回文件树结构
   - `FileEntry { name, path, is_dir, children? }`
   - 忽略隐藏文件（`.` 开头）和 `node_modules` 等常见忽略目录
   
2. `read_file_content(file_path: String) -> FileContent`
   - 读取指定文件内容
   - `FileContent { content: String, is_binary: bool }`
   - 对于二进制文件标记 `is_binary: true` 并不返回内容
   
3. `save_file_content(file_path: String, content: String) -> ()`
   - 保存文件内容（复用现有 `save_skill_content` 的原子写入逻辑）

**前端组件设计**：

- **`SkillFileTree.vue`**：递归渲染目录树
  - 使用缩进 + 展开/收起箭头表示层级
  - 文件图标根据扩展名区分类型（markdown、script、image 等）
  - 选中文件高亮显示
  - SKILL.md 默认选中

- **`FileViewer.vue`**：文件内容查看/编辑
  - 查看模式：Markdown 文件使用 markdown-it 渲染，脚本文件使用 highlight.js 语法高亮
  - 编辑模式：使用 textarea（与现有编辑体验一致），脚本文件编辑也带高亮
  - 支持的高亮语言：JavaScript、TypeScript、Python、Shell、JSON、YAML、Markdown

- **`SkillDetail.vue` 重构**：
  - 右侧面板内部采用左右分栏：文件树（~200px 固定宽度）+ 文件内容区（弹性）
  - header（名称、标签、工具栏）保持不变
  - 编辑模式变为按文件编辑（不再固定编辑 SKILL.md）
  - 文件树分栏支持折叠/展开按钮，用户可手动收起目录树以获得更大的内容区空间

**语法高亮方案选型**：

选择 **highlight.js**：
- 体积较小，适合桌面应用
- 支持常见语言足够
- 可按需加载语言包
- 与 markdown-it 可集成（markdown-it-highlightjs 插件）

### 按钮顺序调整

在 `SkillDetail.vue`（或重构后的 `FileViewer.vue`）中，编辑模式的按钮区域：
```html
<!-- 调整前 -->
<NeuButton @click="handleSave">保存</NeuButton>
<NeuButton @click="cancelEdit">取消</NeuButton>

<!-- 调整后 -->
<NeuButton @click="cancelEdit">{{ t('cancel') }}</NeuButton>
<NeuButton @click="handleSave">{{ t('save') }}</NeuButton>
```

---

## Feature 2: 多语言（i18n）支持

### 技术方案

**框架选择**：`vue-i18n` v10（Vue 3 Composition API 兼容）

**翻译文件组织**：

```typescript
// src/i18n/zh-CN.ts
export default {
  app: {
    brand: 'Skills Manager',
  },
  skill: {
    selectPrompt: '选择一个 Skill 查看详情',
    edit: '编辑',
    save: '保存',
    cancel: '取消',
    version: '版本',
    author: '作者',
    license: '许可证',
    editors: '编辑器',
    path: '路径',
    uninstall: '卸载',
    confirmUninstall: '确认卸载',
    uninstallMessage: '确定要卸载 「{name}」 吗？此操作将永久删除该 Skill 的所有文件，无法撤销。',
    symlinkWarning: '此 Skill 为 Symlink 模式，编辑将影响所有链接到此 Skill 的编辑器。',
    loading: '加载中…',
    openFolder: '打开所在目录',
    // ...
  },
  filter: {
    all: '全部',
    enabled: '已启用',
    disabled: '已禁用',
    refresh: '刷新',
  },
  settings: {
    title: '设置',
    displayLanguage: '界面语言',
    save: '保存',
    close: '关闭',
  },
  about: {
    title: 'Skills Manager',
    description: 'AI 编辑器 Skills 可视化管理工具',
    author: '作者',
    license: '许可证',
    close: '关闭',
  },
}
```

**语言偏好持久化**：

使用 `localStorage`（简单可靠，无需额外 Tauri 插件）：
- key: `skills-manager-locale`
- 值: `'zh-CN'` | `'en'`
- 启动时读取，默认为 `'zh-CN'`

### 设置弹窗设计

**`SettingsDialog.vue`**：
- 将现有的齿轮图标按钮从打开"关于"弹窗改为打开"设置"弹窗
- 弹窗布局：
  - 顶部：`界面语言 (Display Language)` 下拉选择器
  - 中间区域：预留未来设置项扩展
  - 底部：`保存` 按钮
- "关于" 信息可移至设置弹窗内的子区域，或保留为独立入口

**设计决策**：保留"关于"弹窗作为独立入口（通过品牌名点击触发），设置弹窗由齿轮按钮触发。这样职责更清晰。

---

## UI/UX 考量

- 目录树宽度适中（180-220px），不抢占文件内容区空间
- 目录树智能隐藏：当 skill 只有 SKILL.md 没有子目录时，自动隐藏目录树
- 目录树手动折叠：显示目录树时，提供折叠/展开按钮，用户可随时收起/展开目录树
- 文件切换时保持编辑状态提示（若当前文件有未保存修改，切换时提示）
- 语言切换即时生效预览，但需点击保存才真正持久化
- 所有新拟态样式保持一致

<p align="center">
  <img src="src-tauri/icons/icon-readme.png" width="128" height="128" alt="Skills Manager">
</p>

<h1 align="center">Skills Manager</h1>

<p align="center">
  A cross-platform desktop app for managing AI editor skills.
  <br/>
  跨平台桌面应用，统一管理 AI 编辑器的 Skills。
</p>

<p align="center">
  <a href="#features">Features</a> |
  <a href="#installation">Installation</a> |
  <a href="#development">Development</a> |
  <a href="./README.zh-CN.md">中文文档</a>
</p>

---

## Features

- 🔍 **Multi-editor support** — Auto-detect and manage skills for Cursor and Claude Code.
- 📦 **Centralized skill center** — One copy in `~/.agents/skills/`, symlinked to all editors.
- 🔀 **One-click toggle** — Enable / disable skills instantly, restore anytime.
- 📝 **Detail & edit** — View Markdown-rendered docs and edit skill content in-place.
- 🗑️ **Uninstall** — Remove skills with automatic cross-editor cleanup.
- 🎨 **Neumorphic UI** — Soft, modern design built with custom Vue components.
- 💻 **Cross-platform** — Native on macOS, Windows, and Linux via Tauri v2.

## Screenshots

<p align="center">
  <img src="screenshots/1.png" width="700" alt="Skill list view">
</p>

<p align="center">
  <img src="screenshots/2.png" width="700" alt="Skill detail view">
</p>

## Installation

Download the latest release for your platform from the [Releases](https://github.com/nicepkg/skills-manager/releases) page.

| Platform | File |
|----------|------|
| macOS (Apple Silicon) | `Skills Manager_x.x.x_aarch64.dmg` |
| macOS (Intel) | `Skills Manager_x.x.x_x64.dmg` |
| Windows | `Skills Manager_x.x.x_x64-setup.exe` |
| Linux (Debian/Ubuntu) | `Skills Manager_x.x.x_amd64.deb` |
| Linux (AppImage) | `Skills Manager_x.x.x_amd64.AppImage` |

## Development

### Prerequisites

- [Node.js](https://nodejs.org/) >= 20
- [pnpm](https://pnpm.io/) >= 10
- [Rust](https://www.rust-lang.org/tools/install) >= 1.77
- Platform-specific dependencies:
  - **macOS**: Xcode Command Line Tools (`xcode-select --install`)
  - **Linux**: `libwebkit2gtk-4.1-dev`, `libappindicator3-dev`, `librsvg2-dev`, `patchelf`
  - **Windows**: [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/), WebView2

### Setup

```bash
# Clone the repository
git clone https://github.com/nicepkg/skills-manager.git
cd skills-manager

# Install frontend dependencies
pnpm install

# Start development server
pnpm tauri dev

# Build for production
pnpm tauri build
```

The production build outputs to `src-tauri/target/release/bundle/`.

### Project Structure

```
skills-manager/
├── src/                    # Vue 3 frontend
│   ├── components/         # UI components (Neumorphic design system)
│   ├── composables/        # Vue composables (useSkills, useSkillToggle, etc.)
│   ├── types/              # TypeScript type definitions
│   └── styles/             # Global styles
├── src-tauri/              # Rust backend (Tauri v2)
│   └── src/
│       ├── lib.rs          # App entry & command registration
│       ├── commands.rs     # Tauri command handlers
│       ├── editor.rs       # Editor detection & registry
│       ├── skill.rs        # Skill discovery & SKILL.md parsing
│       ├── toggle.rs       # Enable/disable/uninstall logic
│       └── platform.rs     # Platform-specific utilities
├── scripts/                # Icon generation scripts
└── package.json
```

### Tech Stack

| Layer | Technology |
|-------|-----------|
| Framework | [Tauri v2](https://v2.tauri.app/) |
| Frontend | [Vue 3](https://vuejs.org/) + TypeScript |
| Build | [Vite 6](https://vite.dev/) |
| Backend | Rust |
| Markdown | [markdown-it](https://github.com/markdown-it/markdown-it) |

## License

[MIT](./LICENSE)

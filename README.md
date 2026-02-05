# TaskFlow

> A hybrid task management application that combines the simplicity of physical notebooks with the power of digital tools.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Status: MVP Development](https://img.shields.io/badge/Status-MVP%20Development-yellow.svg)]()

---

## 🎯 Vision

TaskFlow is designed for developers and knowledge workers who have abandoned complex task management systems. It enforces simplicity through constraints (one NOW task, weekly reviews) while providing just enough digital advantages (sync, reminders, offline-first).

**Core Principle:** Digital task systems fail not because they lack features, but because they have too many.

---

## ✨ Features (MVP)

- ⚡ **Quick Capture** - Global hotkey (`Cmd/Ctrl+Shift+Space`) for instant task entry
- 📊 **Persistent Dashboard** - Always-visible widget showing NOW/NEXT/WAITING tasks
- 🎯 **One NOW Task Rule** - Enforced focus on single active task
- 📅 **Weekly Review Enforcement** - Mandatory task reconciliation prevents system rot
- 🔄 **Git-Based Sync** - Own your data, sync via your infrastructure
- 🌐 **Offline-First** - Works without network, syncs when available
- 🗣️ **Natural Language Parsing** - "Email John tomorrow at 2pm" → scheduled task
- ⌨️ **Keyboard-First** - Complete workflows without touching mouse
- 🔒 **Privacy-Respecting** - No telemetry, local SQLite database
- 📱 **Cross-Platform** - Desktop (Windows/macOS/Linux) in MVP, mobile in Phase 2

---

## 🚀 Quick Start

### Prerequisites

- **Windows 10+** / **macOS 12+** / **Ubuntu 20.04+**
- **Node.js 18+** ([Download](https://nodejs.org/))
- **Rust 1.70+** ([Install](https://rustup.rs/))
- **Git** ([Install](https://git-scm.com/))

### Installation (Development)

```bash
# Clone repository
git clone https://github.com/brucembudi/kaizen-flow.git
cd kaizen-flow

# Install dependencies
npm install

# Run development server
npm run tauri dev
```

### First Run

1. App launches with empty dashboard
2. Press `Ctrl+Shift+Space` to capture your first task
3. Type: "Review project documentation"
4. Press `Enter` to save
5. Task appears in NEXT section
6. Click task to move to NOW

---

## 📁 Project Structure

```
kaizen-flow/
├── src/                    # React frontend (TypeScript)
│   ├── components/         # UI components
│   │   ├── Dashboard/      # Main dashboard view
│   │   ├── QuickCapture/   # Hotkey overlay
│   │   └── Review/         # Weekly review flow
│   ├── store/              # Zustand state management
│   ├── api/                # Tauri command wrappers
│   └── App.tsx             # Root component
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── commands/       # Tauri command handlers
│   │   ├── db/             # SQLite database layer
│   │   ├── sync/           # Git synchronization
│   │   ├── nlp/            # Natural language parser
│   │   └── system/         # OS integration (hotkeys, tray)
│   └── Cargo.toml
├── Project Plan/           # Planning documents
│   ├── 00. Project Plan.md
│   ├── 01. PRD.md
│   └── 02. Tecnical Spec.md
└── README.md
```

---

## 🛠️ Development

### Running Tests

```bash
# Rust backend tests
cd src-tauri
cargo test

# Frontend tests
npm run test

# Run all tests
npm run test:all
```

### Building for Production

```bash
# Build desktop app
npm run tauri build

# Output:
# - Windows: src-tauri/target/release/bundle/msi/
# - macOS: src-tauri/target/release/bundle/dmg/
# - Linux: src-tauri/target/release/bundle/appimage/
```

### Code Quality

```bash
# Lint
npm run lint              # TypeScript/React
cargo clippy              # Rust

# Format
npm run format            # Prettier
cargo fmt                 # Rust
```

---

## 📖 Documentation

- **[Project Plan](Project%20Plan/00.%20Project%20Plan.md)** - Vision, user stories, wireframes
- **[PRD](Project%20Plan/01.%20PRD.md)** - Product requirements, features, roadmap
- **[Technical Spec](Project%20Plan/02.%20Tecnical%20Spec.md)** - Architecture, API, database schema
- **[Implementation Plan](https://github.com/brucembudi/kaizen-flow/wiki/Implementation-Plan)** - 6-week MVP roadmap

---

## 🗓️ Roadmap

### Phase 1: MVP (6 weeks) - **In Progress** 🚧

**Target:** March 18, 2026

- [x] Project setup and validation
- [ ] Core task management (CRUD)
- [ ] Quick capture with global hotkey
- [ ] Weekly review system
- [ ] Git-based sync
- [ ] Polish and testing

### Phase 2: Mobile (Weeks 7-10)

- Flutter app for iOS/Android
- Home screen widgets
- Feature parity with desktop

### Phase 3: Advanced Features (Weeks 11-14)

- MCP server for Claude Code integration
- Notebook mode (daily journal view)
- Context detection (time/location/activity)

### Phase 4: Collaboration Lite (Weeks 15+)

- Shared task lists (max 10 people)
- Basic assignment workflow
- Optional cloud sync server

---

## 🤝 Contributing

This is currently a personal project in MVP development. Contributions will be welcome after Phase 1 completion.

**Interested in contributing?** Watch this repo and check back in March 2026.

---

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

**Inspired by:**
- Getting Things Done (GTD) by David Allen
- Gemba Kaizen principles
- The simplicity of physical notebooks
- Frustration with abandoned task management apps

**Built with:**
- [Tauri](https://tauri.app/) - Desktop framework
- [React](https://react.dev/) - UI library
- [Rust](https://www.rust-lang.org/) - Backend language
- [SQLite](https://www.sqlite.org/) - Local database

---

## 📊 Status

**Current Phase:** Week 0 - Validation  
**Last Updated:** February 5, 2026  
**Version:** 0.0.1-alpha

### Recent Activity

- ✅ Completed project planning (PRD, Technical Spec)
- ✅ Created implementation plan
- 🚧 Setting up repository structure
- ⏳ Validating Git sync approach
- ⏳ Prototyping global hotkey system

---

## 💬 Contact

**Author:** Bruce Mbudi  
**Email:** bruceowenga@gmail.com
**Blog:** blog.brucembudi.dev
**GitHub:** [@brucembudi](https://github.com/brucembudi)

---

<p align="center">
  <strong>Built with focus. Designed for simplicity. Made for getting things done.</strong>
</p>

# ControlTerminal 🚀

![ControlTerminal Logo](assets/logo.ico)

**ControlTerminal** is a modern, lightweight terminal emulator with a graphical interface written in Rust. It combines the familiar command-line interface of Windows CMD with the advanced features of a modern terminal emulator.

---

## ✨ Features

- 🎨 **Customizable Interface** — change background and text colors on the fly
- 📜 **Smart Scrolling** — automatically follows text when you're at the bottom
- 🔍 **Command History** — navigate with up/down arrows
- ⌨️ **Keyboard Shortcuts** — Ctrl+A (select all), Ctrl+C (copy), Ctrl+V (paste)
- 💾 **Command Logging** — all commands are logged to `.controlllog`
- 🚀 **Built-in Commands** — 50+ commands for files, processes, network, and system
- 📦 **Aliases & Environment Variables** — full support
- 🧮 **Built-in Calculator** — evaluate expressions directly in the terminal
- 🔧 **Startup Manager** — manage Windows autostart programs
- 🪟 **Custom Icon** — your own logo in the title bar and taskbar
- 📦 **Small Size** — only 5 MB!

---

## 📋 Table of Contents

- [Installation](#installation)
- [Building from Source](#building-from-source)
- [Keyboard Shortcuts](#keyboard-shortcuts)
- [Commands](#commands)
- [Examples](#examples)
- [Configuration](#configuration)
- [Logging](#logging)
- [FAQ](#faq)
- [License](#license)

---

## 💻 Installation

### System Requirements
- Windows 10/11 (64-bit)
- 50 MB free disk space
- 512 MB RAM

### Download
1. Download the latest release from the [Releases](https://github.com/VladislavPim/ControlTerminal/releases) page
2. Extract the archive to any folder
3. Run `ControlTerminal.exe`

---

## 🔨 Building from Source

```bash
# Clone the repository
git clone https://github.com/VladislavPim/ControlTerminal.git
cd ControlTerminal

# Build in release mode
cargo build --release
```

# Run the terminal
target\release\ControlTerminal.exe

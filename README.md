```markdown
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

# Run the terminal
target\release\ControlTerminal.exe
```

---

## ⌨️ Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl + A` | Select all text in the current input line |
| `Ctrl + C` | Copy selected text |
| `Ctrl + V` | Paste text from clipboard |
| `↑` / `↓` | Navigate command history |

---

## 📚 Commands

### 📁 Navigation & Files

| Command | Aliases | Description |
|---------|---------|-------------|
| `cd` | `chdir`, `directory` | Change current directory |
| `ls` | `dir`, `list` | List directory contents |
| `pwd` | — | Print current working directory |
| `tree` | — | Display folder tree structure |
| `du` | — | Show disk usage (file/folder size) |
| `df` | `diskspace` | Show free disk space |
| `stat` | `fileinfo` | Show detailed file information |

### 📄 File Operations

| Command | Aliases | Description |
|---------|---------|-------------|
| `cat` | `type`, `show` | Display file contents |
| `cp` | `copy` | Copy files or folders |
| `mv` | `move`, `rename` | Move or rename files/folders |
| `rm` | `del`, `delete` | Delete files or folders |
| `mkdir` | `md`, `makedir` | Create new directory |
| `touch` | `create` | Create empty file or update file timestamp |
| `find` | `search` | Search for files by name |

### 💻 System Information

| Command | Aliases | Description |
|---------|---------|-------------|
| `sysinfo` | `system`, `info`, `neofetch` | Display system hardware information (CPU, RAM, disks) |
| `uptime` | — | Show system uptime |
| `date` | — | Display current date |
| `time` | — | Display current time |
| `hostname` | — | Show computer name |
| `whoami` | — | Show current username |

### ⚙️ Process Management

| Command | Aliases | Description |
|---------|---------|-------------|
| `ps` | `processes`, `tasklist` | List running processes |
| `kill` | `terminate`, `end` | Terminate process by PID or name |

### 🌐 Network

| Command | Aliases | Description |
|---------|---------|-------------|
| `ping` | — | Test connectivity to a host |
| `ipconfig` | `ip` | Display network interface information |
| `netstat` | — | Show network statistics and connections |
| `curl` | `download` | Download file via HTTP/HTTPS |

### 📝 Text Processing

| Command | Aliases | Description |
|---------|---------|-------------|
| `grep` | `findstr` | Search for text pattern in files |
| `head` | — | Display first N lines of file |
| `tail` | — | Display last N lines of file |
| `wc` | — | Count lines, words, and characters |
| `sort` | — | Sort lines alphabetically |
| `uniq` | — | Remove duplicate lines |
| `echo` | — | Print text to output |
| `calc` | `math` | Evaluate mathematical expressions |

### 🎨 Terminal Management

| Command | Aliases | Description |
|---------|---------|-------------|
| `bgcolor` | — | Change terminal background color |
| `fgcolor` | — | Change terminal text color |
| `clear` | `cls` | Clear terminal screen |
| `history` | — | Show command history |
| `alias` | — | Create or list command aliases |
| `set` | `env` | View or set environment variables |
| `run` | `execute`, `start` | Run external program |

**Available colors:** `black`, `white`, `red`, `green`, `blue`, `gray`, `darkgray`, `lightgray`, `yellow`, `cyan`, `magenta`

### 🔌 Startup Management (Windows)

| Command | Aliases | Description |
|---------|---------|-------------|
| `startup add` | — | Add program to Windows autostart |
| `startup remove` | — | Remove program from Windows autostart |
| `startup list` | — | List all programs in Windows autostart |

### 🔧 System Actions

| Command | Aliases | Description |
|---------|---------|-------------|
| `shutdown` | — | Shutdown the computer |
| `reboot` | — | Restart the computer |

### ❓ Help & Exit

| Command | Aliases | Description |
|---------|---------|-------------|
| `help` | — | Display all available commands |
| `exit` | `quit` | Exit ControlTerminal |

**Total Commands: 48** (including aliases) 🚀

---

## 🎯 Examples

### File Navigation
```
C:\Users\User> cd Desktop
C:\Users\User\Desktop> ls
[DIR]  Projects
[FILE] notes.txt
[FILE] todo.md
```

### File Operations
```
C:\Users\User\Desktop> mkdir test
Directory created: test

C:\Users\User\Desktop> touch test.txt
File touched: test.txt
```

### System Information
```
C:\Users\User> sysinfo
System name: Windows
Kernel version: 10.0.22631
OS version: Windows 10 Home
Hostname: DESKTOP-ABC123
Total memory: 16384 MB
Number of CPUs: 8
```

### Network
```
C:\Users\User> ping google.com
Pinging google.com [142.250.185.46] with 32 bytes of data:
Reply from 142.250.185.46: bytes=32 time=14ms TTL=117
```

### Calculator
```
C:\Users\User> calc 2+2
2+2 = 4
```

### Startup Management
```
C:\Users\User> startup add notepad.exe
✅ Added to startup: notepad.exe -> C:\Windows\notepad.exe

C:\Users\User> startup list
📋 Startup programs:
   - notepad.exe
```

### Customization
```
C:\Users\User> bgcolor darkgray
Background color changed

C:\Users\User> fgcolor yellow
Foreground color changed
```

---

## ⚙️ Configuration

Configuration file `.controlconfig` is created in your home directory on first run:

```ini
bgcolor = gray
fgcolor = white
aliases = {
    "ll": "ls -l",
    "gs": "git status"
}
env = {
    "EDITOR": "notepad"
}
```

---

## 📝 Logging

All commands are automatically logged to **`.controlllog`** in the current working directory:

```
[2024-01-15 10:30:45] cd Desktop
[2024-01-15 10:30:48] ls
[2024-01-15 10:30:52] echo Hello
```

---

## ❓ FAQ

### Q: Why can't I create files in C:\?
**A:** You don't have write permissions to the system drive root. Run the terminal as administrator or create files in your user folder.

### Q: Does it support Cyrillic/Russian letters?
**A:** Yes! The terminal fully supports UTF-8, including Cyrillic characters.

### Q: How do I copy text from the terminal?
**A:** Select text with your mouse and press `Ctrl+C`, or use the context menu.

### Q: How do I paste text?
**A:** Place cursor in the input field and press `Ctrl+V`.

### Q: Where are settings stored?
**A:** In `.controlconfig` in your home directory (`C:\Users\YourName\.controlconfig`).

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 👥 Author

- **Vladislav Pim** - [GitHub](https://github.com/VladislavPim)

---

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- GUI powered by [egui](https://github.com/emilk/egui)
- System information via [sysinfo](https://github.com/GuillaumeGomez/sysinfo)

---

**ControlTerminal** — made with ❤️ and Rust 🦀

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![Windows](https://img.shields.io/badge/Platform-Windows-blue.svg)](https://www.microsoft.com/windows)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
```

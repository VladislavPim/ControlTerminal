# README.md for GitHub Repository

```markdown
# ControlTerminal 🚀

![ControlTerminal Logo](assets/logo.ico)

**ControlTerminal** is a modern, lightweight terminal emulator with a graphical interface written in Rust. It combines the familiar command-line interface of Windows CMD with the advanced features of a modern terminal emulator.

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

## 📋 Table of Contents

- [Installation](#installation)
- [Building from Source](#building-from-source)
- [Keyboard Shortcuts](#keyboard-shortcuts)
- [Commands](#commands)
  - [Navigation & Files](#navigation--files)
  - [File Operations](#file-operations)
  - [System Information](#system-information)
  - [Process Management](#process-management)
  - [Network](#network)
  - [Text Processing](#text-processing)
  - [Terminal Management](#terminal-management)
  - [Startup Management](#startup-management)
  - [System Actions](#system-actions)
  - [Help & Exit](#help--exit)
- [Configuration](#configuration)
- [Logging](#logging)
- [FAQ](#faq)
- [License](#license)

## 💻 Installation

### System Requirements
- Windows 10/11 (64-bit)
- 50 MB free disk space
- 512 MB RAM (1+ GB recommended)

### Download
1. Download the latest release from the [Releases](https://github.com/yourusername/control-terminal/releases) page
2. Extract the archive to any folder
3. Run `control-terminal.exe`

## 🔨 Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/control-terminal.git
cd control-terminal

# Build in release mode
cargo build --release

# Run the terminal
target\release\control-terminal.exe
```

## ⌨️ Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl + A` | Select all text in the current input line |
| `Ctrl + C` | Copy selected text |
| `Ctrl + V` | Paste text from clipboard |
| `↑` / `↓` | Navigate command history |

## 📚 Commands

### Navigation & Files

| Command | Aliases | Description | Example |
|---------|---------|-------------|---------|
| `cd` | `chdir`, `directory` | Change directory | `cd C:\Windows` |
| `ls` | `dir`, `list` | List directory contents | `ls` or `ls D:\` |
| `pwd` | — | Print working directory | `pwd` |
| `tree` | — | Show folder tree | `tree` or `tree C:\Projects` |
| `du` | — | Show file/folder size | `du` or `du file.txt` |
| `df` | `diskspace` | Show disk free space | `df` |
| `stat` | `fileinfo` | Detailed file information | `stat Cargo.toml` |

### File Operations

| Command | Aliases | Description | Example |
|---------|---------|-------------|---------|
| `cat` | `type`, `show` | Display file contents | `cat README.md` |
| `cp` | `copy` | Copy files/folders | `cp file.txt backup.txt` |
| `mv` | `move`, `rename` | Move/rename files | `mv old.txt new.txt` |
| `rm` | `del`, `delete` | Delete files/folders | `rm temp.txt` |
| `mkdir` | `md`, `makedir` | Create directory | `mkdir new_folder` |
| `touch` | `create` | Create empty file | `touch notes.txt` |
| `find` | `search` | Search files by name | `find .rs src` |

### System Information

| Command | Aliases | Description | Example |
|---------|---------|-------------|---------|
| `sysinfo` | `system`, `info`, `neofetch` | Hardware information | `sysinfo` |
| `uptime` | — | System uptime | `uptime` |
| `date` | — | Show/set date | `date` |
| `time` | — | Show/set time | `time` |
| `hostname` | — | Computer name | `hostname` |
| `whoami` | — | Current username | `whoami` |

### Process Management

| Command | Aliases | Description | Example |
|---------|---------|-------------|---------|
| `ps` | `processes`, `tasklist` | List running processes | `ps` |
| `kill` | `terminate`, `end` | Terminate process by PID | `kill 1234` |

### Network

| Command | Aliases | Description | Example |
|---------|---------|-------------|---------|
| `ping` | — | Test host connectivity | `ping google.com` |
| `ipconfig` | `ip` | Network interface information | `ipconfig` |
| `netstat` | — | Network statistics | `netstat` |
| `curl` | `download` | Download file via HTTP | `curl https://example.com` |

### Text Processing

| Command | Aliases | Description | Example |
|---------|---------|-------------|---------|
| `grep` | `findstr` | Search for pattern in files | `grep fn src/main.rs` |
| `head` | — | Show first N lines | `head -n 5 file.txt` |
| `tail` | — | Show last N lines | `tail -n 10 log.txt` |
| `wc` | — | Count lines/words/characters | `wc document.txt` |
| `sort` | — | Sort lines | `sort names.txt` |
| `uniq` | — | Remove duplicate lines | `uniq data.txt` |
| `echo` | — | Print text | `echo Hello World` |
| `calc` | `math` | Simple calculator | `calc 2+2*3` |

### Terminal Management

| Command | Aliases | Description | Example |
|---------|---------|-------------|---------|
| `bgcolor` | — | Change background color | `bgcolor blue` |
| `fgcolor` | — | Change text color | `fgcolor yellow` |
| `clear` | `cls` | Clear screen | `clear` |
| `history` | — | Show command history | `history` |
| `alias` | — | Create command alias | `alias ll ls -l` |
| `set` | `env` | View/set environment variables | `set PATH=C:\bin` |
| `run` | `execute`, `start` | Run external program | `run notepad.exe` |

**Available colors**: `black`, `white`, `red`, `green`, `blue`, `gray`, `darkgray`, `lightgray`, `yellow`, `cyan`, `magenta`

### Startup Management

| Command | Aliases | Description | Example |
|---------|---------|-------------|---------|
| `startup add` | — | Add program to Windows autostart | `startup add notepad.exe` |
| `startup remove` | — | Remove program from autostart | `startup remove notepad.exe` |
| `startup list` | — | List autostart programs | `startup list` |

*Note: Works via Windows registry (HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run).*

### System Actions

| Command | Aliases | Description | Example |
|---------|---------|-------------|---------|
| `shutdown` | — | Shutdown computer | `shutdown /s` |
| `reboot` | — | Reboot computer | `reboot` |

### Help & Exit

| Command | Aliases | Description | Example |
|---------|---------|-------------|---------|
| `help` | — | Show all commands | `help` |
| `exit` | `quit` | Exit terminal | `exit` |

## 🎯 Usage Examples

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

C:\Users\User\Desktop> cp test.txt test_copy.txt
Copied test.txt to test_copy.txt
```

### System Information
```
C:\Users\User> sysinfo
System name: Windows
Kernel version: 10.0.22631
OS version: Windows 11 Home
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

### Text Processing
```
C:\Users\User\Desktop> cat notes.txt
apple
banana
apple
cherry

C:\Users\User\Desktop> sort notes.txt
apple
apple
banana
cherry

C:\Users\User\Desktop> grep cherry notes.txt
cherry
```

### Calculator
```
C:\Users\User> calc 2+2
2+2 = 4

C:\Users\User> calc sin(3.1415/2)
sin(3.1415/2) = 0.9999999999999999
```

### Startup Management
```
C:\Users\User> startup add notepad.exe
✅ Added to startup: notepad.exe -> C:\Windows\notepad.exe

C:\Users\User> startup list
📋 Startup programs:
   - notepad.exe
   - discord.exe

C:\Users\User> startup remove notepad.exe
✅ Removed from startup: notepad.exe
```

### Customization
```
C:\Users\User> bgcolor darkgray
Background color changed

C:\Users\User> fgcolor yellow
Foreground color changed
```

### Aliases
```
C:\Users\User> alias ll ls
Alias added

C:\Users\User> ll
[DIR]  Desktop
[DIR]  Downloads
[FILE] file.txt
```

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

## 📝 Logging

All commands are automatically logged to **`.controlllog`** in the current working directory:

```
[2024-01-15 10:30:45] cd Desktop
[2024-01-15 10:30:48] ls
[2024-01-15 10:30:52] echo Hello
```

## ❓ FAQ

### Q: Why can't I create files in C:\?
**A:** You don't have write permissions to the system drive root. Run the terminal as administrator or create files in your user folder (e.g., `C:\Users\YourName`).

### Q: Does it support Cyrillic/Russian letters?
**A:** Yes! The terminal fully supports UTF-8, including Cyrillic characters.

### Q: How do I copy text from the terminal?
**A:** Select text with your mouse and press `Ctrl+C`, or use the context menu (right-click).

### Q: How do I paste text?
**A:** Place cursor in the input field and press `Ctrl+V`.

### Q: Where are settings stored?
**A:** In `.controlconfig` in your home directory (`C:\Users\YourName\.controlconfig`).

### Q: Why doesn't scroll always follow the text?
**A:** Scroll follows only when you're at the bottom. If you scroll up, new text won't move your position — this lets you read history without distraction.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👥 Authors

- **Your Name** - *Initial work* - [GitHub](https://github.com/yourusername)

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

This README includes:
- ✅ Clean, professional formatting
- ✅ Complete command list with aliases and examples
- ✅ Installation instructions
- ✅ Build instructions
- ✅ Keyboard shortcuts
- ✅ Configuration guide
- ✅ FAQ
- ✅ Badges and acknowledgments

You can customize the GitHub username, repository URL, and add screenshots if you want. Perfect for presenting your project to the world! 🚀

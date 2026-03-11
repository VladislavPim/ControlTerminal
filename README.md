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



⌨️ Keyboard Shortcuts
Shortcut	Action
Ctrl + A	Select all text in input line
Ctrl + C	Copy selected text
Ctrl + V	Paste text from clipboard
↑ / ↓	Navigate command history
📚 Commands
Navigation & Files
Command	Aliases	Description	Example
cd	chdir, directory	Change directory	cd C:\Windows
ls	dir, list	List directory contents	ls or ls D:\
pwd	—	Print working directory	pwd
tree	—	Show folder tree	tree or tree C:\Projects
du	—	Show file/folder size	du or du file.txt
df	diskspace	Show disk free space	df
stat	fileinfo	Detailed file information	stat Cargo.toml
File Operations
Command	Aliases	Description	Example
cat	type, show	Display file contents	cat README.md
cp	copy	Copy files/folders	cp file.txt backup.txt
mv	move, rename	Move/rename files	mv old.txt new.txt
rm	del, delete	Delete files/folders	rm temp.txt
mkdir	md, makedir	Create directory	mkdir new_folder
touch	create	Create empty file	touch notes.txt
find	search	Search files by name	find .rs src
System Information
Command	Aliases	Description	Example
sysinfo	system, info, neofetch	Hardware information	sysinfo
uptime	—	System uptime	uptime
date	—	Show/set date	date
time	—	Show/set time	time
hostname	—	Computer name	hostname
whoami	—	Current username	whoami
Process Management
Command	Aliases	Description	Example
ps	processes, tasklist	List running processes	ps
kill	terminate, end	Terminate process by PID	kill 1234
Network
Command	Aliases	Description	Example
ping	—	Test host connectivity	ping google.com
ipconfig	ip	Network interface information	ipconfig
netstat	—	Network statistics	netstat
curl	download	Download file via HTTP	curl https://example.com
Text Processing
Command	Aliases	Description	Example
grep	findstr	Search for pattern in files	grep fn src/main.rs
head	—	Show first N lines	head -n 5 file.txt
tail	—	Show last N lines	tail -n 10 log.txt
wc	—	Count lines/words/characters	wc document.txt
sort	—	Sort lines	sort names.txt
uniq	—	Remove duplicate lines	uniq data.txt
echo	—	Print text	echo Hello World
calc	math	Simple calculator	calc 2+2*3
Terminal Management
Command	Aliases	Description	Example
bgcolor	—	Change background color	bgcolor blue
fgcolor	—	Change text color	fgcolor yellow
clear	cls	Clear screen	clear
history	—	Show command history	history
alias	—	Create command alias	alias ll ls -l
set	env	View/set environment variables	set PATH=C:\bin
run	execute, start	Run external program	run notepad.exe
Available colors: black, white, red, green, blue, gray, darkgray, lightgray, yellow, cyan, magenta

Startup Management
Command	Aliases	Description	Example
startup add	—	Add program to Windows autostart	startup add notepad.exe
startup remove	—	Remove program from autostart	startup remove notepad.exe
startup list	—	List autostart programs	startup list
Note: Works via Windows registry (HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run).

System Actions
Command	Aliases	Description	Example
shutdown	—	Shutdown computer	shutdown /s
reboot	—	Reboot computer	reboot
Help & Exit
Command	Aliases	Description	Example
help	—	Show all commands	help
exit	quit	Exit terminal	exit
# Run the terminal
target\release\ControlTerminal.exe

use crate::terminal::CommandContext;
use egui::Color32;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::collections::HashSet;
use std::process::Command;
use std::env;
use chrono::Local;
use sysinfo::System;
use whoami;
use regex::Regex;
use filetime;
use winreg::RegKey;
use winreg::enums::*;

// --- Существующие функции (без изменений, но для краткости оставлю только заголовки) ---
pub fn cmd_pwd(_args: &[String], ctx: &mut CommandContext) -> Result<String, String> {
    Ok(ctx.state.current_dir.display().to_string())
}

pub fn cmd_cd(args: &[String], ctx: &mut CommandContext) -> Result<String, String> {
    let target = if args.len() > 1 {
        args[1].clone()
    } else {
        dirs::home_dir()
            .ok_or_else(|| "Cannot determine home directory".to_string())?
            .display()
            .to_string()
    };
    let path = ctx.state.current_dir.join(&target);
    if path.exists() && path.is_dir() {
        ctx.state.current_dir = path;
        std::env::set_current_dir(&ctx.state.current_dir)
            .map_err(|e| format!("Cannot set current dir: {}", e))?;
        Ok(format!("Changed directory to {}", ctx.state.current_dir.display()))
    } else {
        Err(format!("Directory not found: {}", target))
    }
}

pub fn cmd_ls(args: &[String], ctx: &mut CommandContext) -> Result<String, String> {
    let path = if args.len() > 1 {
        ctx.state.current_dir.join(&args[1])
    } else {
        ctx.state.current_dir.clone()
    };
    let entries = fs::read_dir(&path)
        .map_err(|e| format!("Cannot read directory: {}", e))?;
    let mut result = Vec::new();
    for entry in entries {
        if let Ok(entry) = entry {
            let file_name = entry.file_name().to_string_lossy().to_string();
            let file_type = if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                "[DIR]"
            } else {
                "[FILE]"
            };
            result.push(format!("{} {}", file_type, file_name));
        }
    }
    if result.is_empty() { Ok("(empty)".to_string()) } else { Ok(result.join("\n")) }
}

pub fn cmd_cat(args: &[String], ctx: &mut CommandContext) -> Result<String, String> {
    if args.len() < 2 { return Err("Usage: cat <file>".to_string()); }
    let path = ctx.state.current_dir.join(&args[1]);
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Cannot read file: {}", e))?;
    Ok(content)
}

pub fn cmd_echo(args: &[String], _ctx: &mut CommandContext) -> Result<String, String> {
    if args.len() > 1 { Ok(args[1..].join(" ")) } else { Ok(String::new()) }
}

pub fn cmd_whoami(_args: &[String], _ctx: &mut CommandContext) -> Result<String, String> {
    Ok(whoami::username())
}

pub fn cmd_date(_args: &[String], _ctx: &mut CommandContext) -> Result<String, String> {
    Ok(Local::now().format("%Y-%m-%d").to_string())
}

pub fn cmd_time(_args: &[String], _ctx: &mut CommandContext) -> Result<String, String> {
    Ok(Local::now().format("%H:%M:%S").to_string())
}

pub fn cmd_uptime(_args: &[String], _ctx: &mut CommandContext) -> Result<String, String> {
    let uptime_secs = System::uptime();
    let days = uptime_secs / 86400;
    let hours = (uptime_secs % 86400) / 3600;
    let minutes = (uptime_secs % 3600) / 60;
    Ok(format!("{} days, {} hours, {} minutes", days, hours, minutes))
}

pub fn cmd_hostname(_args: &[String], _ctx: &mut CommandContext) -> Result<String, String> {
    match whoami::fallible::hostname() {
        Ok(name) => Ok(name),
        Err(_) => Ok("unknown".to_string()),
    }
}

pub fn cmd_sysinfo(_args: &[String], _ctx: &mut CommandContext) -> Result<String, String> {
    let mut system = System::new_all();
    system.refresh_all();
    let mut info = Vec::new();
    info.push(format!("System name: {}", System::name().unwrap_or_default()));
    info.push(format!("Kernel version: {}", System::kernel_version().unwrap_or_default()));
    info.push(format!("OS version: {}", System::os_version().unwrap_or_default()));
    info.push(format!("Hostname: {}", System::host_name().unwrap_or_default()));
    info.push(format!("Total memory: {} MB", system.total_memory() / 1024 / 1024));
    info.push(format!("Used memory: {} MB", system.used_memory() / 1024 / 1024));
    info.push(format!("Total swap: {} MB", system.total_swap() / 1024 / 1024));
    info.push(format!("Used swap: {} MB", system.used_swap() / 1024 / 1024));
    info.push(format!("Number of CPUs: {}", system.cpus().len()));
    Ok(info.join("\n"))
}

pub fn cmd_clear(_args: &[String], _ctx: &mut CommandContext) -> Result<String, String> {
    Ok("\x1b[2J\x1b[1;1H".to_string())
}

pub fn cmd_history(_args: &[String], _ctx: &mut CommandContext) -> Result<String, String> {
    Ok("History command not fully implemented yet".to_string())
}

pub fn cmd_alias(args: &[String], ctx: &mut CommandContext) -> Result<String, String> {
    if args.len() == 1 {
        let mut list: Vec<String> = ctx.state.aliases.iter()
            .map(|(k, v)| format!("{}='{}'", k, v))
            .collect();
        list.sort();
        Ok(list.join("\n"))
    } else if args.len() >= 3 {
        let name = args[1].clone();
        let value = args[2..].join(" ");
        ctx.state.aliases.insert(name, value);
        Ok("Alias added".to_string())
    } else {
        Err("Usage: alias [name value]".to_string())
    }
}

pub fn cmd_set(args: &[String], ctx: &mut CommandContext) -> Result<String, String> {
    if args.len() == 1 {
        let mut list: Vec<String> = ctx.state.env_vars.iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        list.sort();
        Ok(list.join("\n"))
    } else if args.len() == 2 {
        let arg = &args[1];
        if let Some((key, value)) = arg.split_once('=') {
            ctx.state.set_env(key.to_string(), value.to_string());
            Ok(format!("{}={}", key, value))
        } else {
            match ctx.state.get_env(arg) {
                Some(val) => Ok(format!("{}={}", arg, val)),
                None => Err(format!("Variable {} not set", arg)),
            }
        }
    } else {
        Err("Usage: set [VAR[=VALUE]]".to_string())
    }
}

pub fn cmd_bgcolor(args: &[String], ctx: &mut CommandContext) -> Result<String, String> {
    if args.len() < 2 { return Err("Usage: bgcolor <color>".to_string()); }
    let color = parse_color(&args[1])?;
    *ctx.bg_color = color;
    Ok("Background color changed".to_string())
}

pub fn cmd_fgcolor(args: &[String], ctx: &mut CommandContext) -> Result<String, String> {
    if args.len() < 2 { return Err("Usage: fgcolor <color>".to_string()); }
    let color = parse_color(&args[1])?;
    *ctx.fg_color = color;
    Ok("Foreground color changed".to_string())
}

fn parse_color(name: &str) -> Result<Color32, String> {
    match name.to_lowercase().as_str() {
        "black" => Ok(Color32::BLACK),
        "white" => Ok(Color32::WHITE),
        "red" => Ok(Color32::RED),
        "green" => Ok(Color32::GREEN),
        "blue" => Ok(Color32::BLUE),
        "gray" | "grey" => Ok(Color32::GRAY),
        "darkgray" => Ok(Color32::DARK_GRAY),
        "lightgray" => Ok(Color32::LIGHT_GRAY),
        "yellow" => Ok(Color32::YELLOW),
        "cyan" => Ok(Color32::from_rgb(0, 255, 255)),
        "magenta" => Ok(Color32::from_rgb(255, 0, 255)),
        _ => Err(format!("Unknown color: {}", name)),
    }
}

pub fn cmd_help(_args: &[String], _ctx: &mut CommandContext) -> Result<String, String> {
    let help_text = r#"Control Terminal - available commands:

  Navigation: cd, ls, pwd, tree
  Files: cat, cp, mv, rm, mkdir, touch, find, du, df, stat
  System: whoami, date, time, sysinfo, uptime, hostname, ps, kill
  Network: ping, ipconfig, netstat, curl
  Text: grep, head, tail, wc, sort, uniq, echo, calc
  Terminal: clear, history, alias, set, bgcolor, fgcolor, run
  Startup: startup add, startup remove, startup list
  System: shutdown, reboot
  Other: help, exit

Type 'help <command>' for more info on a specific command (not implemented yet)."#;
    Ok(help_text.to_string())
}

pub fn cmd_exit(_args: &[String], _ctx: &mut CommandContext) -> Result<String, String> {
    Ok("\x18EXIT\x18".to_string())
}

pub fn cmd_run(args: &[String], ctx: &mut CommandContext) -> Result<String, String> {
    if args.len() < 2 { return Err("Usage: run <program> [args...]".to_string()); }
    let program = &args[1];
    let program_args = &args[2..];
    let path = if Path::new(program).is_absolute() {
        PathBuf::from(program)
    } else {
        let candidate = ctx.state.current_dir.join(program);
        if candidate.exists() { candidate } else {
            return Err(format!("Program not found: {}", program));
        }
    };
    let output = std::process::Command::new(&path)
        .args(program_args)
        .current_dir(&ctx.state.current_dir)
        .output()
        .map_err(|e| format!("Failed to execute program: {}", e))?;
    let mut result = String::new();
    if !output.stdout.is_empty() { result.push_str(&String::from_utf8_lossy(&output.stdout)); }
    if !output.stderr.is_empty() {
        if !result.is_empty() { result.push('\n'); }
        result.push_str(&String::from_utf8_lossy(&output.stderr));
    }
    if result.is_empty() { result = format!("Program exited with status: {}", output.status); }
    Ok(result)
}

pub fn cmd_calc(args: &[String], _ctx: &mut CommandContext) -> Result<String, String> {
    if args.len() < 2 { return Err("Usage: calc <expression>".to_string()); }
    let expr = args[1..].join(" ");
    match meval::eval_str(&expr) {
        Ok(value) => Ok(format!("{} = {}", expr, value)),
        Err(e) => Err(format!("Calculation error: {}", e)),
    }
}

// --- НОВЫЕ КОМАНДЫ ---

// ========== Управление файлами и папками ==========

pub fn cmd_mkdir(args: &[String], ctx: &mut CommandContext) -> Result<String, String> {
    if args.len() < 2 { return Err("Usage: mkdir <dirname>".to_string()); }
    let path = ctx.state.current_dir.join(&args[1]);
    fs::create_dir(&path)
        .map_err(|e| format!("Cannot create directory: {}", e))?;
    Ok(format!("Directory created: {}", args[1]))
}

pub fn cmd_touch(args: &[String], ctx: &mut CommandContext) -> Result<String, String> {
    if args.len() < 2 { return Err("Usage: touch <filename>".to_string()); }
    let path = ctx.state.current_dir.join(&args[1]);
    if path.exists() {
        // Обновляем время доступа/модификации
        filetime::set_file_mtime(&path, filetime::FileTime::now())
            .map_err(|e| format!("Cannot update file time: {}", e))?;
    } else {
        fs::File::create(&path)
            .map_err(|e| format!("Cannot create file: {}", e))?;
    }
    Ok(format!("File touched: {}", args[1]))
}

pub fn cmd_cp(args: &[String], ctx: &mut CommandContext) -> Result<String, String> {
    if args.len() < 3 { return Err("Usage: cp <source> <dest>".to_string()); }
    let src = ctx.state.current_dir.join(&args[1]);
    let dst = ctx.state.current_dir.join(&args[2]);
    if src.is_dir() {
        copy_dir(&src, &dst).map_err(|e| format!("Copy error: {}", e))?;
    } else {
        fs::copy(&src, &dst).map_err(|e| format!("Copy error: {}", e))?;
    }
    Ok(format!("Copied {} to {}", args[1], args[2]))
}

fn copy_dir(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

pub fn cmd_mv(args: &[String], ctx: &mut CommandContext) -> Result<String, String> {
    if args.len() < 3 { return Err("Usage: mv <source> <dest>".to_string()); }
    let src = ctx.state.current_dir.join(&args[1]);
    let dst = ctx.state.current_dir.join(&args[2]);
    fs::rename(&src, &dst).map_err(|e| format!("Move error: {}", e))?;
    Ok(format!("Moved {} to {}", args[1], args[2]))
}

pub fn cmd_rm(args: &[String], ctx: &mut CommandContext) -> Result<String, String> {
    if args.len() < 2 { return Err("Usage: rm <file/dir> [-r]".to_string()); }
    let recursive = args.contains(&"-r".to_string()) || args.contains(&"--recursive".to_string());
    let target = ctx.state.current_dir.join(&args[1]);
    if target.is_dir() && recursive {
        fs::remove_dir_all(&target).map_err(|e| format!("Cannot remove directory: {}", e))?;
    } else if target.is_dir() {
        fs::remove_dir(&target).map_err(|e| format!("Directory not empty? Use -r: {}", e))?;
    } else {
        fs::remove_file(&target).map_err(|e| format!("Cannot remove file: {}", e))?;
    }
    Ok(format!("Removed: {}", args[1]))
}

// ========== Поиск и информация ==========

pub fn cmd_find(args: &[String], ctx: &mut CommandContext) -> Result<String, String> {
    if args.len() < 2 { return Err("Usage: find <pattern> [path]".to_string()); }
    let pattern = &args[1];
    let start_path = if args.len() > 2 {
        ctx.state.current_dir.join(&args[2])
    } else {
        ctx.state.current_dir.clone()
    };
    let mut results = Vec::new();
    find_files(&start_path, pattern, &mut results, &ctx.state.current_dir)?;
    Ok(results.join("\n"))
}

fn find_files(dir: &Path, pattern: &str, results: &mut Vec<String>, base: &Path) -> Result<(), String> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).map_err(|e| format!("Cannot read dir: {}", e))? {
            let entry = entry.map_err(|e| format!("Entry error: {}", e))?;
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.contains(pattern) {
                    let rel = path.strip_prefix(base).unwrap_or(&path).display();
                    results.push(rel.to_string());
                }
            }
            if path.is_dir() {
                find_files(&path, pattern, results, base)?;
            }
        }
    }
    Ok(())
}

pub fn cmd_tree(args: &[String], ctx: &mut CommandContext) -> Result<String, String> {
    let start_path = if args.len() > 1 {
        ctx.state.current_dir.join(&args[1])
    } else {
        ctx.state.current_dir.clone()
    };
    let mut output = Vec::new();
    tree_helper(&start_path, "".to_string(), &mut output, true)?;
    Ok(output.join("\n"))
}

fn tree_helper(dir: &Path, prefix: String, output: &mut Vec<String>, _is_last: bool) -> Result<(), String> {
    if !dir.is_dir() {
        return Err("Not a directory".to_string());
    }
    let entries: Vec<_> = fs::read_dir(dir)
        .map_err(|e| format!("Cannot read dir: {}", e))?
        .filter_map(Result::ok)
        .collect();
    for (i, entry) in entries.iter().enumerate() {
        let is_last_entry = i == entries.len() - 1;
        let file_name = entry.file_name().to_string_lossy().to_string();
        let marker = if is_last_entry { "└── " } else { "├── " };
        output.push(format!("{}{}{}", prefix, marker, file_name));
        if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
            let new_prefix = if is_last_entry {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };
            tree_helper(&entry.path(), new_prefix, output, is_last_entry)?;
        }
    }
    Ok(())
}

pub fn cmd_du(args: &[String], ctx: &mut CommandContext) -> Result<String, String> {
    let target = if args.len() > 1 {
        ctx.state.current_dir.join(&args[1])
    } else {
        ctx.state.current_dir.clone()
    };
    let size = dir_size(&target)?;
    Ok(format!("{} bytes ({:.2} KB, {:.2} MB)", size, size as f64 / 1024.0, size as f64 / (1024.0*1024.0)))
}

fn dir_size(path: &Path) -> Result<u64, String> {
    let mut total = 0;
    if path.is_file() {
        total = fs::metadata(path).map(|m| m.len()).unwrap_or(0);
    } else if path.is_dir() {
        for entry in fs::read_dir(path).map_err(|e| format!("Cannot read dir: {}", e))? {
            let entry = entry.map_err(|e| format!("Entry error: {}", e))?;
            let sub_path = entry.path();
            total += dir_size(&sub_path)?;
        }
    }
    Ok(total)
}

pub fn cmd_df(_args: &[String], _ctx: &mut CommandContext) -> Result<String, String> {
    // Для Windows можно использовать `GetDiskFreeSpaceEx` или `fs2`, но проще через `wmic` или `std::process::Command`
    #[cfg(windows)]
    {
        let output = Command::new("wmic")
            .args(&["logicaldisk", "get", "size,freespace,caption"])
            .output()
            .map_err(|e| format!("Failed to run wmic: {}", e))?;
        let result = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(result)
    }
    #[cfg(not(windows))]
    {
        let output = Command::new("df")
            .arg("-h")
            .output()
            .map_err(|e| format!("Failed to run df: {}", e))?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

pub fn cmd_stat(args: &[String], ctx: &mut CommandContext) -> Result<String, String> {
    if args.len() < 2 { return Err("Usage: stat <file>".to_string()); }
    let path = ctx.state.current_dir.join(&args[1]);
    let meta = fs::metadata(&path).map_err(|e| format!("Cannot stat file: {}", e))?;
    let mut info = Vec::new();
    info.push(format!("File: {}", path.display()));
    info.push(format!("Size: {} bytes", meta.len()));
    info.push(format!("Type: {}", if meta.is_dir() { "directory" } else { "file" }));
    info.push(format!("Permissions: {:?}", meta.permissions()));
    if let Ok(modified) = meta.modified() {
        if let Ok(duration) = modified.duration_since(std::time::UNIX_EPOCH) {
            let datetime = chrono::DateTime::from_timestamp(duration.as_secs() as i64, 0);
            if let Some(dt) = datetime {
                info.push(format!("Modified: {}", dt.format("%Y-%m-%d %H:%M:%S")));
            }
        }
    }
    Ok(info.join("\n"))
}

// ========== Сеть ==========

pub fn cmd_ping(args: &[String], _ctx: &mut CommandContext) -> Result<String, String> {
    if args.len() < 2 { return Err("Usage: ping <host>".to_string()); }
    let host = &args[1];
    #[cfg(windows)]
    let output = Command::new("ping")
        .args(&["-n", "1", host])
        .output()
        .map_err(|e| format!("Failed to run ping: {}", e))?;
    #[cfg(not(windows))]
    let output = Command::new("ping")
        .args(&["-c", "1", host])
        .output()
        .map_err(|e| format!("Failed to run ping: {}", e))?;
    let result = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(result)
}

pub fn cmd_ipconfig(_args: &[String], _ctx: &mut CommandContext) -> Result<String, String> {
    #[cfg(windows)]
    let output = Command::new("ipconfig")
        .output()
        .map_err(|e| format!("Failed to run ipconfig: {}", e))?;
    #[cfg(not(windows))]
    let output = Command::new("ifconfig")
        .output()
        .map_err(|e| format!("Failed to run ifconfig: {}", e))?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn cmd_netstat(args: &[String], _ctx: &mut CommandContext) -> Result<String, String> {
    let mut cmd = Command::new("netstat");
    if args.len() > 1 {
        cmd.args(&args[1..]);
    }
    let output = cmd.output()
        .map_err(|e| format!("Failed to run netstat: {}", e))?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn cmd_curl(args: &[String], ctx: &mut CommandContext) -> Result<String, String> {
    if args.len() < 2 { return Err("Usage: curl <url> [options]".to_string()); }
    // Просто вызываем внешний curl (должен быть в PATH)
    let output = Command::new("curl")
        .args(&args[1..])
        .current_dir(&ctx.state.current_dir)
        .output()
        .map_err(|e| format!("Failed to run curl: {}", e))?;
    let mut result = String::new();
    if !output.stdout.is_empty() { result.push_str(&String::from_utf8_lossy(&output.stdout)); }
    if !output.stderr.is_empty() {
        if !result.is_empty() { result.push('\n'); }
        result.push_str(&String::from_utf8_lossy(&output.stderr));
    }
    Ok(result)
}

// ========== Работа с текстом ==========

pub fn cmd_grep(args: &[String], ctx: &mut CommandContext) -> Result<String, String> {
    if args.len() < 3 { return Err("Usage: grep <pattern> <file>".to_string()); }
    let pattern = &args[1];
    let file = ctx.state.current_dir.join(&args[2]);
    let content = fs::read_to_string(&file)
        .map_err(|e| format!("Cannot read file: {}", e))?;
    let re = Regex::new(pattern).map_err(|e| format!("Invalid regex: {}", e))?;
    let mut matches = Vec::new();
    for (i, line) in content.lines().enumerate() {
        if re.is_match(line) {
            matches.push(format!("{}:{}", i+1, line));
        }
    }
    if matches.is_empty() {
        Ok("No matches".to_string())
    } else {
        Ok(matches.join("\n"))
    }
}

pub fn cmd_head(args: &[String], ctx: &mut CommandContext) -> Result<String, String> {
    let mut n = 10;
    let mut file_index = 1; // теперь mut
    if args.len() > 2 && args[1].starts_with("-n") {
        if args.len() > 2 {
            n = args[2].parse::<usize>().map_err(|_| "Invalid number")?;
            file_index = 3;
        } else {
            return Err("Usage: head [-n N] <file>".to_string());
        }
    }
    if args.len() <= file_index { return Err("Usage: head [-n N] <file>".to_string()); }
    let file = ctx.state.current_dir.join(&args[file_index]);
    let content = fs::read_to_string(&file)
        .map_err(|e| format!("Cannot read file: {}", e))?;
    let lines: Vec<&str> = content.lines().take(n).collect();
    Ok(lines.join("\n"))
}

pub fn cmd_tail(args: &[String], ctx: &mut CommandContext) -> Result<String, String> {
    let mut n = 10;
    let mut file_index = 1;
    if args.len() > 2 && args[1].starts_with("-n") {
        if args.len() > 2 {
            n = args[2].parse::<usize>().map_err(|_| "Invalid number")?;
            file_index = 3;
        } else {
            return Err("Usage: tail [-n N] <file>".to_string());
        }
    }
    if args.len() <= file_index { return Err("Usage: tail [-n N] <file>".to_string()); }
    let file = ctx.state.current_dir.join(&args[file_index]);
    let content = fs::read_to_string(&file)
        .map_err(|e| format!("Cannot read file: {}", e))?;
    let lines: Vec<&str> = content.lines().collect();
    let start = if lines.len() > n { lines.len() - n } else { 0 };
    Ok(lines[start..].join("\n"))
}

pub fn cmd_wc(args: &[String], ctx: &mut CommandContext) -> Result<String, String> {
    if args.len() < 2 { return Err("Usage: wc <file>".to_string()); }
    let file = ctx.state.current_dir.join(&args[1]);
    let content = fs::read_to_string(&file)
        .map_err(|e| format!("Cannot read file: {}", e))?;
    let lines = content.lines().count();
    let words = content.split_whitespace().count();
    let chars = content.chars().count();
    Ok(format!("{:8} {:8} {:8} {}", lines, words, chars, args[1]))
}

pub fn cmd_sort(args: &[String], ctx: &mut CommandContext) -> Result<String, String> {
    if args.len() < 2 { return Err("Usage: sort <file>".to_string()); }
    let file = ctx.state.current_dir.join(&args[1]);
    let content = fs::read_to_string(&file)
        .map_err(|e| format!("Cannot read file: {}", e))?;
    let mut lines: Vec<&str> = content.lines().collect();
    lines.sort();
    Ok(lines.join("\n"))
}

pub fn cmd_uniq(args: &[String], ctx: &mut CommandContext) -> Result<String, String> {
    if args.len() < 2 { return Err("Usage: uniq <file>".to_string()); }
    let file = ctx.state.current_dir.join(&args[1]);
    let content = fs::read_to_string(&file)
        .map_err(|e| format!("Cannot read file: {}", e))?;
    let mut seen = HashSet::new();
    let mut unique = Vec::new();
    for line in content.lines() {
        if seen.insert(line) {
            unique.push(line);
        }
    }
    Ok(unique.join("\n"))
}

// ========== Процессы ==========

pub fn cmd_ps(_args: &[String], _ctx: &mut CommandContext) -> Result<String, String> {
    let mut system = System::new_all();
    system.refresh_all();
    let mut output = Vec::new();
    output.push(format!("{:>8} {:>8} {:<30} {}", "PID", "PPID", "NAME", "CPU%"));

    for process in system.processes().values() {
        let pid = process.pid().as_u32();        // получаем u32
        let ppid = process.parent().map(|p| p.as_u32()).unwrap_or(0);
        let name = process.name();
        let cpu = process.cpu_usage();
        output.push(format!("{:8} {:8} {:<30} {:.2}", pid, ppid, name, cpu));
    }
    Ok(output.join("\n"))
}

pub fn cmd_kill(args: &[String], _ctx: &mut CommandContext) -> Result<String, String> {
    if args.len() < 2 { return Err("Usage: kill <pid>".to_string()); }
    let pid_str = &args[1];
    let pid = pid_str.parse::<u32>().map_err(|_| "Invalid PID")?;
    #[cfg(windows)]
    {
        use std::process::Command;
        let output = Command::new("taskkill")
            .args(&["/F", "/PID", &pid.to_string()])
            .output()
            .map_err(|e| format!("Failed to kill process: {}", e))?;
        if output.status.success() {
            Ok(format!("Process {} terminated", pid))
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }
    #[cfg(not(windows))]
    {
        // Для Linux/Mac можно использовать libc, но пока заглушка
        Err("Kill command not implemented on this platform".to_string())
    }
}

// ========== Системные действия ==========

pub fn cmd_shutdown(args: &[String], _ctx: &mut CommandContext) -> Result<String, String> {
    #[cfg(windows)]
    {
        if args.len() > 1 && args[1] == "/s" {
            Command::new("shutdown")
                .args(&["/s", "/t", "0"])
                .spawn()
                .map_err(|e| format!("Shutdown failed: {}", e))?;
            Ok("Shutting down...".to_string())
        } else {
            Err("Usage: shutdown /s".to_string())
        }
    }
    #[cfg(not(windows))]
    {
        Command::new("shutdown")
            .arg("-h")
            .arg("now")
            .spawn()
            .map_err(|e| format!("Shutdown failed: {}", e))?;
        Ok("Shutting down...".to_string())
    }
}

pub fn cmd_reboot(_args: &[String], _ctx: &mut CommandContext) -> Result<String, String> {
    #[cfg(windows)]
    {
        Command::new("shutdown")
            .args(&["/r", "/t", "0"])
            .spawn()
            .map_err(|e| format!("Reboot failed: {}", e))?;
        Ok("Rebooting...".to_string())
    }
    #[cfg(not(windows))]
    {
        Command::new("reboot")
            .spawn()
            .map_err(|e| format!("Reboot failed: {}", e))?;
        Ok("Rebooting...".to_string())
    }
}

pub fn cmd_startup(args: &[String], _ctx: &mut CommandContext) -> Result<String, String> {
    if args.len() < 2 {
        return Err("Usage: startup <add|remove|list> [name] [path]".to_string());
    }
    let subcommand = &args[1];
    match subcommand.as_str() {
        "add" => cmd_startup_add(&args[2..]),
        "remove" => cmd_startup_remove(&args[2..]),
        "list" => cmd_startup_list(),
        _ => Err(format!("Unknown startup subcommand: {}", subcommand)),
    }
}

fn cmd_startup_add(args: &[String]) -> Result<String, String> {
    if args.is_empty() {
        return Err("Usage: startup add <name> [path]".to_string());
    }
    let name = &args[0];
    let path = if args.len() > 1 {
        args[1].clone()
    } else {
        // Ищем программу в PATH
        find_in_path(name).ok_or_else(|| format!("Program not found in PATH: {}", name))?
    };

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let run_key = hkcu.open_subkey_with_flags(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
        KEY_WRITE,
    )
    .map_err(|e| format!("Failed to open registry key: {}", e))?;

    run_key
        .set_value(name, &path)
        .map_err(|e| format!("Failed to set registry value: {}", e))?;

    Ok(format!("✅ Добавлено в автозагрузку: {} -> {}", name, path))
}

fn cmd_startup_remove(args: &[String]) -> Result<String, String> {
    if args.is_empty() {
        return Err("Usage: startup remove <name>".to_string());
    }
    let name = &args[0];

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let run_key = hkcu.open_subkey_with_flags(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
        KEY_WRITE,
    )
    .map_err(|e| format!("Failed to open registry key: {}", e))?;

    match run_key.delete_value(name) {
        Ok(_) => Ok(format!("✅ Удалено из автозагрузки: {}", name)),
        Err(e) => Err(format!("Failed to delete value: {}", e)),
    }
}

fn cmd_startup_list() -> Result<String, String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let run_key = hkcu.open_subkey_with_flags(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
        KEY_READ,
    )
    .map_err(|e| format!("Failed to open registry key: {}", e))?;

    let entries: Vec<_> = run_key.enum_values().collect();
    if entries.is_empty() {
        return Ok("📋 Автозагрузка пуста".to_string());
    }

    let mut result = "📋 Автозагрузка:\n".to_string();
    for entry in entries {
        match entry {
            Ok((name, _value)) => result.push_str(&format!("   - {}\n", name)),
            Err(e) => result.push_str(&format!("   - Ошибка чтения: {}\n", e)),
        }
    }
    Ok(result)
}

fn find_in_path(program: &str) -> Option<String> {
    // Если путь уже абсолютный или содержит слеши
    if program.contains('\\') || program.contains('/') {
        if Path::new(program).exists() {
            return Some(program.to_string());
        } else {
            return None;
        }
    }

    // Ищем в PATH
    if let Some(paths) = env::var_os("PATH") {
        for path in env::split_paths(&paths) {
            let full_path = path.join(program);
            if full_path.exists() {
                return Some(full_path.to_string_lossy().to_string());
            }
            // Пробуем с .exe
            let full_path_exe = path.join(format!("{}.exe", program));
            if full_path_exe.exists() {
                return Some(full_path_exe.to_string_lossy().to_string());
            }
        }
    }
    None
}

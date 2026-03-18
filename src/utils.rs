use std::env;
use std::path::{Path, PathBuf};

pub fn search_in_path(program: &str) -> Option<PathBuf> {
    if program.contains('\\') || program.contains('/') {
        let path = PathBuf::from(program);
        if path.exists() {
            return Some(path);
        }
    }

    let program_exe = if !program.ends_with(".exe") {
        format!("{}.exe", program)
    } else {
        program.to_string()
    };

    if let Some(paths) = env::var_os("PATH") {
        for path in env::split_paths(&paths) {
            // пробуем как есть
            let full_path = path.join(program);
            if full_path.exists() {
                return Some(full_path);
            }
            // пробуем с .exe
            let full_path_exe = path.join(&program_exe);
            if full_path_exe.exists() {
                return Some(full_path_exe);
            }
        }
    }
    None
}
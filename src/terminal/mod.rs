pub mod context;
pub use context::CommandContext;

use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

pub struct TerminalState {
    pub current_dir: PathBuf,
    pub aliases: HashMap<String, String>,
    pub env_vars: HashMap<String, String>,
}

impl TerminalState {
    pub fn new() -> Self {
        let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let env_vars = env::vars().collect();
        TerminalState {
            current_dir,
            aliases: HashMap::new(),
            env_vars,
        }
    }

    pub fn set_env(&mut self, key: String, value: String) {
        self.env_vars.insert(key.clone(), value.clone());
        env::set_var(key, value);
    }

    pub fn get_env(&self, key: &str) -> Option<&String> {
        self.env_vars.get(key)
    }
}

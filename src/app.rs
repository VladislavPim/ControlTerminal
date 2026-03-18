use crate::commands::CommandRegistry;
use crate::logger::Logger;
use crate::terminal::{CommandContext, TerminalState};
use eframe::egui;
use egui::{Color32, ScrollArea, TextEdit};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::PathBuf;

// ---------- Структура конфигурации ----------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub bgcolor: Option<String>,
    pub fgcolor: Option<String>,
    pub aliases: Option<HashMap<String, String>>,
    pub env: Option<HashMap<String, String>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bgcolor: Some("gray".to_string()),
            fgcolor: Some("white".to_string()),
            aliases: Some(HashMap::new()),
            env: Some(HashMap::new()),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".controlconfig");

        if !config_path.exists() {
            let default = Config::default();
            let _ = default.save();
            return default;
        }

        match fs::read_to_string(&config_path) {
            Ok(content) => {
                toml::from_str(&content).unwrap_or_else(|e| {
                    eprintln!("Error parsing config: {}", e);
                    Config::default()
                })
            }
            Err(e) => {
                eprintln!("Error reading config: {}", e);
                Config::default()
            }
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let config_path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".controlconfig");
        let toml_string = toml::to_string(self).map_err(|e| e.to_string())?;
        fs::write(config_path, toml_string).map_err(|e| e.to_string())?;
        Ok(())
    }
}

// ---------- Структура одной вкладки ----------
pub struct TerminalTab {
    pub output: VecDeque<String>,
    pub input: String,
    pub history: Vec<String>,
    pub history_index: Option<usize>,
    pub current_dir: PathBuf,
    pub title: String,
    pub editing_title: bool,
    pub temp_title: String,
}

impl TerminalTab {
    pub fn new(initial_dir: PathBuf) -> Self {
        let dir_display = initial_dir.display().to_string();
        Self {
            output: VecDeque::new(),
            input: String::new(),
            history: Vec::new(),
            history_index: None,
            current_dir: initial_dir,
            title: dir_display,
            editing_title: false,
            temp_title: String::new(),
        }
    }

    pub fn add_output(&mut self, line: String) {
        self.output.push_back(line);
        if self.output.len() > 1000 {
            self.output.pop_front();
        }
    }
}

// ---------- Главная структура приложения ----------
pub struct TerminalApp {
    tabs: Vec<TerminalTab>,
    active_tab: usize,
    bg_color: Color32,
    fg_color: Color32,
    registry: CommandRegistry,
    logger: Logger,
    should_exit: bool,
    focus_requested: bool,
    input_id: Option<egui::Id>,
    config: Config,
    global_aliases: HashMap<String, String>,
    global_env: HashMap<String, String>,
    pending_command: Option<String>,
}

// Вспомогательная функция для парсинга цвета
fn parse_color_from_config(name: &str) -> Option<Color32> {
    match name.to_lowercase().as_str() {
        "black" => Some(Color32::BLACK),
        "white" => Some(Color32::WHITE),
        "red" => Some(Color32::RED),
        "green" => Some(Color32::GREEN),
        "blue" => Some(Color32::BLUE),
        "gray" | "grey" => Some(Color32::GRAY),
        "darkgray" => Some(Color32::DARK_GRAY),
        "lightgray" => Some(Color32::LIGHT_GRAY),
        "yellow" => Some(Color32::YELLOW),
        "cyan" => Some(Color32::from_rgb(0, 255, 255)),
        "magenta" => Some(Color32::from_rgb(255, 0, 255)),
        _ => None,
    }
}

impl Default for TerminalApp {
    fn default() -> Self {
        let config = Config::load();

        let bg_color = config
            .bgcolor
            .as_deref()
            .and_then(parse_color_from_config)
            .unwrap_or(Color32::from_gray(128));
        let fg_color = config
            .fgcolor
            .as_deref()
            .and_then(parse_color_from_config)
            .unwrap_or(Color32::WHITE);

        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let mut app = Self {
            tabs: vec![TerminalTab::new(home)],
            active_tab: 0,
            bg_color,
            fg_color,
            registry: CommandRegistry::new(),
            logger: Logger::new(),
            should_exit: false,
            focus_requested: false,
            input_id: None,
            config,
            global_aliases: HashMap::new(),
            global_env: HashMap::new(),
            pending_command: None,
        };

        // Загружаем алиасы из конфига в глобальные
        if let Some(aliases) = &app.config.aliases {
            for (k, v) in aliases {
                app.global_aliases.insert(k.clone(), v.clone());
            }
        }

        // Загружаем переменные окружения
        if let Some(env_vars) = &app.config.env {
            for (k, v) in env_vars {
                app.global_env.insert(k.clone(), v.clone());
                std::env::set_var(k, v);
            }
        }

        // Приветственное сообщение в первой вкладке
        app.tabs[0].add_output("Control Terminal v1.1".to_string());
        app.tabs[0].add_output("You are using the stable version.".to_string());
        app.tabs[0].add_output("Good luck!".to_string());
        app.tabs[0].add_output("For a full list of commands, type 'help'.".to_string());
        app.tabs[0].add_output(String::new());

        app
    }
}

impl TerminalApp {
    pub fn new() -> Self {
        Self::default()
    }

    fn save_config(&mut self) {
        self.config.bgcolor = Some(color_to_string(self.bg_color));
        self.config.fgcolor = Some(color_to_string(self.fg_color));
        self.config.aliases = Some(self.global_aliases.clone());
        self.config.env = Some(self.global_env.clone());
        let _ = self.config.save();
    }

    fn new_tab(&mut self) {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let mut new_tab = TerminalTab::new(home);
        new_tab.add_output("Control Terminal v1.1".to_string());
        new_tab.add_output("New tab opened.".to_string());
        new_tab.add_output(String::new());
        self.tabs.push(new_tab);
        self.active_tab = self.tabs.len() - 1;
    }

    fn close_tab(&mut self, index: usize) {
        if self.tabs.len() <= 1 {
            return;
        }
        self.tabs.remove(index);
        if self.active_tab >= index && self.active_tab > 0 {
            self.active_tab -= 1;
        }
    }

    fn execute_command(&mut self, command_line: &str) {
    self.logger.log_command(command_line);
    let active_idx = self.active_tab;
    self.tabs[active_idx].history.push(command_line.to_string());
    self.tabs[active_idx].history_index = None;

    let trimmed = command_line.trim();
    if trimmed.is_empty() {
        return;
    }

    let prompt = format!("{}> ", self.tabs[active_idx].current_dir.display());
    self.tabs[active_idx].add_output(format!("{}{}", prompt, command_line));

    let mut parts: Vec<String> = trimmed.split_whitespace().map(String::from).collect();
    if parts.is_empty() {
        return;
    }

    // Проверка на глобальные алиасы
    if let Some(alias_value) = self.global_aliases.get(&parts[0]) {
        let mut new_parts: Vec<String> = alias_value.split_whitespace().map(String::from).collect();
        new_parts.extend(parts.into_iter().skip(1));
        parts = new_parts;
    }

    let cmd_name = parts[0].clone();
    let args = parts;

    // Проверяем, есть ли такая встроенная команда
    if let Some(cmd_fn) = self.registry.get(&cmd_name) {
        // Встроенная команда
        let mut state = TerminalState {
            current_dir: self.tabs[active_idx].current_dir.clone(),
            aliases: self.global_aliases.clone(),
            env_vars: self.global_env.clone(),
        };

        let mut ctx = CommandContext {
            state: &mut state,
            bg_color: &mut self.bg_color,
            fg_color: &mut self.fg_color,
            logger: &mut self.logger,
        };

        match cmd_fn(&args, &mut ctx) {
            Ok(output) => {
                if output == "\x18EXIT\x18" {
                    self.should_exit = true;
                } else if output == "\x1b[2J\x1b[1;1H" {
                    self.tabs[active_idx].output.clear();
                } else {
                    for line in output.lines() {
                        self.tabs[active_idx].add_output(line.to_string());
                    }
                }
                self.tabs[active_idx].current_dir = state.current_dir;
                self.global_aliases = state.aliases;
                self.global_env = state.env_vars;
                self.save_config();
            }
            Err(err) => {
                self.tabs[active_idx].add_output(format!("Error: {}", err));
            }
        }
    } else {
        // Не встроенная команда – пробуем выполнить как внешнюю программу
        match crate::utils::search_in_path(&cmd_name) {
            Some(path) => {
                let output = std::process::Command::new(&path)
                    .args(&args[1..]) // все аргументы, кроме имени команды
                    .current_dir(&self.tabs[active_idx].current_dir)
                    .output()
                    .map_err(|e| format!("Failed to execute: {}", e));

                match output {
                    Ok(output) => {
                        let mut result = String::new();
                        if !output.stdout.is_empty() {
                            result.push_str(&String::from_utf8_lossy(&output.stdout));
                        }
                        if !output.stderr.is_empty() {
                            if !result.is_empty() { result.push('\n'); }
                            result.push_str(&String::from_utf8_lossy(&output.stderr));
                        }
                        if result.is_empty() {
                            result = format!("Program exited with status: {}", output.status);
                        }
                        for line in result.lines() {
                            self.tabs[active_idx].add_output(line.to_string());
                        }
                    }
                    Err(e) => {
                        self.tabs[active_idx].add_output(format!("Error: {}", e));
                    }
                }
            }
            None => {
                // Ничего не нашли – выдаём ошибку
                self.tabs[active_idx].add_output(format!("Unknown command: {}", cmd_name));
            }
        }
    }
    self.tabs[active_idx].add_output(String::new()); // пустая строка между командами
}
}

impl eframe::App for TerminalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.should_exit {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(self.bg_color))
            .show(ctx, |ui| {
                ui.visuals_mut().override_text_color = Some(self.fg_color);

                // ----- Панель вкладок с белым фоном и чёрным текстом -----
                ui.horizontal(|ui| {
                    if ui.button(" + ").clicked() {
                        self.new_tab();
                    }

                    let tabs_len = self.tabs.len();
                    let mut to_close = None;

                    for (i, tab) in self.tabs.iter_mut().enumerate() {
                        let is_active = i == self.active_tab;

                        let (bg_fill, text_color) = if is_active {
                            (Color32::from_gray(220), Color32::BLACK)
                        } else {
                            (Color32::WHITE, Color32::BLACK)
                        };

                        ui.scope(|ui| {
                            ui.visuals_mut().override_text_color = Some(text_color);
                            ui.style_mut().visuals.widgets.inactive.bg_fill = bg_fill;
                            ui.style_mut().visuals.widgets.hovered.bg_fill = Color32::from_gray(200);
                            ui.style_mut().visuals.widgets.active.bg_fill = Color32::from_gray(210);

                            if tab.editing_title {
                                let response = ui.text_edit_singleline(&mut tab.temp_title);
                                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                    if !tab.temp_title.is_empty() {
                                        tab.title = tab.temp_title.clone();
                                    }
                                    tab.editing_title = false;
                                }
                                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                                    tab.editing_title = false;
                                }
                            } else {
                                let label = format!(" 📁 {} ", tab.title);
                                let button = ui.button(label);
                                if button.clicked() {
                                    self.active_tab = i;
                                }
                                if button.double_clicked() {
                                    tab.editing_title = true;
                                    tab.temp_title = tab.title.clone();
                                }
                            }
                        });

                        if tabs_len > 1 && ui.button(" ✕ ").clicked() {
                            to_close = Some(i);
                        }
                    }

                    if let Some(index) = to_close {
                        self.close_tab(index);
                    }
                });

                ui.separator();

                // ----- Содержимое активной вкладки -----
                let active_idx = self.active_tab;
                let mut cmd_to_execute = None;

                {
                    let active_tab = &mut self.tabs[active_idx];

                    ScrollArea::vertical()
                        .stick_to_bottom(true)
                        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());

                            for line in &active_tab.output {
                                ui.label(line);
                            }

                            ui.add_space(2.0);

                            let response = ui.horizontal(|ui| {
                                let prompt = format!("{}> ", active_tab.current_dir.display());
                                ui.label(prompt);

                                let text_edit = TextEdit::singleline(&mut active_tab.input)
                                    .desired_width(f32::INFINITY)
                                    .text_color(self.fg_color)
                                    .font(egui::TextStyle::Monospace)
                                    .frame(false);

                                ui.add(text_edit)
                            }).inner;

                            self.input_id = Some(response.id);

                            if !self.focus_requested {
                                ui.ctx().memory_mut(|mem| mem.request_focus(response.id));
                                self.focus_requested = true;
                            }

                            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                let command = active_tab.input.trim().to_string();
                                if !command.is_empty() {
                                    cmd_to_execute = Some(command.clone());
                                    active_tab.input.clear();
                                }
                            }

                            if response.has_focus() {
                                let input_state = ui.input(|i| i.clone());
                                let ctrl_pressed = input_state.modifiers.ctrl;

                                if !ctrl_pressed {
                                    if input_state.key_pressed(egui::Key::ArrowUp) {
                                        if !active_tab.history.is_empty() {
                                            let new_index = match active_tab.history_index {
                                                None => Some(active_tab.history.len() - 1),
                                                Some(0) => Some(0),
                                                Some(i) => Some(i - 1),
                                            };
                                            if let Some(idx) = new_index {
                                                active_tab.input = active_tab.history[idx].clone();
                                                active_tab.history_index = new_index;
                                            }
                                        }
                                    } else if input_state.key_pressed(egui::Key::ArrowDown) {
                                        if let Some(idx) = active_tab.history_index {
                                            if idx + 1 < active_tab.history.len() {
                                                active_tab.input = active_tab.history[idx + 1].clone();
                                                active_tab.history_index = Some(idx + 1);
                                            } else {
                                                active_tab.input.clear();
                                                active_tab.history_index = None;
                                            }
                                        }
                                    }
                                }
                            }

                            ui.add_space(50.0);
                        });
                }

                if let Some(cmd) = cmd_to_execute {
                    self.execute_command(&cmd);
                }

                // Горячие клавиши для вкладок
                if ui.input(|i| i.key_pressed(egui::Key::T) && i.modifiers.ctrl) {
                    self.new_tab();
                }
                if ui.input(|i| i.key_pressed(egui::Key::W) && i.modifiers.ctrl) && self.tabs.len() > 1 {
                    self.close_tab(self.active_tab);
                }
                if ui.input(|i| i.key_pressed(egui::Key::Tab) && i.modifiers.ctrl && !i.modifiers.shift) {
                    self.active_tab = (self.active_tab + 1) % self.tabs.len();
                }
                if ui.input(|i| i.key_pressed(egui::Key::Tab) && i.modifiers.ctrl && i.modifiers.shift) {
                    self.active_tab = if self.active_tab == 0 {
                        self.tabs.len() - 1
                    } else {
                        self.active_tab - 1
                    };
                }

                // 🔥 Принудительный фокус на поле ввода, если не редактируется заголовок
                let editing = self.tabs.iter().any(|tab| tab.editing_title);
                if !editing {
                    if let Some(id) = self.input_id {
                        ctx.memory_mut(|mem| mem.request_focus(id));
                    }
                }
            });
    }
}

fn color_to_string(color: Color32) -> String {
    match color {
        Color32::BLACK => "black".to_string(),
        Color32::WHITE => "white".to_string(),
        Color32::RED => "red".to_string(),
        Color32::GREEN => "green".to_string(),
        Color32::BLUE => "blue".to_string(),
        Color32::GRAY => "gray".to_string(),
        Color32::DARK_GRAY => "darkgray".to_string(),
        Color32::LIGHT_GRAY => "lightgray".to_string(),
        Color32::YELLOW => "yellow".to_string(),
        _ if color == Color32::from_rgb(0, 255, 255) => "cyan".to_string(),
        _ if color == Color32::from_rgb(255, 0, 255) => "magenta".to_string(),
        _ => "white".to_string(),
    }
}

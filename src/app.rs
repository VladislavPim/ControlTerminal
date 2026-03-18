use crate::commands::CommandRegistry;
use crate::logger::Logger;
use crate::terminal::{CommandContext, TerminalState};
use eframe::egui
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

// ---------- Вспомогательная функция для парсинга цвета ----------
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

// ---------- Главная структура приложения ----------
pub struct TerminalApp {
    output: VecDeque<String>,
    input: String,
    history: Vec<String>,
    history_index: Option<usize>,
    state: TerminalState,
    registry: CommandRegistry,
    bg_color: Color32,
    fg_color: Color32,
    logger: Logger,
    should_exit: bool,
    input_id: Option<egui::Id>,
    focus_requested: bool,      // добавляем
    scroll_to_bottom: bool,     // добавляем
    config: Config,
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

        let mut app = Self {
            output: VecDeque::new(),
            input: String::new(),
            history: Vec::new(),
            history_index: None,
            state: TerminalState::new(),
            registry: CommandRegistry::new(),
            bg_color,
            fg_color,
            logger: Logger::new(),
            should_exit: false,
            input_id: None,
            focus_requested: false,
            scroll_to_bottom: true,
            config,
        };

        // Загружаем алиасы из конфига
        if let Some(aliases) = &app.config.aliases {
            for (k, v) in aliases {
                app.state.aliases.insert(k.clone(), v.clone());
            }
        }

        // Загружаем переменные окружения из конфига
        if let Some(env_vars) = &app.config.env {
            for (k, v) in env_vars {
                app.state.set_env(k.clone(), v.clone());
            }
        }

        // Приветственное сообщение при запуске
        // Приветственное сообщение при запуске
app.add_output("Control Terminal v1.0.0".to_string());
app.add_output("You are using the stable version.".to_string());
app.add_output("Good luck!".to_string());
app.add_output("For a full list of commands, type 'help'.".to_string());
app.add_output(String::new()); // пустая строка для отступа

        app
    }
}

impl TerminalApp {
    pub fn new() -> Self {
        Self::default()
    }

    fn add_output(&mut self, line: String) {
        self.output.push_back(line);
        if self.output.len() > 1000 {
            self.output.pop_front();
        }
    }

    fn save_config(&mut self) {
        // Обновляем цвета в конфиге
        self.config.bgcolor = Some(color_to_string(self.bg_color));
        self.config.fgcolor = Some(color_to_string(self.fg_color));

        // Алиасы
        let mut aliases_map = HashMap::new();
        for (k, v) in &self.state.aliases {
            aliases_map.insert(k.clone(), v.clone());
        }
        self.config.aliases = Some(aliases_map);

        // Переменные окружения
        let mut env_map = HashMap::new();
        for (k, v) in &self.state.env_vars {
            env_map.insert(k.clone(), v.clone());
        }
        self.config.env = Some(env_map);

        let _ = self.config.save();
    }

    fn execute_command(&mut self, command_line: &str) {
        self.logger.log_command(command_line);
        self.history.push(command_line.to_string());
        self.history_index = None;

        let trimmed = command_line.trim();
        if trimmed.is_empty() {
            return;
        }

        let prompt = format!("{}> ", self.state.current_dir.display());
        self.add_output(format!("{}{}", prompt, command_line));

        let mut parts: Vec<String> = trimmed.split_whitespace().map(String::from).collect();
        if parts.is_empty() {
            return;
        }

        // Проверка на алиас
        if let Some(alias_value) = self.state.aliases.get(&parts[0]) {
            let mut new_parts: Vec<String> = alias_value.split_whitespace().map(String::from).collect();
            new_parts.extend(parts.into_iter().skip(1));
            parts = new_parts;
        }

        let cmd_name = parts[0].clone();
        let args = parts;

        let cmd_fn = match self.registry.get(&cmd_name) {
            Some(f) => f,
            None => {
                self.add_output(format!("Unknown command: {}", cmd_name));
                return;
            }
        };

        let mut ctx = CommandContext {
            state: &mut self.state,
            bg_color: &mut self.bg_color,
            fg_color: &mut self.fg_color,
            logger: &mut self.logger,
        };

        match cmd_fn(&args, &mut ctx) {
            Ok(output) => {
                if output == "\x18EXIT\x18" {
                    self.should_exit = true;
                } else if output == "\x1b[2J\x1b[1;1H" {
                    self.output.clear();
                } else {
                    for line in output.lines() {
                        self.add_output(line.to_string());
                    }
                }
            }
            Err(err) => {
                self.add_output(format!("Error: {}", err));
            }
        }
        self.add_output(String::new()); // пустая строка между командами

        // Сохраняем конфиг (на случай, если команды bgcolor/fgcolor/alias/set изменили состояние)
        self.save_config();
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

                ScrollArea::vertical()
                    .stick_to_bottom(true)
                    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());

                        // Вывод истории
                        for line in &self.output {
                            ui.label(line);
                        }

                        ui.add_space(2.0);

                        // Строка ввода
                        let response = ui.horizontal(|ui| {
                            let prompt = format!("{}> ", self.state.current_dir.display());
                            ui.label(prompt);

                            let text_edit = TextEdit::singleline(&mut self.input)
                                .desired_width(f32::INFINITY)
                                .text_color(self.fg_color)
                                .font(egui::TextStyle::Monospace)
                                .frame(false);

                            ui.add(text_edit)
                        }).inner;

                        self.input_id = Some(response.id);

                        // Обработка Enter
                        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            let command = self.input.trim().to_string();
                            if !command.is_empty() {
                                self.execute_command(&command);
                                self.input.clear();
                            }
                        }

                        // Навигация по истории (только если не зажат Ctrl)
                        if response.has_focus() {
                            let input_state = ui.input(|i| i.clone());
                            let ctrl_pressed = input_state.modifiers.ctrl;

                            if !ctrl_pressed {
                                if input_state.key_pressed(egui::Key::ArrowUp) {
                                    if !self.history.is_empty() {
                                        let new_index = match self.history_index {
                                            None => Some(self.history.len() - 1),
                                            Some(0) => Some(0),
                                            Some(i) => Some(i - 1),
                                        };
                                        if let Some(idx) = new_index {
                                            self.input = self.history[idx].clone();
                                            self.history_index = new_index;
                                        }
                                    }
                                } else if input_state.key_pressed(egui::Key::ArrowDown) {
                                    if let Some(idx) = self.history_index {
                                        if idx + 1 < self.history.len() {
                                            self.input = self.history[idx + 1].clone();
                                            self.history_index = Some(idx + 1);
                                        } else {
                                            self.input.clear();
                                            self.history_index = None;
                                        }
                                    }
                                }
                            }
                        }

                        // Пустое пространство для прокрутки вниз
                        ui.add_space(50.0);
                    });
            });

        // Принудительный фокус на поле ввода всегда
        if let Some(id) = self.input_id {
            ctx.memory_mut(|mem| mem.request_focus(id));
        } else {
            ctx.request_repaint();
        }
    }
}

// Вспомогательная функция для преобразования Color32 в строку цвета (для сохранения в конфиг)
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

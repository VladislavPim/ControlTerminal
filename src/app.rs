use crate::commands::CommandRegistry;
use crate::logger::Logger;
use crate::terminal::{CommandContext, TerminalState};
use eframe::egui;
use egui::{Color32, ScrollArea, TextEdit};
use std::collections::VecDeque;

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
}

impl Default for TerminalApp {
    fn default() -> Self {
        Self {
            output: VecDeque::new(),
            input: String::new(),
            history: Vec::new(),
            history_index: None,
            state: TerminalState::new(),
            registry: CommandRegistry::new(),
            bg_color: Color32::from_gray(128),
            fg_color: Color32::WHITE,
            logger: Logger::new(),
            should_exit: false,
            input_id: None,
        }
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
                    .stick_to_bottom(true) // автоматически оставаться внизу, если пользователь не прокрутил вверх
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

                        // Навигация по истории (стрелки вверх/вниз)
                        // Навигация по истории (только если не зажат Ctrl)
if response.has_focus() {
    let input_state = ui.input(|i| i.clone());
    
    // Проверяем, зажат ли Ctrl
    let ctrl_pressed = input_state.modifiers.ctrl;
    
    if !ctrl_pressed {
        // Стрелки работают только без Ctrl
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
    // Если Ctrl зажат — ничего не делаем, пусть TextEdit сам обрабатывает
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
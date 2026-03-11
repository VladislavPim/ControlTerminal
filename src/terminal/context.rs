use crate::logger::Logger;
use crate::terminal::TerminalState;
use egui::Color32;

pub struct CommandContext<'a> {
    pub state: &'a mut TerminalState,
    pub bg_color: &'a mut Color32,
    pub fg_color: &'a mut Color32,
    pub logger: &'a mut Logger,
}
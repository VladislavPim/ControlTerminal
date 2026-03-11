#![windows_subsystem = "windows"]

mod app;
mod commands;
mod logger;
mod terminal;

use app::TerminalApp;
use eframe::egui;

fn load_icon() -> Option<egui::IconData> {
    // Встраиваем файл иконки в бинарник
    let icon_bytes = include_bytes!("../assets/logo.ico");
    // Загружаем изображение из памяти
    let image = image::load_from_memory(icon_bytes).ok()?.into_rgba8();
    let (width, height) = image.dimensions();
    Some(egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    })
}

fn main() -> Result<(), eframe::Error> {
    let icon = load_icon();
    
    // Строим ViewportBuilder поэтапно
    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([800.0, 600.0])
        .with_resizable(true)
        .with_active(true);
    
    // Если иконка загружена, добавляем её
    if let Some(icon_data) = icon {
        viewport = viewport.with_icon(icon_data);
    }
    // Метод with_taskbar_icon не нужен — иконка окна автоматически появляется на панели задач

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    
    eframe::run_native(
        "Control Terminal",
        options,
        Box::new(|_cc| Box::new(TerminalApp::new())),
    )
}
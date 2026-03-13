extern crate winres;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        
        // Иконка
        res.set_icon("assets/logo.ico");
        
        res.set("FileVersion", "1.0.0.0");
        res.set("ProductVersion", "1.0.0.0");
        res.set("CompanyName", "VladislavPim");
        res.set("FileDescription", "Control Terminal");
        res.set("ProductName", "ControlTerminal");
        res.set("LegalCopyright", "© 2026 VladislavPim");
        res.set("OriginalFilename", "ControlTerminal.exe");
        res.set("InternalName", "ControlTerminal");
        
        // Язык (английский США)
        res.set_language(0x0409);
        
        res.compile().unwrap();
    }
}

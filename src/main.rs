use eframe::egui;

struct Rust2048;

impl Default for Rust2048 {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for Rust2048 {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");

            if ui.button("Click each year").clicked() {
                println!("Button click registered");
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "rust 2048",
        native_options,
        Box::new(|_cc| Box::<Rust2048>::default()),
    )
}

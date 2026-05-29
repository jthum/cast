use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "eframe Baseline",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<BaselineApp>::default())),
    )
}

#[derive(Default)]
struct BaselineApp;

impl eframe::App for BaselineApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("eframe baseline");
        ui.label("Minimal native window for memory benchmarking.");
    }
}

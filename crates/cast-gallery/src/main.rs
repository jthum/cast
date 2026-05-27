use cast::{
    Badge, Button, Card, CastTheme, Intent, ThemeMode, apply_theme,
    egui::{self, CentralPanel, Panel, RichText},
};

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Cast Gallery",
        native_options,
        Box::new(|cc| {
            let app = CastGallery::new();
            apply_theme(&cc.egui_ctx, &app.theme);
            Ok(Box::new(app))
        }),
    )
}

struct CastGallery {
    theme: CastTheme,
    mode: ThemeMode,
    search: String,
    enabled: bool,
}

impl CastGallery {
    fn new() -> Self {
        Self {
            theme: CastTheme::light(),
            mode: ThemeMode::Light,
            search: String::new(),
            enabled: true,
        }
    }

    fn set_mode(&mut self, ctx: &egui::Context, mode: ThemeMode) {
        self.mode = mode;
        self.theme = match mode {
            ThemeMode::Light => CastTheme::light(),
            ThemeMode::Dark => CastTheme::dark(),
        };
        cast::set_theme(ctx, self.theme.clone());
    }
}

impl eframe::App for CastGallery {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        Panel::top("top_bar").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Cast Gallery");
                ui.separator();
                ui.selectable_value(&mut self.mode, ThemeMode::Light, "Light");
                ui.selectable_value(&mut self.mode, ThemeMode::Dark, "Dark");
                if self.mode != self.theme.mode {
                    self.set_mode(&ctx, self.mode);
                }
            });
        });

        Panel::left("sidebar")
            .resizable(false)
            .default_size(220.0)
            .show_inside(ui, |ui| {
                ui.heading("Components");
                ui.separator();
                ui.label("Foundation");
                ui.label("Button");
                ui.label("Badge");
                ui.label("Card");
                ui.label("Inputs");
            });

        CentralPanel::default().show_inside(ui, |ui| {
            ui.heading(RichText::new("Foundations").size(24.0));
            ui.label("Runtime theme switching, semantic tokens, and initial primitives.");
            ui.add_space(12.0);

            Card::new().show(ui, |ui| {
                ui.heading("Buttons and badges");
                ui.horizontal_wrapped(|ui| {
                    ui.add(Button::new("Approve"));
                    ui.add(Badge::new("Running").intent(Intent::Info));
                    ui.add(Badge::new("Success").intent(Intent::Success));
                    ui.add(Badge::new("Warning").intent(Intent::Warning));
                    ui.add(Badge::new("Danger").intent(Intent::Danger));
                });
            });

            ui.add_space(12.0);

            Card::new().show(ui, |ui| {
                ui.heading("Form controls");
                ui.horizontal(|ui| {
                    ui.label("Search");
                    ui.text_edit_singleline(&mut self.search);
                });
                ui.checkbox(&mut self.enabled, "Enabled");
            });
        });
    }
}

use cast::{
    Badge, Button, Card, CastTheme, Intent, Size, ThemeMode, Variant,
    egui::{self, CentralPanel, Panel, RichText},
};

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Cast Gallery",
        native_options,
        Box::new(|cc| {
            let app = CastGallery::new();
            cast::set_theme(&cc.egui_ctx, app.theme.clone());
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

            show_theme_foundation(ui);
            ui.add_space(12.0);
            show_buttons_and_badges(ui);
            ui.add_space(12.0);
            show_raw_egui_controls(ui, &mut self.search, &mut self.enabled);
        });
    }
}

fn show_theme_foundation(ui: &mut egui::Ui) {
    Card::new().show(ui, |ui| {
        ui.heading("Theme boundary");
        ui.label("Cast widgets resolve semantic styling from CastTheme.");
        ui.label("Raw egui widgets inherit the derived egui::Style fallback.");
        ui.horizontal_wrapped(|ui| {
            ui.add(Badge::new("CastTheme").intent(Intent::Primary));
            ui.add(Badge::new("egui::Style fallback").intent(Intent::Neutral));
            ui.add(Badge::new("runtime switching").intent(Intent::Info));
        });
    });
}

fn show_buttons_and_badges(ui: &mut egui::Ui) {
    Card::new().show(ui, |ui| {
        ui.heading("Buttons");
        ui.horizontal_wrapped(|ui| {
            ui.add(Button::new("Primary"));
            ui.add(Button::new("Success").intent(Intent::Success));
            ui.add(Button::new("Warning").intent(Intent::Warning));
            ui.add(Button::new("Danger").intent(Intent::Danger));
            ui.add(
                Button::new("Outline")
                    .intent(Intent::Primary)
                    .variant(Variant::Outline),
            );
            ui.add(
                Button::new("Ghost")
                    .intent(Intent::Primary)
                    .variant(Variant::Ghost),
            );
        });

        ui.add_space(8.0);
        ui.heading("Sizes");
        ui.horizontal_wrapped(|ui| {
            ui.add(Button::new("Small").size(Size::Small));
            ui.add(Button::new("Medium").size(Size::Medium));
            ui.add(Button::new("Large").size(Size::Large));
        });

        ui.add_space(8.0);
        ui.heading("Badges");
        ui.horizontal_wrapped(|ui| {
            ui.add(Badge::new("Neutral"));
            ui.add(Badge::new("Primary").intent(Intent::Primary));
            ui.add(Badge::new("Success").intent(Intent::Success));
            ui.add(Badge::new("Warning").intent(Intent::Warning));
            ui.add(Badge::new("Danger").intent(Intent::Danger));
            ui.add(Badge::new("Info").intent(Intent::Info));
            ui.add(
                Badge::new("Outline")
                    .intent(Intent::Primary)
                    .variant(Variant::Outline),
            );
        });
    });
}

fn show_raw_egui_controls(ui: &mut egui::Ui, search: &mut String, enabled: &mut bool) {
    Card::new().show(ui, |ui| {
        ui.heading("Raw egui controls");
        ui.horizontal(|ui| {
            ui.label("Search");
            ui.text_edit_singleline(search);
        });
        ui.checkbox(enabled, "Enabled");
        ui.horizontal_wrapped(|ui| {
            let _ = ui.button("egui button");
            ui.hyperlink_to("egui link", "https://github.com/emilk/egui");
        });
    });
}

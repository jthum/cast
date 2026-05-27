use cast::{
    Alert, Badge, Button, Card, CastPaletteInput, CastTheme, Checkbox, Intent, Label, Link, Notice,
    Panel as CastPanel, SearchInput, SemanticColorTokens, Separator, Size, Switch, TextInput,
    ThemeMode, ThemeSeed, Variant,
    egui::{self, CentralPanel, Color32, Panel as EguiPanel, RichText},
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
    seed: ThemeSeed,
    search: String,
    name: String,
    enabled: bool,
    notifications: bool,
    indeterminate: bool,
}

impl CastGallery {
    fn new() -> Self {
        let mode = ThemeMode::Light;
        let seed = ThemeSeed::for_mode(mode);
        let theme = seed.clone().resolve();

        Self {
            theme,
            seed,
            search: String::new(),
            name: String::from("Cast"),
            enabled: true,
            notifications: true,
            indeterminate: false,
        }
    }

    fn set_mode(&mut self, ctx: &egui::Context, mode: ThemeMode) {
        self.seed.mode = mode;
        self.apply_theme(ctx);
    }

    fn apply_theme(&mut self, ctx: &egui::Context) {
        self.theme = self.seed.clone().resolve();
        cast::set_theme(ctx, self.theme.clone());
    }
}

impl eframe::App for CastGallery {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        EguiPanel::top("top_bar").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Cast Gallery");
                ui.separator();
                let mut mode = self.seed.mode;
                ui.selectable_value(&mut mode, ThemeMode::Light, "Light");
                ui.selectable_value(&mut mode, ThemeMode::Dark, "Dark");
                if mode != self.seed.mode {
                    self.set_mode(&ctx, mode);
                }
            });
        });

        EguiPanel::left("sidebar")
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
                ui.separator();
                if show_theme_editor(ui, &mut self.seed) {
                    self.apply_theme(&ctx);
                }
            });

        CentralPanel::default().show_inside(ui, |ui| {
            ui.heading(RichText::new("Foundations").size(24.0));
            ui.label("Runtime theme switching, live palette editing, semantic tokens, and initial primitives.");
            ui.add_space(12.0);

            show_theme_foundation(ui);
            ui.add_space(12.0);
            show_palette_preview(ui, &self.theme);
            ui.add_space(12.0);
            show_buttons_and_badges(ui);
            ui.add_space(12.0);
            show_surfaces(ui);
            ui.add_space(12.0);
            show_text_and_feedback(ui);
            ui.add_space(12.0);
            show_forms(
                ui,
                &mut self.search,
                &mut self.name,
                &mut self.enabled,
                &mut self.notifications,
                &mut self.indeterminate,
            );
            ui.add_space(12.0);
            show_raw_egui_controls(ui, &mut self.search, &mut self.enabled);
        });
    }
}

fn show_theme_editor(ui: &mut egui::Ui, seed: &mut ThemeSeed) -> bool {
    let mut changed = false;
    ui.heading("Theme");

    changed |= color_row(ui, "Primary", &mut seed.palette.primary);
    changed |= optional_color_row(
        ui,
        "Secondary",
        &mut seed.palette.secondary,
        CastPaletteInput::default_for(seed.mode)
            .secondary
            .unwrap_or(Color32::from_rgb(124, 58, 237)),
    );
    changed |= optional_color_row(
        ui,
        "Neutral",
        &mut seed.palette.neutral,
        CastPaletteInput::default_for(seed.mode)
            .neutral
            .unwrap_or(Color32::from_rgb(100, 116, 139)),
    );
    changed |= optional_color_row(
        ui,
        "Success",
        &mut seed.palette.success,
        CastPaletteInput::default_for(seed.mode)
            .success
            .unwrap_or(Color32::from_rgb(22, 163, 74)),
    );
    changed |= optional_color_row(
        ui,
        "Warning",
        &mut seed.palette.warning,
        CastPaletteInput::default_for(seed.mode)
            .warning
            .unwrap_or(Color32::from_rgb(217, 119, 6)),
    );
    changed |= optional_color_row(
        ui,
        "Danger",
        &mut seed.palette.danger,
        CastPaletteInput::default_for(seed.mode)
            .danger
            .unwrap_or(Color32::from_rgb(220, 38, 38)),
    );
    changed |= optional_color_row(
        ui,
        "Info",
        &mut seed.palette.info,
        CastPaletteInput::default_for(seed.mode)
            .info
            .unwrap_or(Color32::from_rgb(8, 145, 178)),
    );

    ui.separator();
    let spacing_changed = theme_slider(ui, "Spacing", &mut seed.spacing.md, 8.0..=20.0);
    changed |= spacing_changed;
    if spacing_changed {
        seed.spacing.xs = seed.spacing.md / 3.0;
        seed.spacing.sm = seed.spacing.md * 2.0 / 3.0;
        seed.spacing.lg = seed.spacing.md * 4.0 / 3.0;
        seed.spacing.xl = seed.spacing.md * 2.0;
    }
    let radius_changed = theme_slider(ui, "Radius", &mut seed.radius.md, 0.0..=16.0);
    changed |= radius_changed;
    if radius_changed {
        seed.radius.sm = (seed.radius.md - 2.0).max(0.0);
        seed.radius.lg = seed.radius.md + 2.0;
    }
    let stroke_changed = theme_slider(ui, "Border", &mut seed.stroke.sm, 0.0..=3.0);
    changed |= stroke_changed;
    if stroke_changed {
        seed.stroke.md = seed.stroke.sm + 0.5;
        seed.stroke.lg = seed.stroke.sm + 1.0;
    }
    let typography_changed = theme_slider(ui, "Text", &mut seed.typography.body.size, 12.0..=18.0);
    changed |= typography_changed;
    if typography_changed {
        seed.typography.small.size = seed.typography.body.size - 2.0;
        seed.typography.heading.size = seed.typography.body.size + 6.0;
    }
    let controls_changed = theme_slider(ui, "Control", &mut seed.controls.min_height, 26.0..=44.0);
    changed |= controls_changed;
    if controls_changed {
        seed.controls.padding_x = seed.controls.min_height * 0.375;
        seed.controls.padding_y = seed.controls.min_height * 0.22;
    }

    ui.horizontal(|ui| {
        if ui.button("Reset").clicked() {
            *seed = ThemeSeed::for_mode(seed.mode);
            changed = true;
        }
        if ui.button("Primary only").clicked() {
            seed.palette = CastPaletteInput::from_primary(seed.palette.primary);
            changed = true;
        }
    });

    changed
}

fn theme_slider(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut f32,
    range: std::ops::RangeInclusive<f32>,
) -> bool {
    ui.add(egui::Slider::new(value, range).text(label))
        .changed()
}

fn color_row(ui: &mut egui::Ui, label: &str, color: &mut Color32) -> bool {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.color_edit_button_srgba(color).changed()
    })
    .inner
}

fn optional_color_row(
    ui: &mut egui::Ui,
    label: &str,
    color: &mut Option<Color32>,
    fallback: Color32,
) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        let mut enabled = color.is_some();
        if ui.checkbox(&mut enabled, label).changed() {
            *color = enabled.then_some(fallback);
            changed = true;
        }

        if let Some(color) = color {
            changed |= ui.color_edit_button_srgba(color).changed();
        }
    });

    changed
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
            ui.add(Badge::new("secondary").intent(Intent::Secondary));
        });
    });
}

fn show_palette_preview(ui: &mut egui::Ui, theme: &CastTheme) {
    Card::new().show(ui, |ui| {
        ui.heading("Derived palette");
        palette_family_row(ui, "Primary", theme.colors.primary_family);
        palette_family_row(ui, "Secondary", theme.colors.secondary_family);
        palette_family_row(ui, "Success", theme.colors.success_family);
        palette_family_row(ui, "Warning", theme.colors.warning_family);
        palette_family_row(ui, "Danger", theme.colors.danger_family);
        palette_family_row(ui, "Info", theme.colors.info_family);
    });
}

fn palette_family_row(ui: &mut egui::Ui, label: &str, family: SemanticColorTokens) {
    ui.horizontal(|ui| {
        ui.label(label);
        color_swatch(ui, family.base, "base");
        color_swatch(ui, family.subtle, "subtle");
        color_swatch(ui, family.muted, "muted");
        color_swatch(ui, family.border, "border");
        color_swatch(ui, family.hover, "hover");
        color_swatch(ui, family.active, "active");
    });
}

fn color_swatch(ui: &mut egui::Ui, color: Color32, label: &str) {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(24.0, 18.0), egui::Sense::hover());
    ui.painter().rect_filled(rect, 3, color);
    response.on_hover_text(label);
}

fn show_buttons_and_badges(ui: &mut egui::Ui) {
    Card::new().show(ui, |ui| {
        ui.heading("Buttons");
        ui.horizontal_wrapped(|ui| {
            ui.add(Button::new("Primary"));
            ui.add(Button::new("Secondary").intent(Intent::Secondary));
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
            ui.add(Badge::new("Secondary").intent(Intent::Secondary));
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

fn show_surfaces(ui: &mut egui::Ui) {
    Card::new().show(ui, |ui| {
        ui.heading("Surfaces");
        ui.label("Card frames primary content. Panel frames secondary or raised content.");
        ui.add(Separator::new());
        CastPanel::new().show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.add(Badge::new("Panel").intent(Intent::Neutral));
                ui.label("A raised surface for dense app UI regions.");
            });
        });
    });
}

fn show_text_and_feedback(ui: &mut egui::Ui) {
    Card::new().show(ui, |ui| {
        ui.heading("Text and feedback");
        ui.horizontal_wrapped(|ui| {
            ui.add(Label::new("Default label"));
            ui.add(Label::new("Muted label").muted());
            ui.add(Label::new("Small label").size(Size::Small));
            ui.add(Link::new("Action link"));
            ui.add(Link::new("egui").to("https://github.com/emilk/egui"));
        });
        ui.add(Separator::new().spacing(10.0));
        ui.add(
            Alert::new("Build succeeded")
                .body("The latest component pass compiled and passed focused checks.")
                .intent(Intent::Success),
        );
        ui.add_space(8.0);
        ui.add(
            Alert::new("Review warning")
                .body("Palette derivation still needs dedicated contrast work.")
                .intent(Intent::Warning),
        );
        ui.add_space(8.0);
        ui.add(Notice::new("Neutral notice").body("Notices use the same feedback foundation."));
    });
}

fn show_forms(
    ui: &mut egui::Ui,
    search: &mut String,
    name: &mut String,
    enabled: &mut bool,
    notifications: &mut bool,
    indeterminate: &mut bool,
) {
    Card::new().show(ui, |ui| {
        ui.heading("Forms");
        ui.horizontal(|ui| {
            ui.label("Name");
            ui.add(TextInput::new(name).hint_text("Project name").width(220.0));
        });
        ui.horizontal(|ui| {
            ui.label("Search");
            ui.add(SearchInput::new(search).width(220.0));
        });
        ui.add(Separator::new().spacing(10.0));
        ui.horizontal_wrapped(|ui| {
            ui.add(Checkbox::new(enabled, "Enabled"));
            ui.add(Checkbox::new(indeterminate, "Mixed").indeterminate(true));
        });
        ui.horizontal(|ui| {
            ui.add(Switch::new(notifications));
            ui.label("Notifications");
        });
        ui.horizontal_wrapped(|ui| {
            ui.add(Switch::new(enabled).size(Size::Small));
            ui.add(Switch::new(enabled).size(Size::Medium));
            ui.add(Switch::new(enabled).size(Size::Large));
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

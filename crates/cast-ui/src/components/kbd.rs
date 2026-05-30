use egui::{
    Color32, Response, Sense, StrokeKind, Ui, Widget,
    text::{LayoutJob, TextFormat},
};

use crate::{
    foundation::Size,
    theme::{CastTheme, theme_for_ui},
};

#[derive(Clone, Debug)]
pub struct Kbd {
    keys: Vec<String>,
    size: Size,
}

impl Kbd {
    #[must_use]
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            keys: vec![key.into()],
            size: Size::Medium,
        }
    }

    #[must_use]
    pub fn shortcut<I, K>(keys: I) -> Self
    where
        I: IntoIterator<Item = K>,
        K: Into<String>,
    {
        Self {
            keys: keys.into_iter().map(Into::into).collect(),
            size: Size::Medium,
        }
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}

impl Widget for Kbd {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let metrics = kbd_metrics(&theme, self.size);
        let keys = if self.keys.is_empty() {
            vec![String::new()]
        } else {
            self.keys
        };
        let key_galleys = keys
            .iter()
            .map(|key| {
                ui.painter()
                    .layout_job(kbd_layout_job(key, &theme, metrics.text_size))
            })
            .collect::<Vec<_>>();
        let joiner_galley = ui
            .painter()
            .layout_job(kbd_joiner_layout_job(&theme, metrics.text_size));
        let key_widths = key_galleys
            .iter()
            .map(|galley| (galley.size().x + metrics.padding_x * 2.0).max(metrics.min_height))
            .collect::<Vec<_>>();
        let joiners_width = if key_widths.len() > 1 {
            (key_widths.len() - 1) as f32 * (joiner_galley.size().x + metrics.gap * 2.0)
        } else {
            0.0
        };
        let width = key_widths.iter().sum::<f32>() + joiners_width;
        let (rect, response) =
            ui.allocate_exact_size(egui::vec2(width, metrics.min_height), Sense::hover());

        if ui.is_rect_visible(rect) {
            let mut x = rect.min.x;

            for (index, galley) in key_galleys.into_iter().enumerate() {
                if index > 0 {
                    ui.painter().galley(
                        egui::pos2(
                            x + metrics.gap,
                            rect.center().y - joiner_galley.size().y / 2.0,
                        ),
                        joiner_galley.clone(),
                        theme.colors.text_subtle,
                    );
                    x += joiner_galley.size().x + metrics.gap * 2.0;
                }

                let key_rect = egui::Rect::from_min_size(
                    egui::pos2(x, rect.min.y),
                    egui::vec2(key_widths[index], metrics.min_height),
                );
                paint_keycap(ui, &theme, key_rect, metrics.radius);
                ui.painter().galley(
                    egui::pos2(
                        key_rect.center().x - galley.size().x / 2.0,
                        key_rect.center().y - galley.size().y / 2.0,
                    ),
                    galley,
                    theme.colors.text,
                );
                x += key_rect.width();
            }
        }

        response
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct KbdMetrics {
    min_height: f32,
    padding_x: f32,
    gap: f32,
    radius: f32,
    text_size: f32,
}

fn kbd_metrics(theme: &CastTheme, size: Size) -> KbdMetrics {
    match size {
        Size::Small => KbdMetrics {
            min_height: 22.0,
            padding_x: theme.spacing.xs + 1.0,
            gap: theme.spacing.xs * 0.5,
            radius: theme.radius.sm,
            text_size: theme.typography.small.size - 1.0,
        },
        Size::Medium => KbdMetrics {
            min_height: 26.0,
            padding_x: theme.spacing.sm,
            gap: theme.spacing.xs,
            radius: theme.radius.sm + 1.0,
            text_size: theme.typography.small.size,
        },
        Size::Large => KbdMetrics {
            min_height: 30.0,
            padding_x: theme.spacing.sm + 1.0,
            gap: theme.spacing.xs,
            radius: theme.radius.md,
            text_size: theme.typography.body.size,
        },
    }
}

fn paint_keycap(ui: &Ui, theme: &CastTheme, rect: egui::Rect, radius: f32) {
    ui.painter().rect(
        rect,
        egui::CornerRadius::same(radius as u8),
        theme.colors.surface_muted,
        egui::Stroke::new(theme.stroke.sm.max(1.0), theme.colors.border),
        StrokeKind::Outside,
    );
}

fn kbd_layout_job(key: &str, theme: &CastTheme, text_size: f32) -> LayoutJob {
    LayoutJob::single_section(
        key.to_owned(),
        TextFormat {
            font_id: egui::FontId::new(text_size, theme.typography.code.family.clone()),
            extra_letter_spacing: theme.typography.letter_spacing,
            color: Color32::PLACEHOLDER,
            ..Default::default()
        },
    )
}

fn kbd_joiner_layout_job(theme: &CastTheme, text_size: f32) -> LayoutJob {
    LayoutJob::single_section(
        "+".to_owned(),
        TextFormat {
            font_id: egui::FontId::new(text_size, theme.typography.small.family.clone()),
            extra_letter_spacing: 0.0,
            color: Color32::PLACEHOLDER,
            ..Default::default()
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kbd_defaults_to_single_medium_key() {
        let kbd = Kbd::new("K");

        assert_eq!(kbd.keys, ["K"]);
        assert_eq!(kbd.size, Size::Medium);
    }

    #[test]
    fn kbd_shortcut_collects_keys() {
        let kbd = Kbd::shortcut(["Ctrl", "K"]).size(Size::Small);

        assert_eq!(kbd.keys, ["Ctrl", "K"]);
        assert_eq!(kbd.size, Size::Small);
    }

    #[test]
    fn kbd_metrics_scale_by_size() {
        let theme = CastTheme::light();

        assert!(
            kbd_metrics(&theme, Size::Small).min_height
                < kbd_metrics(&theme, Size::Medium).min_height
        );
        assert!(
            kbd_metrics(&theme, Size::Medium).min_height
                < kbd_metrics(&theme, Size::Large).min_height
        );
    }
}

use egui::{Color32, Response, Sense, Ui, Vec2, Widget};

use crate::{
    color::mix_with_transparent,
    foundation::{Intent, Size},
    theme::{CastTheme, theme_for_ui},
};

#[derive(Clone, Debug)]
pub struct Avatar {
    label: String,
    intent: Intent,
    size: Size,
}

impl Avatar {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            intent: Intent::Primary,
            size: Size::Medium,
        }
    }

    #[must_use]
    pub fn intent(mut self, intent: Intent) -> Self {
        self.intent = intent;
        self
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}

impl Widget for Avatar {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let side = avatar_side(self.size);
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(side), Sense::hover());

        if ui.is_rect_visible(rect) {
            let family = avatar_family(&theme, self.intent);
            ui.painter().circle_filled(
                rect.center(),
                side / 2.0,
                mix_with_transparent(family.base, avatar_fill_alpha(self.intent)),
            );
            ui.painter().circle_stroke(
                rect.center(),
                side / 2.0,
                egui::Stroke::new(theme.stroke.sm, mix_with_transparent(family.base, 0.30)),
            );

            let initials = avatar_initials(&self.label);
            let font = avatar_font(&theme, self.size);
            let galley = ui.painter().layout_no_wrap(initials, font, family.emphasis);
            ui.painter()
                .galley(rect.center() - galley.size() / 2.0, galley, family.emphasis);
        }

        response
    }
}

#[derive(Clone, Copy)]
struct AvatarFamily {
    base: Color32,
    emphasis: Color32,
}

fn avatar_family(theme: &CastTheme, intent: Intent) -> AvatarFamily {
    match intent {
        Intent::Neutral => AvatarFamily {
            base: theme.colors.text_muted,
            emphasis: theme.colors.text,
        },
        Intent::Primary => AvatarFamily {
            base: theme.colors.primary_family.base,
            emphasis: theme.colors.primary_family.emphasis,
        },
        Intent::Secondary => AvatarFamily {
            base: theme.colors.secondary_family.base,
            emphasis: theme.colors.secondary_family.emphasis,
        },
        Intent::Success => AvatarFamily {
            base: theme.colors.success_family.base,
            emphasis: theme.colors.success_family.emphasis,
        },
        Intent::Warning => AvatarFamily {
            base: theme.colors.warning_family.base,
            emphasis: theme.colors.warning_family.emphasis,
        },
        Intent::Danger => AvatarFamily {
            base: theme.colors.danger_family.base,
            emphasis: theme.colors.danger_family.emphasis,
        },
        Intent::Info => AvatarFamily {
            base: theme.colors.info_family.base,
            emphasis: theme.colors.info_family.emphasis,
        },
    }
}

fn avatar_initials(label: &str) -> String {
    let initials = label
        .split_whitespace()
        .filter_map(|part| part.chars().next())
        .take(2)
        .collect::<String>()
        .to_uppercase();

    if initials.is_empty() {
        "?".to_owned()
    } else {
        initials
    }
}

fn avatar_font(theme: &CastTheme, size: Size) -> egui::FontId {
    let mut font = theme.typography.strong.clone();
    font.size = match size {
        Size::Small => theme.typography.caption.size,
        Size::Medium => theme.typography.small.size,
        Size::Large => theme.typography.body.size,
    };
    font
}

fn avatar_side(size: Size) -> f32 {
    match size {
        Size::Small => 24.0,
        Size::Medium => 32.0,
        Size::Large => 40.0,
    }
}

fn avatar_fill_alpha(intent: Intent) -> f32 {
    if intent == Intent::Neutral {
        0.10
    } else {
        0.12
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn avatar_initials_use_first_two_words() {
        assert_eq!(avatar_initials("Sarah Parker"), "SP");
        assert_eq!(avatar_initials("Cast"), "C");
        assert_eq!(avatar_initials(""), "?");
    }

    #[test]
    fn avatar_sizes_scale() {
        assert!(avatar_side(Size::Small) < avatar_side(Size::Medium));
        assert!(avatar_side(Size::Medium) < avatar_side(Size::Large));
    }
}

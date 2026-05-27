use egui::{Color32, Context, FontFamily, FontId, Stroke, Style, Ui, Vec2, Visuals};

const THEME_ID: &str = "cast_theme";

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThemeMode {
    Light,
    Dark,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug)]
pub struct CastTheme {
    pub mode: ThemeMode,
    pub palette: CastPaletteInput,
    pub colors: ColorTokens,
    pub spacing: SpacingTokens,
    pub radius: RadiusTokens,
    pub stroke: StrokeTokens,
    pub typography: TypographyTokens,
    pub controls: ControlTokens,
    pub focus: FocusTokens,
    pub elevation: ElevationTokens,
    pub animation: AnimationTokens,
}

impl CastTheme {
    #[must_use]
    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
            palette: CastPaletteInput {
                accent: Color32::from_rgb(37, 99, 235),
                neutral: Some(Color32::from_rgb(100, 116, 139)),
                success: Some(Color32::from_rgb(22, 163, 74)),
                warning: Some(Color32::from_rgb(217, 119, 6)),
                danger: Some(Color32::from_rgb(220, 38, 38)),
                info: Some(Color32::from_rgb(8, 145, 178)),
            },
            colors: ColorTokens::light(),
            spacing: SpacingTokens::default(),
            radius: RadiusTokens::default(),
            stroke: StrokeTokens::default(),
            typography: TypographyTokens::default(),
            controls: ControlTokens::default(),
            focus: FocusTokens::light(),
            elevation: ElevationTokens::default(),
            animation: AnimationTokens::default(),
        }
    }

    #[must_use]
    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
            palette: CastPaletteInput {
                accent: Color32::from_rgb(96, 165, 250),
                neutral: Some(Color32::from_rgb(148, 163, 184)),
                success: Some(Color32::from_rgb(74, 222, 128)),
                warning: Some(Color32::from_rgb(251, 191, 36)),
                danger: Some(Color32::from_rgb(248, 113, 113)),
                info: Some(Color32::from_rgb(34, 211, 238)),
            },
            colors: ColorTokens::dark(),
            spacing: SpacingTokens::default(),
            radius: RadiusTokens::default(),
            stroke: StrokeTokens::default(),
            typography: TypographyTokens::default(),
            controls: ControlTokens::default(),
            focus: FocusTokens::dark(),
            elevation: ElevationTokens::default(),
            animation: AnimationTokens::default(),
        }
    }

    #[must_use]
    pub fn to_egui_style(&self) -> Style {
        let mut style = Style {
            visuals: self.to_egui_visuals(),
            ..Style::default()
        };

        style.spacing.item_spacing = Vec2::splat(self.spacing.sm);
        style.spacing.button_padding = Vec2::new(self.controls.padding_x, self.controls.padding_y);
        style
    }

    #[must_use]
    pub fn to_egui_visuals(&self) -> Visuals {
        let mut visuals = match self.mode {
            ThemeMode::Light => Visuals::light(),
            ThemeMode::Dark => Visuals::dark(),
        };

        visuals.panel_fill = self.colors.background;
        visuals.window_fill = self.colors.surface_overlay;
        visuals.extreme_bg_color = self.colors.surface_muted;
        visuals.faint_bg_color = self.colors.surface;
        visuals.code_bg_color = self.colors.surface_muted;
        visuals.selection.bg_fill = self.colors.selection;
        visuals.selection.stroke = Stroke::new(self.stroke.md, self.colors.primary);
        visuals.override_text_color = Some(self.colors.text);
        visuals.hyperlink_color = self.colors.link;

        visuals.widgets.noninteractive.bg_fill = self.colors.surface;
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(self.stroke.sm, self.colors.text);
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(self.stroke.sm, self.colors.border);

        visuals.widgets.inactive.bg_fill = self.colors.surface;
        visuals.widgets.inactive.fg_stroke = Stroke::new(self.stroke.sm, self.colors.text);
        visuals.widgets.inactive.bg_stroke = Stroke::new(self.stroke.sm, self.colors.border);

        visuals.widgets.hovered.bg_fill = self.colors.surface_raised;
        visuals.widgets.hovered.fg_stroke = Stroke::new(self.stroke.sm, self.colors.text);
        visuals.widgets.hovered.bg_stroke = Stroke::new(self.stroke.sm, self.colors.border_strong);

        visuals.widgets.active.bg_fill = self.colors.primary;
        visuals.widgets.active.fg_stroke = Stroke::new(self.stroke.sm, self.colors.primary_fg);
        visuals.widgets.active.bg_stroke = Stroke::new(self.stroke.sm, self.colors.primary);

        visuals.widgets.open.bg_fill = self.colors.surface_raised;
        visuals.widgets.open.fg_stroke = Stroke::new(self.stroke.sm, self.colors.text);
        visuals.widgets.open.bg_stroke = Stroke::new(self.stroke.sm, self.colors.border_strong);

        visuals
    }
}

impl Default for CastTheme {
    fn default() -> Self {
        Self::light()
    }
}

pub fn set_theme(ctx: &Context, theme: CastTheme) {
    apply_theme(ctx, &theme);
    ctx.data_mut(|data| data.insert_temp(egui::Id::new(THEME_ID), theme));
}

pub fn apply_theme(ctx: &Context, theme: &CastTheme) {
    ctx.set_global_style(theme.to_egui_style());
}

#[must_use]
pub fn current_theme(ctx: &Context) -> Option<CastTheme> {
    ctx.data(|data| data.get_temp(egui::Id::new(THEME_ID)))
}

#[must_use]
pub fn theme_for_ui(ui: &Ui) -> CastTheme {
    current_theme(ui.ctx()).unwrap_or_else(CastTheme::light)
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug)]
pub struct CastPaletteInput {
    pub accent: Color32,
    pub neutral: Option<Color32>,
    pub success: Option<Color32>,
    pub warning: Option<Color32>,
    pub danger: Option<Color32>,
    pub info: Option<Color32>,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug)]
pub struct ColorTokens {
    pub background: Color32,
    pub surface: Color32,
    pub surface_muted: Color32,
    pub surface_raised: Color32,
    pub surface_overlay: Color32,
    pub border: Color32,
    pub border_strong: Color32,
    pub text: Color32,
    pub text_muted: Color32,
    pub text_subtle: Color32,
    pub primary: Color32,
    pub primary_fg: Color32,
    pub success: Color32,
    pub success_fg: Color32,
    pub warning: Color32,
    pub warning_fg: Color32,
    pub danger: Color32,
    pub danger_fg: Color32,
    pub info: Color32,
    pub info_fg: Color32,
    pub selection: Color32,
    pub focus: Color32,
    pub link: Color32,
}

impl ColorTokens {
    #[must_use]
    pub fn light() -> Self {
        Self {
            background: Color32::from_rgb(248, 250, 252),
            surface: Color32::WHITE,
            surface_muted: Color32::from_rgb(241, 245, 249),
            surface_raised: Color32::from_rgb(248, 250, 252),
            surface_overlay: Color32::WHITE,
            border: Color32::from_rgb(226, 232, 240),
            border_strong: Color32::from_rgb(148, 163, 184),
            text: Color32::from_rgb(15, 23, 42),
            text_muted: Color32::from_rgb(71, 85, 105),
            text_subtle: Color32::from_rgb(100, 116, 139),
            primary: Color32::from_rgb(37, 99, 235),
            primary_fg: Color32::WHITE,
            success: Color32::from_rgb(22, 163, 74),
            success_fg: Color32::WHITE,
            warning: Color32::from_rgb(217, 119, 6),
            warning_fg: Color32::WHITE,
            danger: Color32::from_rgb(220, 38, 38),
            danger_fg: Color32::WHITE,
            info: Color32::from_rgb(8, 145, 178),
            info_fg: Color32::WHITE,
            selection: Color32::from_rgba_premultiplied(37, 99, 235, 48),
            focus: Color32::from_rgb(37, 99, 235),
            link: Color32::from_rgb(29, 78, 216),
        }
    }

    #[must_use]
    pub fn dark() -> Self {
        Self {
            background: Color32::from_rgb(2, 6, 23),
            surface: Color32::from_rgb(15, 23, 42),
            surface_muted: Color32::from_rgb(30, 41, 59),
            surface_raised: Color32::from_rgb(30, 41, 59),
            surface_overlay: Color32::from_rgb(15, 23, 42),
            border: Color32::from_rgb(51, 65, 85),
            border_strong: Color32::from_rgb(100, 116, 139),
            text: Color32::from_rgb(248, 250, 252),
            text_muted: Color32::from_rgb(203, 213, 225),
            text_subtle: Color32::from_rgb(148, 163, 184),
            primary: Color32::from_rgb(96, 165, 250),
            primary_fg: Color32::from_rgb(15, 23, 42),
            success: Color32::from_rgb(74, 222, 128),
            success_fg: Color32::from_rgb(5, 46, 22),
            warning: Color32::from_rgb(251, 191, 36),
            warning_fg: Color32::from_rgb(69, 26, 3),
            danger: Color32::from_rgb(248, 113, 113),
            danger_fg: Color32::from_rgb(69, 10, 10),
            info: Color32::from_rgb(34, 211, 238),
            info_fg: Color32::from_rgb(8, 47, 73),
            selection: Color32::from_rgba_premultiplied(96, 165, 250, 64),
            focus: Color32::from_rgb(147, 197, 253),
            link: Color32::from_rgb(147, 197, 253),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug)]
pub struct SpacingTokens {
    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
}

impl Default for SpacingTokens {
    fn default() -> Self {
        Self {
            xs: 4.0,
            sm: 8.0,
            md: 12.0,
            lg: 16.0,
            xl: 24.0,
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug)]
pub struct RadiusTokens {
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub full: f32,
}

impl Default for RadiusTokens {
    fn default() -> Self {
        Self {
            sm: 4.0,
            md: 6.0,
            lg: 8.0,
            full: 999.0,
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug)]
pub struct StrokeTokens {
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
}

impl Default for StrokeTokens {
    fn default() -> Self {
        Self {
            sm: 1.0,
            md: 1.5,
            lg: 2.0,
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug)]
pub struct TypographyTokens {
    pub body: FontId,
    pub small: FontId,
    pub heading: FontId,
}

impl Default for TypographyTokens {
    fn default() -> Self {
        Self {
            body: FontId::new(14.0, FontFamily::Proportional),
            small: FontId::new(12.0, FontFamily::Proportional),
            heading: FontId::new(20.0, FontFamily::Proportional),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug)]
pub struct ControlTokens {
    pub min_height: f32,
    pub padding_x: f32,
    pub padding_y: f32,
}

impl Default for ControlTokens {
    fn default() -> Self {
        Self {
            min_height: 32.0,
            padding_x: 12.0,
            padding_y: 7.0,
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug)]
pub struct FocusTokens {
    pub width: f32,
    pub color: Color32,
}

impl FocusTokens {
    #[must_use]
    pub fn light() -> Self {
        Self {
            width: 2.0,
            color: ColorTokens::light().focus,
        }
    }

    #[must_use]
    pub fn dark() -> Self {
        Self {
            width: 2.0,
            color: ColorTokens::dark().focus,
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug)]
pub struct ElevationTokens {
    pub shadow_alpha: u8,
}

impl Default for ElevationTokens {
    fn default() -> Self {
        Self { shadow_alpha: 24 }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug)]
pub struct AnimationTokens {
    pub fast_ms: u64,
    pub normal_ms: u64,
}

impl Default for AnimationTokens {
    fn default() -> Self {
        Self {
            fast_ms: 100,
            normal_ms: 160,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_theme_stores_current_theme() {
        let ctx = Context::default();
        let theme = CastTheme::dark();

        set_theme(&ctx, theme);

        assert_eq!(
            current_theme(&ctx).map(|theme| theme.mode),
            Some(ThemeMode::Dark)
        );
    }

    #[test]
    fn egui_style_uses_theme_spacing_and_visuals() {
        let theme = CastTheme::light();
        let style = theme.to_egui_style();

        assert_eq!(style.spacing.item_spacing, Vec2::splat(theme.spacing.sm));
        assert_eq!(style.spacing.button_padding.x, theme.controls.padding_x);
        assert_eq!(style.visuals.panel_fill, theme.colors.background);
        assert_eq!(style.visuals.hyperlink_color, theme.colors.link);
    }
}

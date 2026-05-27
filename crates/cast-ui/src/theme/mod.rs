use egui::{Color32, Context, FontFamily, FontId, Stroke, Style, Ui, Vec2, Visuals};

use crate::color::{accessible_foreground, mix_oklch, with_alpha};

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
        Self::from_palette(
            ThemeMode::Light,
            CastPaletteInput::default_for(ThemeMode::Light),
        )
    }

    #[must_use]
    pub fn dark() -> Self {
        Self::from_palette(
            ThemeMode::Dark,
            CastPaletteInput::default_for(ThemeMode::Dark),
        )
    }

    #[must_use]
    pub fn from_palette(mode: ThemeMode, palette: CastPaletteInput) -> Self {
        let colors = ColorTokens::from_palette(mode, &palette);
        let focus = FocusTokens {
            width: 2.0,
            color: colors.focus,
        };

        Self {
            mode,
            palette,
            colors,
            spacing: SpacingTokens::default(),
            radius: RadiusTokens::default(),
            stroke: StrokeTokens::default(),
            typography: TypographyTokens::default(),
            controls: ControlTokens::default(),
            focus,
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
    pub primary: Color32,
    pub secondary: Option<Color32>,
    pub neutral: Option<Color32>,
    pub success: Option<Color32>,
    pub warning: Option<Color32>,
    pub danger: Option<Color32>,
    pub info: Option<Color32>,
}

impl CastPaletteInput {
    #[must_use]
    pub fn from_primary(primary: Color32) -> Self {
        Self {
            primary,
            secondary: None,
            neutral: None,
            success: None,
            warning: None,
            danger: None,
            info: None,
        }
    }

    #[must_use]
    pub fn default_for(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Light => Self {
                primary: Color32::from_rgb(37, 99, 235),
                secondary: Some(Color32::from_rgb(124, 58, 237)),
                neutral: Some(Color32::from_rgb(100, 116, 139)),
                success: Some(Color32::from_rgb(22, 163, 74)),
                warning: Some(Color32::from_rgb(217, 119, 6)),
                danger: Some(Color32::from_rgb(220, 38, 38)),
                info: Some(Color32::from_rgb(8, 145, 178)),
            },
            ThemeMode::Dark => Self {
                primary: Color32::from_rgb(96, 165, 250),
                secondary: Some(Color32::from_rgb(196, 181, 253)),
                neutral: Some(Color32::from_rgb(148, 163, 184)),
                success: Some(Color32::from_rgb(74, 222, 128)),
                warning: Some(Color32::from_rgb(251, 191, 36)),
                danger: Some(Color32::from_rgb(248, 113, 113)),
                info: Some(Color32::from_rgb(34, 211, 238)),
            },
        }
    }
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
    pub secondary: Color32,
    pub secondary_fg: Color32,
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
        Self::from_palette(
            ThemeMode::Light,
            &CastPaletteInput::default_for(ThemeMode::Light),
        )
    }

    #[must_use]
    pub fn dark() -> Self {
        Self::from_palette(
            ThemeMode::Dark,
            &CastPaletteInput::default_for(ThemeMode::Dark),
        )
    }

    #[must_use]
    pub fn from_palette(mode: ThemeMode, palette: &CastPaletteInput) -> Self {
        match mode {
            ThemeMode::Light => Self::from_light_palette(palette),
            ThemeMode::Dark => Self::from_dark_palette(palette),
        }
    }

    fn from_light_palette(palette: &CastPaletteInput) -> Self {
        let neutral = palette.neutral.unwrap_or(Color32::from_rgb(100, 116, 139));
        let primary = palette.primary;
        let secondary = palette
            .secondary
            .unwrap_or_else(|| mix_oklch(primary, neutral, 0.35));
        let success = palette.success.unwrap_or(Color32::from_rgb(22, 163, 74));
        let warning = palette.warning.unwrap_or(Color32::from_rgb(217, 119, 6));
        let danger = palette.danger.unwrap_or(Color32::from_rgb(220, 38, 38));
        let info = palette.info.unwrap_or(Color32::from_rgb(8, 145, 178));

        Self {
            background: mix_oklch(neutral, Color32::WHITE, 0.94),
            surface: Color32::WHITE,
            surface_muted: mix_oklch(neutral, Color32::WHITE, 0.86),
            surface_raised: mix_oklch(neutral, Color32::WHITE, 0.92),
            surface_overlay: Color32::WHITE,
            border: mix_oklch(neutral, Color32::WHITE, 0.72),
            border_strong: mix_oklch(neutral, Color32::WHITE, 0.35),
            text: mix_oklch(neutral, Color32::BLACK, 0.78),
            text_muted: mix_oklch(neutral, Color32::BLACK, 0.35),
            text_subtle: neutral,
            primary,
            primary_fg: accessible_foreground(primary),
            secondary,
            secondary_fg: accessible_foreground(secondary),
            success,
            success_fg: accessible_foreground(success),
            warning,
            warning_fg: accessible_foreground(warning),
            danger,
            danger_fg: accessible_foreground(danger),
            info,
            info_fg: accessible_foreground(info),
            selection: with_alpha(primary, 48),
            focus: primary,
            link: primary,
        }
    }

    fn from_dark_palette(palette: &CastPaletteInput) -> Self {
        let neutral = palette.neutral.unwrap_or(Color32::from_rgb(148, 163, 184));
        let primary = palette.primary;
        let secondary = palette
            .secondary
            .unwrap_or_else(|| mix_oklch(primary, neutral, 0.35));
        let success = palette.success.unwrap_or(Color32::from_rgb(74, 222, 128));
        let warning = palette.warning.unwrap_or(Color32::from_rgb(251, 191, 36));
        let danger = palette.danger.unwrap_or(Color32::from_rgb(248, 113, 113));
        let info = palette.info.unwrap_or(Color32::from_rgb(34, 211, 238));

        Self {
            background: mix_oklch(neutral, Color32::BLACK, 0.90),
            surface: mix_oklch(neutral, Color32::BLACK, 0.78),
            surface_muted: mix_oklch(neutral, Color32::BLACK, 0.64),
            surface_raised: mix_oklch(neutral, Color32::BLACK, 0.66),
            surface_overlay: mix_oklch(neutral, Color32::BLACK, 0.78),
            border: mix_oklch(neutral, Color32::BLACK, 0.50),
            border_strong: mix_oklch(neutral, Color32::BLACK, 0.20),
            text: mix_oklch(neutral, Color32::WHITE, 0.92),
            text_muted: mix_oklch(neutral, Color32::WHITE, 0.62),
            text_subtle: mix_oklch(neutral, Color32::WHITE, 0.35),
            primary,
            primary_fg: accessible_foreground(primary),
            secondary,
            secondary_fg: accessible_foreground(secondary),
            success,
            success_fg: accessible_foreground(success),
            warning,
            warning_fg: accessible_foreground(warning),
            danger,
            danger_fg: accessible_foreground(danger),
            info,
            info_fg: accessible_foreground(info),
            selection: with_alpha(primary, 64),
            focus: primary,
            link: primary,
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
    use crate::contrast_ratio;

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

    #[test]
    fn palette_theme_uses_runtime_primary_color() {
        let primary = Color32::from_rgb(180, 80, 210);
        let theme =
            CastTheme::from_palette(ThemeMode::Light, CastPaletteInput::from_primary(primary));

        assert_eq!(theme.colors.primary, primary);
        assert_eq!(theme.colors.focus, primary);
        assert_eq!(theme.colors.link, primary);
    }

    #[test]
    fn derived_foregrounds_have_reasonable_contrast() {
        for theme in [
            CastTheme::from_palette(
                ThemeMode::Light,
                CastPaletteInput::from_primary(Color32::from_rgb(37, 99, 235)),
            ),
            CastTheme::from_palette(
                ThemeMode::Dark,
                CastPaletteInput::from_primary(Color32::from_rgb(96, 165, 250)),
            ),
        ] {
            assert!(contrast_ratio(theme.colors.primary, theme.colors.primary_fg) >= 4.5);
            assert!(contrast_ratio(theme.colors.success, theme.colors.success_fg) >= 4.5);
            assert!(contrast_ratio(theme.colors.warning, theme.colors.warning_fg) >= 4.5);
            assert!(contrast_ratio(theme.colors.danger, theme.colors.danger_fg) >= 4.5);
        }
    }
}

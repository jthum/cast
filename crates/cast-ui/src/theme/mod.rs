use std::{borrow::Cow, fs, io, path::Path, sync::Arc};

use egui::{
    Color32, Context, FontData, FontDefinitions, FontFamily, FontId, Stroke, Style, TextStyle, Ui,
    Vec2, Visuals,
};

use crate::color::{accessible_foreground, mix_oklch, with_alpha};

const THEME_ID: &str = "cast_theme";
const INTER_REGULAR_FONT: &str = "cast_inter_regular";
const INTER_MEDIUM_FONT: &str = "cast_inter_medium";
const INTER_SEMIBOLD_FONT: &str = "cast_inter_semibold";
const INTER_REGULAR_FAMILY: &str = "Cast Inter";
const INTER_MEDIUM_FAMILY: &str = "Cast Inter Medium";
const INTER_SEMIBOLD_FAMILY: &str = "Cast Inter SemiBold";
const CAST_CODE_STYLE: &str = "cast_code";
const CAST_LABEL_STYLE: &str = "cast_label";
const CAST_CAPTION_STYLE: &str = "cast_caption";
const CAST_BODY_STRONG_STYLE: &str = "cast_body_strong";
const CAST_HEADING_SM_STYLE: &str = "cast_heading_sm";
const CAST_HEADING_LG_STYLE: &str = "cast_heading_lg";

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
    pub seed: ThemeSeed,
    pub palette: CastPaletteInput,
    pub colors: ColorTokens,
    pub components: ComponentTokens,
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
        ThemeSeed::new(mode, palette).resolve()
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug)]
pub struct ThemeSeed {
    pub mode: ThemeMode,
    pub palette: CastPaletteInput,
    pub component_overrides: ComponentTokenOverrides,
    pub spacing: SpacingTokens,
    pub radius: RadiusTokens,
    pub stroke: StrokeTokens,
    pub typography: TypographyTokens,
    pub controls: ControlTokens,
    pub elevation: ElevationTokens,
    pub animation: AnimationTokens,
}

impl ThemeSeed {
    #[must_use]
    pub fn new(mode: ThemeMode, palette: CastPaletteInput) -> Self {
        Self {
            mode,
            palette,
            component_overrides: ComponentTokenOverrides::default(),
            spacing: SpacingTokens::default(),
            radius: RadiusTokens::default(),
            stroke: StrokeTokens::default(),
            typography: TypographyTokens::default(),
            controls: ControlTokens::default(),
            elevation: ElevationTokens::default(),
            animation: AnimationTokens::default(),
        }
    }

    #[must_use]
    pub fn for_mode(mode: ThemeMode) -> Self {
        Self::new(mode, CastPaletteInput::default_for(mode))
    }

    #[must_use]
    pub fn with_mode(mut self, mode: ThemeMode) -> Self {
        self.mode = mode;
        self
    }

    #[must_use]
    pub fn with_palette(mut self, palette: CastPaletteInput) -> Self {
        self.palette = palette;
        self
    }

    #[must_use]
    pub fn with_primary(mut self, primary: Color32) -> Self {
        self.palette.primary = primary;
        self
    }

    #[must_use]
    pub fn with_component_overrides(mut self, overrides: ComponentTokenOverrides) -> Self {
        self.component_overrides = overrides;
        self
    }

    #[must_use]
    pub fn with_typography(mut self, typography: TypographyTokens) -> Self {
        self.typography = typography;
        self
    }

    #[must_use]
    pub fn with_reduced_motion(mut self, reduced_motion: bool) -> Self {
        self.animation.reduced_motion = reduced_motion;
        self
    }

    #[must_use]
    pub fn with_animation_enabled(mut self, enabled: bool) -> Self {
        self.animation.enabled = enabled;
        self
    }

    #[must_use]
    pub fn with_duration_scale(mut self, duration_scale: f32) -> Self {
        self.animation.duration_scale = duration_scale.max(0.0);
        self
    }

    #[must_use]
    pub fn with_density(mut self, min_height: f32, spacing: f32) -> Self {
        self.set_density(min_height, spacing);
        self
    }

    pub fn set_density(&mut self, min_height: f32, spacing: f32) {
        self.controls.min_height = min_height;
        self.controls.padding_x = min_height * 0.375;
        self.controls.padding_y = min_height * 0.22;
        self.spacing.md = spacing;
        self.spacing.xs = spacing / 3.0;
        self.spacing.sm = spacing * 2.0 / 3.0;
        self.spacing.lg = spacing * 4.0 / 3.0;
        self.spacing.xl = spacing * 2.0;
    }

    #[must_use]
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.set_radius(radius);
        self
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.radius.md = radius;
        self.radius.sm = (radius - 2.0).max(0.0);
        self.radius.lg = radius + 2.0;
    }

    #[must_use]
    pub fn resolve(self) -> CastTheme {
        let colors = ColorTokens::from_palette(self.mode, &self.palette);
        let mut components = ComponentTokens::derive(
            &colors,
            &self.spacing,
            &self.radius,
            &self.stroke,
            &self.controls,
        );
        self.component_overrides.apply_to(&mut components);
        let focus = FocusTokens {
            width: 2.0,
            color: colors.focus,
        };

        CastTheme {
            mode: self.mode,
            palette: self.palette.clone(),
            seed: self.clone(),
            colors,
            components,
            spacing: self.spacing,
            radius: self.radius,
            stroke: self.stroke,
            typography: self.typography,
            controls: self.controls,
            focus,
            elevation: self.elevation,
            animation: self.animation,
        }
    }
}

impl CastTheme {
    #[must_use]
    pub fn to_egui_style(&self) -> Style {
        let mut style = Style {
            visuals: self.to_egui_visuals(),
            ..Style::default()
        };

        style.spacing.item_spacing = Vec2::splat(self.spacing.sm);
        style.spacing.button_padding = Vec2::new(self.controls.padding_x, self.controls.padding_y);
        style.animation_time = self.animation.normal_seconds();
        style
            .text_styles
            .insert(TextStyle::Body, self.typography.body.clone());
        style
            .text_styles
            .insert(TextStyle::Button, self.typography.button.clone());
        style
            .text_styles
            .insert(TextStyle::Small, self.typography.small.clone());
        style
            .text_styles
            .insert(TextStyle::Heading, self.typography.heading.clone());
        style
            .text_styles
            .insert(TextStyle::Monospace, self.typography.code.clone());
        style.text_styles.insert(
            TextStyle::Name(Arc::from(CAST_CODE_STYLE)),
            self.typography.code.clone(),
        );
        style.text_styles.insert(
            TextStyle::Name(Arc::from(CAST_LABEL_STYLE)),
            self.typography.label.clone(),
        );
        style.text_styles.insert(
            TextStyle::Name(Arc::from(CAST_CAPTION_STYLE)),
            self.typography.caption.clone(),
        );
        style.text_styles.insert(
            TextStyle::Name(Arc::from(CAST_BODY_STRONG_STYLE)),
            self.typography.body_strong.clone(),
        );
        style.text_styles.insert(
            TextStyle::Name(Arc::from(CAST_HEADING_SM_STYLE)),
            self.typography.heading_sm.clone(),
        );
        style.text_styles.insert(
            TextStyle::Name(Arc::from(CAST_HEADING_LG_STYLE)),
            self.typography.heading_lg.clone(),
        );
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

pub fn install_inter_fonts(ctx: &Context) {
    install_font_stack(ctx, &FontStack::inter());
}

pub fn install_font_stack(ctx: &Context, stack: &FontStack) {
    let mut fonts = FontDefinitions::default();

    for face in &stack.faces {
        insert_font(&mut fonts, face);
    }

    if let Some(name) = stack.body.first() {
        prepend_family_font(&mut fonts, FontFamily::Proportional, name);
    }
    if let Some(name) = stack.mono.first() {
        prepend_family_font(&mut fonts, FontFamily::Monospace, name);
    }
    set_named_family(&mut fonts, &stack.body_family, &stack.body);
    set_named_family(&mut fonts, &stack.button_family, &stack.button);
    set_named_family(&mut fonts, &stack.strong_family, &stack.strong);
    set_named_family(&mut fonts, &stack.heading_family, &stack.heading);
    if !stack.mono.is_empty() {
        set_named_family(&mut fonts, &stack.mono_family, &stack.mono);
    }

    ctx.set_fonts(fonts);
}

fn insert_font(fonts: &mut FontDefinitions, face: &FontFace) {
    fonts.font_data.insert(
        face.name.clone(),
        Arc::new(FontData {
            font: face.bytes.clone(),
            index: 0,
            tweak: Default::default(),
        }),
    );
}

fn prepend_family_font(fonts: &mut FontDefinitions, family: FontFamily, name: &str) {
    fonts
        .families
        .entry(family)
        .or_default()
        .insert(0, name.to_owned());
}

fn set_named_family(fonts: &mut FontDefinitions, family: &str, names: &[String]) {
    fonts
        .families
        .insert(FontFamily::Name(Arc::<str>::from(family)), names.to_vec());
}

#[derive(Clone, Debug)]
pub struct FontFace {
    pub name: String,
    pub bytes: Cow<'static, [u8]>,
}

impl FontFace {
    #[must_use]
    pub fn from_static(name: impl Into<String>, bytes: &'static [u8]) -> Self {
        Self {
            name: name.into(),
            bytes: Cow::Borrowed(bytes),
        }
    }

    #[must_use]
    pub fn from_owned(name: impl Into<String>, bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            name: name.into(),
            bytes: Cow::Owned(bytes.into()),
        }
    }

    pub fn from_path(name: impl Into<String>, path: impl AsRef<Path>) -> io::Result<Self> {
        fs::read(path).map(|bytes| Self::from_owned(name, bytes))
    }
}

#[derive(Clone, Debug)]
pub struct FontStack {
    pub faces: Vec<FontFace>,
    pub body_family: String,
    pub button_family: String,
    pub strong_family: String,
    pub heading_family: String,
    pub mono_family: String,
    pub body: Vec<String>,
    pub button: Vec<String>,
    pub strong: Vec<String>,
    pub heading: Vec<String>,
    pub mono: Vec<String>,
}

impl FontStack {
    pub const INTER_BODY_FAMILY: &'static str = INTER_REGULAR_FAMILY;
    pub const INTER_BUTTON_FAMILY: &'static str = INTER_MEDIUM_FAMILY;
    pub const INTER_STRONG_FAMILY: &'static str = INTER_SEMIBOLD_FAMILY;
    pub const INTER_HEADING_FAMILY: &'static str = INTER_SEMIBOLD_FAMILY;

    #[must_use]
    pub fn inter() -> Self {
        Self {
            faces: vec![
                FontFace::from_static(
                    INTER_REGULAR_FONT,
                    include_bytes!("../../assets/fonts/inter/Inter-Regular.ttf"),
                ),
                FontFace::from_static(
                    INTER_MEDIUM_FONT,
                    include_bytes!("../../assets/fonts/inter/Inter-Medium.ttf"),
                ),
                FontFace::from_static(
                    INTER_SEMIBOLD_FONT,
                    include_bytes!("../../assets/fonts/inter/Inter-SemiBold.ttf"),
                ),
            ],
            body_family: INTER_REGULAR_FAMILY.to_owned(),
            button_family: INTER_MEDIUM_FAMILY.to_owned(),
            strong_family: INTER_SEMIBOLD_FAMILY.to_owned(),
            heading_family: INTER_SEMIBOLD_FAMILY.to_owned(),
            mono_family: "Cast Mono".to_owned(),
            body: vec![
                INTER_REGULAR_FONT.to_owned(),
                INTER_MEDIUM_FONT.to_owned(),
                INTER_SEMIBOLD_FONT.to_owned(),
            ],
            button: vec![
                INTER_MEDIUM_FONT.to_owned(),
                INTER_REGULAR_FONT.to_owned(),
                INTER_SEMIBOLD_FONT.to_owned(),
            ],
            strong: vec![
                INTER_SEMIBOLD_FONT.to_owned(),
                INTER_MEDIUM_FONT.to_owned(),
                INTER_REGULAR_FONT.to_owned(),
            ],
            heading: vec![
                INTER_SEMIBOLD_FONT.to_owned(),
                INTER_MEDIUM_FONT.to_owned(),
                INTER_REGULAR_FONT.to_owned(),
            ],
            mono: Vec::new(),
        }
    }

    #[must_use]
    pub fn builder() -> FontStackBuilder {
        FontStackBuilder::default()
    }

    #[must_use]
    pub fn google_fonts_css2_url(families: &[GoogleFontFamily]) -> String {
        let query = families
            .iter()
            .map(GoogleFontFamily::query)
            .collect::<Vec<_>>()
            .join("&");
        format!("https://fonts.googleapis.com/css2?{query}&display=swap")
    }

    #[must_use]
    pub fn google_fonts_css2_url_for_names(families: &[&str]) -> String {
        let families = families
            .iter()
            .map(|family| GoogleFontFamily::named(*family))
            .collect::<Vec<_>>();
        Self::google_fonts_css2_url(&families)
    }
}

#[derive(Clone, Debug, Default)]
pub struct FontStackBuilder {
    faces: Vec<FontFace>,
    body: FontRoleBuilder,
    button: FontRoleBuilder,
    strong: FontRoleBuilder,
    heading: FontRoleBuilder,
    mono: FontRoleBuilder,
}

impl FontStackBuilder {
    #[must_use]
    pub fn face(mut self, face: FontFace) -> Self {
        self.faces.push(face);
        self
    }

    #[must_use]
    pub fn faces(mut self, faces: impl IntoIterator<Item = FontFace>) -> Self {
        self.faces.extend(faces);
        self
    }

    #[must_use]
    pub fn body_family(
        mut self,
        family: impl Into<String>,
        fonts: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.body = FontRoleBuilder::new(family, fonts);
        self
    }

    #[must_use]
    pub fn button_family(
        mut self,
        family: impl Into<String>,
        fonts: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.button = FontRoleBuilder::new(family, fonts);
        self
    }

    #[must_use]
    pub fn strong_family(
        mut self,
        family: impl Into<String>,
        fonts: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.strong = FontRoleBuilder::new(family, fonts);
        self
    }

    #[must_use]
    pub fn heading_family(
        mut self,
        family: impl Into<String>,
        fonts: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.heading = FontRoleBuilder::new(family, fonts);
        self
    }

    #[must_use]
    pub fn mono_family(
        mut self,
        family: impl Into<String>,
        fonts: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.mono = FontRoleBuilder::new(family, fonts);
        self
    }

    #[must_use]
    pub fn build(self) -> FontStack {
        let default_fonts = self
            .faces
            .iter()
            .map(|face| face.name.clone())
            .collect::<Vec<_>>();
        let body = self.body.or_fonts("Cast Body", default_fonts);
        let button = self.button.or_clone("Cast Button", &body);
        let strong = self.strong.or_clone("Cast Strong", &button);
        let heading = self.heading.or_clone("Cast Heading", &strong);
        let mono = self.mono;

        FontStack {
            faces: self.faces,
            body_family: body.family,
            button_family: button.family,
            strong_family: strong.family,
            heading_family: heading.family,
            mono_family: mono.family_or("Cast Mono"),
            body: body.fonts,
            button: button.fonts,
            strong: strong.fonts,
            heading: heading.fonts,
            mono: mono.fonts,
        }
    }
}

#[derive(Clone, Debug, Default)]
struct FontRoleBuilder {
    family: Option<String>,
    fonts: Vec<String>,
}

impl FontRoleBuilder {
    fn new(family: impl Into<String>, fonts: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            family: Some(family.into()),
            fonts: fonts.into_iter().map(Into::into).collect(),
        }
    }

    fn or_fonts(self, family: impl Into<String>, fallback_fonts: Vec<String>) -> FontRole {
        FontRole {
            family: self.family.unwrap_or_else(|| family.into()),
            fonts: if self.fonts.is_empty() {
                fallback_fonts
            } else {
                self.fonts
            },
        }
    }

    fn or_clone(self, family: impl Into<String>, fallback: &FontRole) -> FontRole {
        FontRole {
            family: self.family.unwrap_or_else(|| family.into()),
            fonts: if self.fonts.is_empty() {
                fallback.fonts.clone()
            } else {
                self.fonts
            },
        }
    }

    fn family_or(&self, family: impl Into<String>) -> String {
        self.family.clone().unwrap_or_else(|| family.into())
    }
}

#[derive(Clone, Debug)]
struct FontRole {
    family: String,
    fonts: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GoogleFontFamily {
    pub name: String,
    pub weights: Vec<u16>,
}

impl GoogleFontFamily {
    #[must_use]
    pub fn named(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            weights: Vec::new(),
        }
    }

    #[must_use]
    pub fn new(name: impl Into<String>, weights: impl Into<Vec<u16>>) -> Self {
        Self {
            name: name.into(),
            weights: weights.into(),
        }
    }

    fn query(&self) -> String {
        let name = self.name.replace(' ', "+");
        if self.weights.is_empty() {
            return format!("family={name}");
        }

        let weights = self
            .weights
            .iter()
            .map(u16::to_string)
            .collect::<Vec<_>>()
            .join(";");
        format!("family={name}:wght@{weights}")
    }
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
    pub primary_family: SemanticColorTokens,
    pub primary: Color32,
    pub primary_fg: Color32,
    pub secondary_family: SemanticColorTokens,
    pub secondary: Color32,
    pub secondary_fg: Color32,
    pub success_family: SemanticColorTokens,
    pub success: Color32,
    pub success_fg: Color32,
    pub warning_family: SemanticColorTokens,
    pub warning: Color32,
    pub warning_fg: Color32,
    pub danger_family: SemanticColorTokens,
    pub danger: Color32,
    pub danger_fg: Color32,
    pub info_family: SemanticColorTokens,
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

        let surface = Color32::WHITE;
        Self::from_parts(
            ThemeMode::Light,
            surface,
            mix_oklch(neutral, Color32::WHITE, 0.94),
            mix_oklch(neutral, Color32::WHITE, 0.86),
            mix_oklch(neutral, Color32::WHITE, 0.92),
            surface,
            mix_oklch(neutral, Color32::WHITE, 0.72),
            mix_oklch(neutral, Color32::WHITE, 0.35),
            mix_oklch(neutral, Color32::BLACK, 0.78),
            mix_oklch(neutral, Color32::BLACK, 0.35),
            neutral,
            primary,
            secondary,
            success,
            warning,
            danger,
            info,
            48,
        )
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

        let surface = mix_oklch(neutral, Color32::BLACK, 0.78);
        Self::from_parts(
            ThemeMode::Dark,
            surface,
            mix_oklch(neutral, Color32::BLACK, 0.90),
            mix_oklch(neutral, Color32::BLACK, 0.64),
            mix_oklch(neutral, Color32::BLACK, 0.66),
            surface,
            mix_oklch(neutral, Color32::BLACK, 0.50),
            mix_oklch(neutral, Color32::BLACK, 0.20),
            mix_oklch(neutral, Color32::WHITE, 0.92),
            mix_oklch(neutral, Color32::WHITE, 0.62),
            mix_oklch(neutral, Color32::WHITE, 0.35),
            primary,
            secondary,
            success,
            warning,
            danger,
            info,
            64,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn from_parts(
        mode: ThemeMode,
        surface: Color32,
        background: Color32,
        surface_muted: Color32,
        surface_raised: Color32,
        surface_overlay: Color32,
        border: Color32,
        border_strong: Color32,
        text: Color32,
        text_muted: Color32,
        text_subtle: Color32,
        primary: Color32,
        secondary: Color32,
        success: Color32,
        warning: Color32,
        danger: Color32,
        info: Color32,
        selection_alpha: u8,
    ) -> Self {
        let primary_family = SemanticColorTokens::derive(primary, mode, surface);
        let secondary_family = SemanticColorTokens::derive(secondary, mode, surface);
        let success_family = SemanticColorTokens::derive(success, mode, surface);
        let warning_family = SemanticColorTokens::derive(warning, mode, surface);
        let danger_family = SemanticColorTokens::derive(danger, mode, surface);
        let info_family = SemanticColorTokens::derive(info, mode, surface);

        Self {
            background,
            surface,
            surface_muted,
            surface_raised,
            surface_overlay,
            border,
            border_strong,
            text,
            text_muted,
            text_subtle,
            primary_family,
            primary,
            primary_fg: primary_family.fg,
            secondary_family,
            secondary,
            secondary_fg: secondary_family.fg,
            success_family,
            success,
            success_fg: success_family.fg,
            warning_family,
            warning,
            warning_fg: warning_family.fg,
            danger_family,
            danger,
            danger_fg: danger_family.fg,
            info_family,
            info,
            info_fg: info_family.fg,
            selection: with_alpha(primary, selection_alpha),
            focus: primary,
            link: primary,
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SemanticColorTokens {
    pub base: Color32,
    pub fg: Color32,
    pub subtle: Color32,
    pub muted: Color32,
    pub emphasis: Color32,
    pub border: Color32,
    pub hover: Color32,
    pub active: Color32,
    pub disabled: Color32,
}

impl SemanticColorTokens {
    #[must_use]
    pub fn derive(base: Color32, mode: ThemeMode, surface: Color32) -> Self {
        let (subtle_mix, muted_mix, border_mix, hover_mix, active_mix, disabled_mix) = match mode {
            ThemeMode::Light => (0.90, 0.78, 0.58, 0.90, 0.80, 0.84),
            ThemeMode::Dark => (0.84, 0.70, 0.46, 0.82, 0.74, 0.78),
        };
        let hover_anchor = match mode {
            ThemeMode::Light => Color32::BLACK,
            ThemeMode::Dark => Color32::WHITE,
        };

        Self {
            base,
            fg: accessible_foreground(base),
            subtle: mix_oklch(base, surface, subtle_mix),
            muted: mix_oklch(base, surface, muted_mix),
            emphasis: base,
            border: mix_oklch(base, surface, border_mix),
            hover: mix_oklch(base, hover_anchor, hover_mix),
            active: mix_oklch(base, hover_anchor, active_mix),
            disabled: mix_oklch(base, surface, disabled_mix),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug)]
pub struct ComponentTokens {
    pub button: ButtonTokens,
    pub badge: BadgeTokens,
    pub card: SurfaceTokens,
    pub panel: SurfaceTokens,
    pub input: InputTokens,
    pub alert: FeedbackTokens,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ComponentTokenOverrides {
    pub button: ButtonTokenOverrides,
    pub badge: BadgeTokenOverrides,
    pub input: InputTokenOverrides,
    pub card: SurfaceTokenOverrides,
    pub panel: SurfaceTokenOverrides,
    pub alert: FeedbackTokenOverrides,
}

impl ComponentTokenOverrides {
    pub fn apply_to(&self, components: &mut ComponentTokens) {
        self.button.apply_to(&mut components.button);
        self.badge.apply_to(&mut components.badge);
        self.input.apply_to(&mut components.input);
        self.card.apply_to(&mut components.card);
        self.panel.apply_to(&mut components.panel);
        self.alert.apply_to(&mut components.alert);
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        *self == Self::default()
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ButtonTokenOverrides {
    pub radius: Option<f32>,
    pub border_width: Option<f32>,
    pub padding_x: Option<f32>,
    pub padding_y: Option<f32>,
    pub min_height: Option<f32>,
}

impl ButtonTokenOverrides {
    fn apply_to(&self, tokens: &mut ButtonTokens) {
        if let Some(value) = self.radius {
            tokens.radius = value;
        }
        if let Some(value) = self.border_width {
            tokens.border_width = value;
        }
        if let Some(value) = self.padding_x {
            tokens.padding_x = value;
        }
        if let Some(value) = self.padding_y {
            tokens.padding_y = value;
        }
        if let Some(value) = self.min_height {
            tokens.min_height = value;
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BadgeTokenOverrides {
    pub radius: Option<f32>,
    pub border_width: Option<f32>,
    pub padding_x: Option<f32>,
    pub padding_y: Option<f32>,
    pub min_height: Option<f32>,
}

impl BadgeTokenOverrides {
    fn apply_to(&self, tokens: &mut BadgeTokens) {
        if let Some(value) = self.radius {
            tokens.radius = value;
        }
        if let Some(value) = self.border_width {
            tokens.border_width = value;
        }
        if let Some(value) = self.padding_x {
            tokens.padding_x = value;
        }
        if let Some(value) = self.padding_y {
            tokens.padding_y = value;
        }
        if let Some(value) = self.min_height {
            tokens.min_height = value;
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SurfaceTokenOverrides {
    pub fill: Option<Color32>,
    pub border: Option<Color32>,
    pub border_width: Option<f32>,
    pub radius: Option<f32>,
    pub padding: Option<f32>,
}

impl SurfaceTokenOverrides {
    fn apply_to(&self, tokens: &mut SurfaceTokens) {
        if let Some(value) = self.fill {
            tokens.fill = value;
        }
        if let Some(value) = self.border {
            tokens.border = value;
        }
        if let Some(value) = self.border_width {
            tokens.border_width = value;
        }
        if let Some(value) = self.radius {
            tokens.radius = value;
        }
        if let Some(value) = self.padding {
            tokens.padding = value;
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FeedbackTokenOverrides {
    pub radius: Option<f32>,
    pub border_width: Option<f32>,
    pub padding: Option<f32>,
}

impl FeedbackTokenOverrides {
    fn apply_to(&self, tokens: &mut FeedbackTokens) {
        if let Some(value) = self.radius {
            tokens.radius = value;
        }
        if let Some(value) = self.border_width {
            tokens.border_width = value;
        }
        if let Some(value) = self.padding {
            tokens.padding = value;
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct InputTokenOverrides {
    pub fill: Option<Color32>,
    pub fg: Option<Color32>,
    pub border: Option<Color32>,
    pub focus_border: Option<Color32>,
    pub placeholder: Option<Color32>,
    pub border_width: Option<f32>,
    pub radius: Option<f32>,
    pub padding_x: Option<f32>,
    pub padding_y: Option<f32>,
    pub min_height: Option<f32>,
}

impl InputTokenOverrides {
    fn apply_to(&self, tokens: &mut InputTokens) {
        if let Some(value) = self.fill {
            tokens.fill = value;
        }
        if let Some(value) = self.fg {
            tokens.fg = value;
        }
        if let Some(value) = self.border {
            tokens.border = value;
        }
        if let Some(value) = self.focus_border {
            tokens.focus_border = value;
        }
        if let Some(value) = self.placeholder {
            tokens.placeholder = value;
        }
        if let Some(value) = self.border_width {
            tokens.border_width = value;
        }
        if let Some(value) = self.radius {
            tokens.radius = value;
        }
        if let Some(value) = self.padding_x {
            tokens.padding_x = value;
        }
        if let Some(value) = self.padding_y {
            tokens.padding_y = value;
        }
        if let Some(value) = self.min_height {
            tokens.min_height = value;
        }
    }
}

impl ComponentTokens {
    #[must_use]
    pub fn derive(
        colors: &ColorTokens,
        spacing: &SpacingTokens,
        radius: &RadiusTokens,
        stroke: &StrokeTokens,
        controls: &ControlTokens,
    ) -> Self {
        Self {
            button: ButtonTokens {
                radius: radius.md,
                border_width: stroke.sm,
                padding_x: controls.padding_x,
                padding_y: controls.padding_y,
                min_height: controls.min_height,
            },
            badge: BadgeTokens {
                radius: radius.full,
                border_width: stroke.sm,
                padding_x: spacing.sm,
                padding_y: spacing.xs,
                min_height: controls.min_height - 6.0,
            },
            card: SurfaceTokens {
                fill: colors.surface,
                border: colors.border,
                border_width: stroke.sm,
                radius: radius.lg,
                padding: spacing.lg,
            },
            panel: SurfaceTokens {
                fill: colors.surface_raised,
                border: colors.border,
                border_width: stroke.sm,
                radius: radius.lg,
                padding: spacing.lg,
            },
            input: InputTokens {
                fill: colors.surface,
                fg: colors.text,
                border: colors.border,
                focus_border: colors.focus,
                placeholder: colors.text_subtle,
                border_width: stroke.sm,
                radius: radius.md,
                padding_x: controls.padding_x,
                padding_y: controls.padding_y,
                min_height: controls.min_height,
            },
            alert: FeedbackTokens {
                radius: radius.lg,
                border_width: stroke.sm,
                padding: spacing.md,
            },
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ButtonTokens {
    pub radius: f32,
    pub border_width: f32,
    pub padding_x: f32,
    pub padding_y: f32,
    pub min_height: f32,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BadgeTokens {
    pub radius: f32,
    pub border_width: f32,
    pub padding_x: f32,
    pub padding_y: f32,
    pub min_height: f32,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceTokens {
    pub fill: Color32,
    pub border: Color32,
    pub border_width: f32,
    pub radius: f32,
    pub padding: f32,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputTokens {
    pub fill: Color32,
    pub fg: Color32,
    pub border: Color32,
    pub focus_border: Color32,
    pub placeholder: Color32,
    pub border_width: f32,
    pub radius: f32,
    pub padding_x: f32,
    pub padding_y: f32,
    pub min_height: f32,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FeedbackTokens {
    pub radius: f32,
    pub border_width: f32,
    pub padding: f32,
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
    pub xs: FontId,
    pub body: FontId,
    pub small: FontId,
    pub label: FontId,
    pub caption: FontId,
    pub body_strong: FontId,
    pub heading: FontId,
    pub heading_sm: FontId,
    pub heading_lg: FontId,
    pub button: FontId,
    pub strong: FontId,
    pub code: FontId,
}

impl TypographyTokens {
    #[must_use]
    pub fn inter() -> Self {
        Self::from_font_stack(&FontStack::inter())
    }

    #[must_use]
    pub fn with_body_size(mut self, body_size: f32) -> Self {
        self.set_body_size(body_size);
        self
    }

    pub fn set_body_size(&mut self, body_size: f32) {
        let body_size = body_size.max(1.0);
        self.xs.size = (body_size - 3.0).max(1.0);
        self.body.size = body_size;
        self.small.size = (body_size - 2.0).max(1.0);
        self.label.size = (body_size - 2.0).max(1.0);
        self.caption.size = (body_size - 3.0).max(1.0);
        self.body_strong.size = body_size;
        self.heading.size = body_size + 7.0;
        self.heading_sm.size = body_size + 2.0;
        self.heading_lg.size = body_size + 10.0;
        self.button.size = body_size;
        self.strong.size = body_size;
        self.code.size = (body_size - 1.0).max(1.0);
    }

    pub fn set_body_family(&mut self, family: FontFamily) {
        self.xs.family = family.clone();
        self.body.family = family.clone();
        self.small.family = family.clone();
        self.caption.family = family;
    }

    pub fn set_button_family(&mut self, family: FontFamily) {
        self.label.family = family.clone();
        self.button.family = family;
    }

    pub fn set_strong_family(&mut self, family: FontFamily) {
        self.body_strong.family = family.clone();
        self.strong.family = family;
    }

    pub fn set_heading_family(&mut self, family: FontFamily) {
        self.heading.family = family.clone();
        self.heading_sm.family = family.clone();
        self.heading_lg.family = family;
    }

    pub fn set_code_family(&mut self, family: FontFamily) {
        self.code.family = family;
    }

    #[must_use]
    pub fn from_font_stack(stack: &FontStack) -> Self {
        let body = FontFamily::Name(Arc::<str>::from(stack.body_family.as_str()));
        let button = FontFamily::Name(Arc::<str>::from(stack.button_family.as_str()));
        let strong = FontFamily::Name(Arc::<str>::from(stack.strong_family.as_str()));
        let heading = FontFamily::Name(Arc::<str>::from(stack.heading_family.as_str()));
        let mono = if stack.mono.is_empty() {
            FontFamily::Monospace
        } else {
            FontFamily::Name(Arc::<str>::from(stack.mono_family.as_str()))
        };

        Self {
            xs: FontId::new(11.0, body.clone()),
            body: FontId::new(14.0, body.clone()),
            small: FontId::new(12.0, body.clone()),
            label: FontId::new(12.0, button.clone()),
            caption: FontId::new(11.0, body),
            body_strong: FontId::new(14.0, strong.clone()),
            heading: FontId::new(20.0, heading.clone()),
            heading_sm: FontId::new(16.0, heading.clone()),
            heading_lg: FontId::new(24.0, heading),
            button: FontId::new(14.0, button),
            strong: FontId::new(14.0, strong),
            code: FontId::new(13.0, mono),
        }
    }
}

impl Default for TypographyTokens {
    fn default() -> Self {
        Self {
            xs: FontId::new(11.0, FontFamily::Proportional),
            body: FontId::new(14.0, FontFamily::Proportional),
            small: FontId::new(12.0, FontFamily::Proportional),
            label: FontId::new(12.0, FontFamily::Proportional),
            caption: FontId::new(11.0, FontFamily::Proportional),
            body_strong: FontId::new(14.0, FontFamily::Proportional),
            heading: FontId::new(20.0, FontFamily::Proportional),
            heading_sm: FontId::new(16.0, FontFamily::Proportional),
            heading_lg: FontId::new(24.0, FontFamily::Proportional),
            button: FontId::new(14.0, FontFamily::Proportional),
            strong: FontId::new(14.0, FontFamily::Proportional),
            code: FontId::new(13.0, FontFamily::Monospace),
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
    pub enabled: bool,
    pub reduced_motion: bool,
    pub duration_scale: f32,
    pub fast_ms: u64,
    pub normal_ms: u64,
}

impl AnimationTokens {
    #[must_use]
    pub fn fast_seconds(&self) -> f32 {
        self.seconds(self.fast_ms)
    }

    #[must_use]
    pub fn normal_seconds(&self) -> f32 {
        self.seconds(self.normal_ms)
    }

    #[must_use]
    pub fn should_animate(&self) -> bool {
        self.enabled && !self.reduced_motion && self.duration_scale > 0.0
    }

    fn seconds(&self, milliseconds: u64) -> f32 {
        if self.should_animate() {
            milliseconds as f32 * self.duration_scale / 1000.0
        } else {
            0.0
        }
    }
}

impl Default for AnimationTokens {
    fn default() -> Self {
        Self {
            enabled: true,
            reduced_motion: false,
            duration_scale: 1.0,
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
        assert_eq!(style.text_styles[&TextStyle::Body], theme.typography.body);
        assert_eq!(
            style.text_styles[&TextStyle::Button],
            theme.typography.button
        );
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

    #[test]
    fn semantic_family_derives_variant_roles() {
        let theme = CastTheme::light();
        let family = theme.colors.primary_family;

        assert_eq!(family.base, theme.colors.primary);
        assert_eq!(family.fg, theme.colors.primary_fg);
        assert_ne!(family.subtle, family.base);
        assert_ne!(family.hover, family.active);
    }

    #[test]
    fn component_tokens_follow_global_tokens() {
        let theme = CastTheme::light();

        assert_eq!(theme.components.card.fill, theme.colors.surface);
        assert_eq!(theme.components.card.border, theme.colors.border);
        assert_eq!(theme.components.panel.fill, theme.colors.surface_raised);
        assert_eq!(theme.components.input.fg, theme.colors.text);
        assert_eq!(theme.components.input.focus_border, theme.colors.focus);
    }

    #[test]
    fn theme_seed_controls_spacing_radius_stroke_and_type() {
        let mut seed = ThemeSeed::for_mode(ThemeMode::Light);
        seed.spacing.md = 18.0;
        seed.radius.md = 12.0;
        seed.stroke.sm = 2.0;
        seed.typography.body.size = 16.0;
        seed.controls.min_height = 40.0;

        let theme = seed.resolve();

        assert_eq!(theme.spacing.md, 18.0);
        assert_eq!(theme.components.card.radius, theme.radius.lg);
        assert_eq!(theme.components.card.border_width, 2.0);
        assert_eq!(theme.typography.body.size, 16.0);
        assert_eq!(theme.components.button.min_height, 40.0);
    }

    #[test]
    fn theme_seed_component_overrides_are_applied_after_derivation() {
        let mut seed = ThemeSeed::for_mode(ThemeMode::Light);
        seed.radius.md = 10.0;
        seed.component_overrides.button.radius = Some(14.0);
        seed.component_overrides.badge.min_height = Some(18.0);
        seed.component_overrides.input.min_height = Some(42.0);
        seed.component_overrides.card.padding = Some(22.0);
        seed.component_overrides.panel.radius = Some(16.0);
        seed.component_overrides.alert.padding = Some(20.0);

        let theme = seed.resolve();

        assert_eq!(theme.components.button.radius, 14.0);
        assert_eq!(theme.components.badge.min_height, 18.0);
        assert_eq!(theme.components.input.min_height, 42.0);
        assert_eq!(theme.components.card.padding, 22.0);
        assert_eq!(theme.components.panel.radius, 16.0);
        assert_eq!(theme.components.alert.padding, 20.0);
        assert_eq!(theme.radius.md, 10.0);
    }

    #[test]
    fn theme_seed_helpers_update_related_tokens() {
        let theme = ThemeSeed::for_mode(ThemeMode::Light)
            .with_density(40.0, 15.0)
            .with_radius(9.0)
            .with_reduced_motion(true)
            .resolve();

        assert_eq!(theme.controls.min_height, 40.0);
        assert_eq!(theme.controls.padding_x, 15.0);
        assert_eq!(theme.spacing.md, 15.0);
        assert_eq!(theme.spacing.lg, 20.0);
        assert_eq!(theme.radius.sm, 7.0);
        assert_eq!(theme.radius.lg, 11.0);
        assert!(!theme.animation.should_animate());
    }

    #[test]
    fn inter_typography_uses_distinct_weight_families() {
        let typography = TypographyTokens::inter();

        assert_ne!(typography.body.family, typography.button.family);
        assert_ne!(typography.button.family, typography.heading.family);
        assert_eq!(typography.body.size, 14.0);
    }

    #[test]
    fn typography_body_size_updates_related_roles() {
        let typography = TypographyTokens::inter().with_body_size(16.0);

        assert_eq!(typography.body.size, 16.0);
        assert_eq!(typography.caption.size, 13.0);
        assert_eq!(typography.heading_lg.size, 26.0);
        assert_eq!(typography.code.size, 15.0);
    }

    #[test]
    fn typography_family_setters_update_role_groups() {
        let mut typography = TypographyTokens::default();
        let heading = FontFamily::Name(Arc::from("Heading"));
        let body = FontFamily::Name(Arc::from("Body"));

        typography.set_heading_family(heading.clone());
        typography.set_body_family(body.clone());

        assert_eq!(typography.heading.family, heading);
        assert_eq!(typography.heading_sm.family, heading);
        assert_eq!(typography.heading_lg.family, heading);
        assert_eq!(typography.body.family, body);
        assert_eq!(typography.caption.family, body);
    }

    #[test]
    fn font_face_from_path_reads_owned_bytes() {
        let path = std::env::temp_dir().join(format!(
            "cast-ui-font-face-{}-{}.ttf",
            std::process::id(),
            "read"
        ));
        std::fs::write(&path, [1, 2, 3, 4]).unwrap();

        let face = FontFace::from_path("custom_font", &path).unwrap();

        assert_eq!(face.name, "custom_font");
        assert_eq!(face.bytes.as_ref(), &[1, 2, 3, 4]);
        assert!(matches!(face.bytes, Cow::Owned(_)));

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn font_stack_builder_assigns_role_families() {
        let stack = FontStack::builder()
            .face(FontFace::from_owned("body_regular", vec![0]))
            .face(FontFace::from_owned("heading_semibold", vec![1]))
            .body_family("Body", ["body_regular"])
            .heading_family("Heading", ["heading_semibold", "body_regular"])
            .mono_family("Mono", ["code_regular"])
            .build();

        assert_eq!(stack.body_family, "Body");
        assert_eq!(stack.body, vec!["body_regular".to_owned()]);
        assert_eq!(stack.button, vec!["body_regular".to_owned()]);
        assert_eq!(stack.strong, vec!["body_regular".to_owned()]);
        assert_eq!(stack.heading_family, "Heading");
        assert_eq!(
            stack.heading,
            vec!["heading_semibold".to_owned(), "body_regular".to_owned()]
        );
        assert_eq!(stack.mono_family, "Mono");
        assert_eq!(stack.mono, vec!["code_regular".to_owned()]);
    }

    #[test]
    fn font_stack_builder_defaults_body_to_registered_faces() {
        let stack = FontStack::builder()
            .faces([
                FontFace::from_owned("regular", vec![0]),
                FontFace::from_owned("medium", vec![1]),
            ])
            .build();

        let expected = vec!["regular".to_owned(), "medium".to_owned()];
        assert_eq!(stack.body, expected);
        assert_eq!(stack.button, expected);
        assert_eq!(stack.heading, expected);
        assert!(stack.mono.is_empty());
    }

    #[test]
    fn google_fonts_css2_url_encodes_families_and_weights() {
        let url = FontStack::google_fonts_css2_url(&[
            GoogleFontFamily::new("Inter", vec![400, 500, 600]),
            GoogleFontFamily::new("JetBrains Mono", vec![400]),
        ]);

        assert_eq!(
            url,
            "https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&family=JetBrains+Mono:wght@400&display=swap"
        );
    }

    #[test]
    fn google_fonts_css2_url_can_be_built_from_names() {
        let url = FontStack::google_fonts_css2_url_for_names(&["Inter", "JetBrains Mono"]);

        assert_eq!(
            url,
            "https://fonts.googleapis.com/css2?family=Inter&family=JetBrains+Mono&display=swap"
        );
    }

    #[test]
    fn animation_tokens_reduce_motion_to_zero_duration() {
        let mut animation = AnimationTokens::default();

        assert!(animation.fast_seconds() > 0.0);

        animation.reduced_motion = true;

        assert_eq!(animation.fast_seconds(), 0.0);
        assert!(!animation.should_animate());
    }
}

# Cast Design Tokens

Cast themes are resolved at runtime from a small editable `ThemeSeed` into a complete `CastTheme`. Widgets read the resolved theme from egui context with `theme_for_ui(ui)`, while the gallery keeps the seed editable so token changes can be previewed live. The crate package is `cast-ui`, but docs use the recommended dependency alias `cast`.

The current model separates seed input, global tokens, component tokens, and egui style integration:

- `ThemeSeed`: user-editable source values such as mode, palette, spacing, radius, stroke, typography, controls, tone, motion, scroll, elevation, and component overrides.
- `CastTheme`: resolved runtime theme used by widgets.
- `ColorTokens`: global semantic colors and derived color families.
- `ComponentTokens`: component and surface role tokens derived from global tokens.
- `apply_theme` / `set_theme`: bridge Cast tokens into egui style and store the full Cast theme for widgets.

## Palette

`CastPaletteInput` is the editable color seed:

- `primary`
- `secondary`
- `neutral`
- `success`
- `warning`
- `danger`
- `info`

`primary` is required. The other values are optional in the seed and can fall back to mode-specific defaults or derivation rules.

## Color Tokens

`ColorTokens` contains global UI colors:

- `background`
- `surface`
- `surface_muted`
- `surface_raised`
- `surface_overlay`
- `border`
- `border_strong`
- `text`
- `text_muted`
- `text_subtle`
- `selection`
- `focus`
- `link`

It also exposes semantic families:

- `neutral_family`
- `primary_family`
- `secondary_family`
- `success_family`
- `warning_family`
- `danger_family`
- `info_family`

Each `SemanticColorTokens` value contains:

- `base`: source semantic color.
- `fg`: accessible foreground for solid usage on `base`.
- `subtle`: low-emphasis semantic surface.
- `muted`: stronger semantic surface.
- `emphasis`: readable semantic foreground on normal surfaces.
- `border`: semantic border color.
- `hover`: interaction color.
- `active`: stronger interaction color.
- `disabled`: disabled-state semantic surface.

Semantic families are derived in OKLCH and contrast-adjusted where needed. `neutral_family` exists so neutral chrome can be token-driven without borrowing primary color.

## Spacing, Radius, And Stroke

`SpacingTokens`:

- `xs`
- `sm`
- `md`
- `lg`
- `xl`

`RadiusTokens`:

- `sm`
- `md`
- `lg`
- `full`

`StrokeTokens`:

- `sm`
- `md`
- `lg`

These are global scale tokens. Component tokens derive from them, and `ThemeSeed::set_density` / `ThemeSeed::set_radius` update related values together.

## Typography

`TypographyTokens` contains role-based font IDs:

- `xs`
- `body`
- `small`
- `label`
- `caption`
- `body_strong`
- `heading`
- `heading_sm`
- `heading_lg`
- `button`
- `strong`
- `code`
- `letter_spacing`

The font stack supports separate role families for body, controls, strong text, headings, and monospace text. Cast ships with Inter and JetBrains Mono, and external font paths can be loaded by applications.

## Controls

`ControlTokens` contains shared control metrics:

- `min_height`
- `padding_x`
- `padding_y`

Buttons, inputs, and other controls derive their default sizing from these values.

## Tone

`ToneTokens` contains shared alpha stops for accent-tinted fills and borders:

- `subtle_fill_alpha`
- `subtle_hover_fill_alpha`
- `subtle_active_fill_alpha`
- `subtle_border_alpha`
- `subtle_hover_border_alpha`
- `subtle_active_border_alpha`
- `disabled_border_alpha`

These are the named version of the shadcn-like pattern we use for subtle semantic surfaces: a very light tinted fill paired with a stronger tinted border. Buttons, badges, alerts, tabs, and segmented controls should use these stops when they need an accent tint rather than choosing new local alpha values.

## Elevation

`ElevationTokens` controls shadow behavior:

- `shadow_alpha`: global shadow intensity.
- `card`
- `panel`
- `menu`
- `tooltip`
- `toast`
- `popover`
- `dialog`
- `sheet`

Each role is a `ShadowTokens` value:

- `alpha_scale`
- `blur`
- `spread`
- `offset_y`

The final alpha is derived from `shadow_alpha * alpha_scale`, clamped to the valid alpha range. This keeps shadow strength globally adjustable while allowing each surface role to retain its own relative depth.

## Component Tokens

`ComponentTokens` contains resolved component roles:

- `button`
- `badge`
- `card`
- `panel`
- `inset`
- `row`
- `section`
- `input`
- `alert`

`ButtonTokens`:

- `radius`
- `border_width`
- `padding_x`
- `padding_y`
- `min_height`

`BadgeTokens`:

- `radius`
- `border_width`
- `padding_x`
- `padding_y`
- `min_height`

`SurfaceTokens` are used by `card`, `panel`, `inset`, and `row`:

- `fill`
- `border`
- `border_width`
- `radius`
- `padding`

`SurfaceSectionTokens`:

- `muted_fill`: header/footer chrome for sectioned surfaces. This is derived slightly lighter than `surface_muted` so card, dialog, sheet, popover, and panel headers do not become visually heavy.
- `divider`
- `divider_width`
- `padding`
- `compact_padding`

`InputTokens`:

- `fill`
- `fg`
- `border`
- `focus_border`
- `placeholder`
- `border_width`
- `radius`
- `padding_x`
- `padding_y`
- `min_height`

`FeedbackTokens`:

- `radius`
- `border_width`
- `padding`

## Surface Roles

Cast uses generic surface roles instead of component-specific visual recipes:

- `card`: primary framed surface.
- `panel`: raised or secondary framed surface.
- `inset`: nested surface or well inside a card or panel.
- `row`: repeated item surface inside composed layouts.
- `section`: header/footer/divider chrome used by sectioned surfaces.

This means a style like a rounded context card should be achievable by changing global surface, section, radius, spacing, and elevation tokens, while the application or pattern still owns the content structure.

## Overrides

`ComponentTokenOverrides` lets a seed override derived component tokens after derivation:

- `button`
- `badge`
- `input`
- `card`
- `panel`
- `inset`
- `row`
- `section`
- `alert`

Overrides are optional. A `None` field keeps the derived value. This allows a theme to keep most derivation behavior while changing specific component metrics or colors.

## Runtime Editing

Themes are runtime values, not compile-time configuration. The expected application flow is:

```rust
seed.palette.primary = egui::Color32::from_rgb(147, 51, 234);
let theme = seed.clone().resolve();
cast::set_theme(ctx, theme);
```

The gallery's Theme Lab is the live preview surface for this workflow. It currently exposes palette, spacing, radius, border, control height, shadow intensity, typography, motion, scroll, and component overrides.

## Documentation Status

This file documents the current public token model. It does not yet document every component-specific use site or every derived color formula. The gallery remains the primary visual documentation and regression surface for how these tokens render in practice.

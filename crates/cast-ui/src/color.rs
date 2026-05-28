use egui::Color32;
use palette::{
    Clamp, IntoColor, IsWithinBounds, OklabHue, Oklch, Srgb, convert::FromColorUnclamped,
};

#[must_use]
pub fn contrast_ratio(a: Color32, b: Color32) -> f32 {
    let a = relative_luminance(a);
    let b = relative_luminance(b);
    let lighter = a.max(b);
    let darker = a.min(b);

    (lighter + 0.05) / (darker + 0.05)
}

#[must_use]
pub(crate) fn accessible_foreground(background: Color32) -> Color32 {
    let light = Color32::WHITE;
    let dark = Color32::BLACK;

    if contrast_ratio(background, light) >= contrast_ratio(background, dark) {
        light
    } else {
        dark
    }
}

#[must_use]
pub(crate) fn mix_oklch(a: Color32, b: Color32, t: f32) -> Color32 {
    let a = color32_to_oklch(a);
    let b = color32_to_oklch(b);
    let mixed = mix_oklch_raw(a, b, t);

    oklch_to_color32_gamut_mapped(mixed)
}

#[must_use]
pub fn mix_with_transparent(color: Color32, amount: f32) -> Color32 {
    let alpha = (amount.clamp(0.0, 1.0) * 255.0).round() as u8;
    with_alpha(color, alpha)
}

#[must_use]
pub(crate) fn with_alpha(color: Color32, alpha: u8) -> Color32 {
    Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha)
}

fn relative_luminance(color: Color32) -> f32 {
    fn channel(value: u8) -> f32 {
        let value = f32::from(value) / 255.0;
        if value <= 0.03928 {
            value / 12.92
        } else {
            ((value + 0.055) / 1.055).powf(2.4)
        }
    }

    0.2126 * channel(color.r()) + 0.7152 * channel(color.g()) + 0.0722 * channel(color.b())
}

fn color32_to_oklch(color: Color32) -> Oklch {
    Srgb::new(
        f32::from(color.r()) / 255.0,
        f32::from(color.g()) / 255.0,
        f32::from(color.b()) / 255.0,
    )
    .into_color()
}

fn mix_oklch_raw(a: Oklch, b: Oklch, t: f32) -> Oklch {
    let t = t.clamp(0.0, 1.0);
    let hue = shortest_hue_mix(a.hue.into_raw_degrees(), b.hue.into_raw_degrees(), t);

    Oklch::new(
        lerp(a.l, b.l, t).clamp(0.0, 1.0),
        lerp(a.chroma, b.chroma, t).max(0.0),
        OklabHue::from_degrees(hue),
    )
}

fn shortest_hue_mix(a: f32, b: f32, t: f32) -> f32 {
    let delta = (b - a + 540.0).rem_euclid(360.0) - 180.0;
    (a + delta * t).rem_euclid(360.0)
}

fn oklch_to_color32_gamut_mapped(mut color: Oklch) -> Color32 {
    color.l = color.l.clamp(0.0, 1.0);
    color.chroma = color.chroma.max(0.0);

    for _ in 0..32 {
        let rgb: Srgb = Srgb::from_color_unclamped(color);
        if rgb.is_within_bounds() {
            return srgb_to_color32(rgb);
        }
        color.chroma *= 0.92;
    }

    srgb_to_color32(Srgb::from_color_unclamped(color).clamp())
}

fn srgb_to_color32(rgb: Srgb) -> Color32 {
    let rgb: Srgb<u8> = rgb.clamp().into_format();
    Color32::from_rgb(rgb.red, rgb.green, rgb.blue)
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oklch_mix_stays_displayable() {
        let mixed = mix_oklch(
            Color32::from_rgb(37, 99, 235),
            Color32::from_rgb(255, 255, 255),
            0.70,
        );

        assert_eq!(mixed.a(), 255);
    }

    #[test]
    fn transparent_mix_keeps_hue_and_applies_fractional_alpha() {
        let primary = Color32::from_rgb(37, 99, 235);
        let mixed = mix_with_transparent(primary, 0.30);
        let [r, g, b, a] = mixed.to_srgba_unmultiplied();

        assert!((i16::from(r) - i16::from(primary.r())).abs() <= 2);
        assert!((i16::from(g) - i16::from(primary.g())).abs() <= 2);
        assert!((i16::from(b) - i16::from(primary.b())).abs() <= 2);
        assert_eq!(a, 77);
    }

    #[test]
    fn contrast_ratio_prefers_black_on_warning_yellow() {
        let warning = Color32::from_rgb(251, 191, 36);

        assert!(contrast_ratio(warning, Color32::BLACK) > contrast_ratio(warning, Color32::WHITE));
    }
}

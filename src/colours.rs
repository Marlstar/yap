use iced::{theme::Custom, Color};
use std::sync::Arc;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Colours {
    #[serde(with = "ColorDef")]
    pub base: Color,
    #[serde(with = "ColorDef")]
    pub surface: Color,
    #[serde(with = "ColorDef")]
    pub text: Color,
    #[serde(with = "ColorDef")]
    pub accent: Color,
    #[serde(with = "ColorDef")]
    pub ok: Color,
    #[serde(with = "ColorDef")]
    pub warn: Color,
    #[serde(with = "ColorDef")]
    pub error: Color,
}
impl Default for Colours {
    fn default() -> Self {
        let palette = catppuccin::PALETTE.mocha.colors;

        Colours {
            base: Self::iced_from_catppuccin(palette.base),
            surface: Self::iced_from_catppuccin(palette.surface0),
            text: Self::iced_from_catppuccin(palette.text),
            accent: Self::iced_from_catppuccin(palette.mauve),
            ok: Self::iced_from_catppuccin(palette.green),
            warn: Self::iced_from_catppuccin(palette.yellow),
            error: Self::iced_from_catppuccin(palette.red),
        }
    }
}
impl Colours {
    fn iced_from_catppuccin(c: catppuccin::Color) -> Color {
        let rgb = c.rgb;
        return Color::from_rgb8(rgb.r, rgb.g, rgb.b);
    }

    pub fn iced_palette(&self) -> iced::theme::Palette {
        iced::theme::Palette {
            background: self.base,
            primary: self.accent,
            text: self.text,
            success: self.ok,
            warning: self.warn,
            danger: self.error,
        }
    }

    pub fn iced_theme(&self) -> iced::theme::Theme {
        return iced::theme::Theme::Custom(Arc::new(Custom::new("Yap".to_string(), self.iced_palette())));
    }
}


/// Implement [`serde::Serialize`] and [`serde::Deserialize`] for [`iced::Color`]
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(remote = "Color")]
struct ColorDef {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

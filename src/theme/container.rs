use iced::widget::container::{self, Catalog, Style, StyleFn};
use iced::Background;
use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(container::transparent)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn surface(theme: &Theme) -> Style {
    Style {
        background: Some(Background::Color(theme.surface)),
        ..Default::default()
    }
}

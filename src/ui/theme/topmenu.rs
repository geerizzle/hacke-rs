use iced::{color, widget::container, Background, Color};

#[derive(Default)]
struct TopMenuStyle;

impl container::StyleSheet for TopMenuStyle {
    type Style = iced::Theme;

    fn appearance(&self, _: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(color!(0, 0, 0))),
            text_color: Some(Color::WHITE),
            border: iced::Border::default(),
            shadow: iced::Shadow::default(),
        }
    }
}

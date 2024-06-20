use iced::{
    widget::{column, row, Button, Row, Space, Text},
    Length,
};

use crate::posts::Post;

pub fn new_post_card<'a>(post: &'a Post) -> Row<'a, crate::Message> {
    let url = if let Some(url) = &post.url {
        url.clone()
    } else {
        "No URL".to_string()
    };

    let text = Text::new(url.clone()).size(16);

    row![
        Space::new(10, 10),
        column![
            Text::new(post.title.as_ref().unwrap()).size(24),
            Space::new(Length::Fill, Length::Fixed(15.)),
            Button::new(text)
                .on_press(crate::Message::OpenUrl(url))
                .padding(10)
                .style(iced::theme::Button::Secondary),
        ],
    ]
    .width(Length::Fill)
    .height(Length::Fixed(150.))
}

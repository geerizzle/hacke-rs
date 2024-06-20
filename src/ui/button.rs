use iced::widget::Button;

pub fn new_topmenu_button<'a>(
    title: &'a str,
    message: crate::Message,
) -> Button<'a, crate::Message> {
    Button::new(title).on_press(message).padding(10)
}

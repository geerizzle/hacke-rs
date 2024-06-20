use iced::widget::{column, Column, Scrollable, Space};
use iced::{widget::row, Length, Sandbox};
use query::{Filter, Query};
use ui::button::new_topmenu_button;
use ui::card::new_post_card;

mod consts;
mod posts;
mod query;
mod ui;

struct HackerNews {
    query: Query,
    last_filter: Filter,
}

#[derive(Debug, Clone)]
enum Message {
    SortBest,
    SortNewest,
    SortTop,
    OpenUrl(String),
}

impl Sandbox for HackerNews {
    type Message = Message;

    fn new() -> HackerNews {
        HackerNews {
            query: Query::new(),
            last_filter: Filter::Top,
        }
    }

    fn title(&self) -> String {
        String::from("Hacke-rs")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SortBest => {
                self.last_filter = Filter::Best;
                self.query.update(Filter::Best).unwrap();
            }
            Message::SortNewest => {
                self.last_filter = Filter::New;
                self.query.update(Filter::New).unwrap();
            }
            Message::SortTop => {
                self.last_filter = Filter::Top;
                self.query.update(Filter::Top).unwrap();
            }
            Message::OpenUrl(url) => {
                if url == "No URL" {
                    return;
                }
                open::that(url).unwrap();
            }
        }
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    fn view(&self) -> iced::Element<Self::Message> {
        let top_menu = row![
            Space::new(10, 10),
            new_topmenu_button("Refresh", Message::SortNewest),
            Space::new(Length::Fill, Length::Fixed(10.)),
            new_topmenu_button("Newest", Message::SortNewest),
            Space::new(10, 10),
            new_topmenu_button("Top", Message::SortTop),
            Space::new(10, 10),
            new_topmenu_button("Best", Message::SortBest),
            Space::new(10, 10),
        ];
        let mut column: Column<Message> = column![].push(Space::new(10, 10));
        for post in self.query.posts().iter() {
            column = column.push(new_post_card(&post)).spacing(20);
        }

        let scrollable = Scrollable::new(column)
            .width(Length::Fill)
            .height(Length::Fill);

        let top_margin = Space::new(10, 10)
            .width(Length::Fill)
            .height(Length::Fixed(10.));

        column![top_margin, top_menu, scrollable].into()
    }
}

fn main() -> iced::Result {
    HackerNews::run(iced::Settings::default())?;

    Ok(())
}

use iced::{Color, Task, widget};
use iced_anim::{AnimationBuilder, Easing};

use crate::app::{
    app::{QuickKey, ViewPageName},
    message::Message,
    page::page::ViewPage,
};

pub struct MenuPage {
    list: Vec<String>,
    current_item: usize,
}

impl MenuPage {
    pub fn new() -> Self {
        Self {
            list: vec![
                "Playlist".to_owned(),
                "Album".to_owned(),
                "Options".to_owned(),
                "Quit".to_owned(),
            ],
            current_item: 0,
        }
    }
}

impl ViewPage for MenuPage {
    fn view(&self) -> iced::Element<'_, crate::app::message::Message> {
        let mut widget_list = widget::Column::new().spacing(10);

        for (index, item) in self.list.iter().enumerate() {
            let item_size = if index == self.current_item {
                (50.0, Color::from_rgba(0.9, 0.9, 0.9, 0.98))
            } else {
                (30.0, Color::from_rgba(0.85, 0.85, 0.85, 0.68))
            };
            let animated_item =
                AnimationBuilder::new(item_size, |(item_size_val, item_color_val)| {
                    widget::text(item.clone())
                        .size(item_size_val)
                        .color(item_color_val)
                        .into()
                })
                .animates_layout(true)
                .animation(Easing::EASE_IN_OUT.quick());
            widget_list = widget_list.push(animated_item);
        }

        widget::stack![widget_list].into()
    }

    fn update(
        &mut self,
        message: crate::app::message::Message,
    ) -> iced::Task<crate::app::message::Message> {
        match message {
            Message::QuickKeyAction(key) => match key {
                QuickKey::KEYL => {
                    if self.current_item >= 1 {
                        self.current_item -= 1;
                    }
                    Task::none()
                }
                QuickKey::KEYR => {
                    if self.current_item < (self.list.len() - 1) {
                        self.current_item += 1;
                    }
                    Task::none()
                }
                _ => Task::none(),
            },
            _ => Task::none(),
        }
    }

    fn name(&self) -> ViewPageName {
        ViewPageName::Menu
    }
}

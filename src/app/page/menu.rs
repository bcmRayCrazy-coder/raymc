use iced::{
    Color, Element,
    Length::Fill,
    Padding, Task,
    widget::{self},
};
use iced_anim::{Animated, AnimationBuilder, Easing, animation::animation};

use crate::{
    app::{
        app::{QuickKey, ViewPageName},
        message::{MenuMessage, Message},
        page::page::ViewPage,
    },
    cache,
};

pub struct MenuPage {
    list: Vec<String>,
    current_item: usize,
    page_height: f32,

    anim_padding_y: Animated<f32>,
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
            page_height: 0.0,

            anim_padding_y: Animated::transition(0.0, Easing::EASE_IN_OUT.quick()),
        }
    }

    fn widget_list(&self) -> Element<'_, Message> {
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

        widget_list.into()
    }

    fn update_list_scroll(&mut self) {
        let list_selected_y = 40.0 * self.current_item as f32;
        let list_offset_y = self.page_height / 2.0 + 50.0;
        self.anim_padding_y.update(iced_anim::Event::Target(
            list_selected_y * -1.0 + list_offset_y,
        ));
    }
}

impl ViewPage for MenuPage {
    fn view(&self) -> iced::Element<'_, crate::app::message::Message> {
        let background = widget::image(cache::get_cached_image_handle("bg.png").unwrap())
            .width(Fill)
            .height(Fill)
            .content_fit(iced::ContentFit::Cover);

        let widget_list = animation(
            &self.anim_padding_y,
            widget::container(self.widget_list())
                .padding(Padding::new(10.0).top(iced::Pixels::from(*self.anim_padding_y.value())))
                .height(Fill)
                .clip(true), // .into()
        )
        .on_update(|e| Message::Menu(MenuMessage::UpdatePaddingY(e)));
        // .animates_layout(true)
        // .animation(Easing::EASE_IN_OUT.quick());

        widget::stack![background, widget_list].into()
    }

    fn update(
        &mut self,
        message: crate::app::message::Message,
    ) -> iced::Task<crate::app::message::Message> {
        match message {
            Message::Menu(MenuMessage::UpdatePaddingY(event)) => {
                self.anim_padding_y.update(event);
                Task::none()
            }

            Message::OnPageShow => {
                self.update_list_scroll();
                Task::none()
            }

            Message::OnWindowResize(size) => {
                self.page_height = size.height;
                println!("Page size {}", self.page_height);
                Task::none()
            }

            Message::QuickKeyAction(key) => match key {
                QuickKey::KEYL => {
                    if self.current_item >= 1 {
                        self.current_item -= 1;
                        self.update_list_scroll();
                    }
                    Task::none()
                }
                QuickKey::KEYR => {
                    if self.current_item < (self.list.len() - 1) {
                        self.current_item += 1;
                        self.update_list_scroll();
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

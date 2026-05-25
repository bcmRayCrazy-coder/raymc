use iced::{
    Color, Element,
    Length::Fill,
    Padding, Task,
    widget::{self},
};
use iced_anim::{Animated, AnimationBuilder, Easing, animation::animation};

use crate::{
    cache,
    ui::{
        app::{QuickKey, ViewPageName},
        message::{MenuMessage, Message},
        page::page::ViewPage,
    },
};

pub struct MenuPage {
    list: Vec<String>,
    list_icon: Vec<String>,
    current_item: usize,
    current_icon: usize,

    page_width: f32,
    page_height: f32,

    anim_icon_scale: Animated<f32>,
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
            list_icon: vec![
                "icons/playlist.png".to_owned(),
                "icons/album.png".to_owned(),
                "icons/options.png".to_owned(),
                "icons/quit.png".to_owned(),
            ],
            current_item: 0,
            current_icon: 0,

            page_width: 0.0,
            page_height: 0.0,

            anim_icon_scale: Animated::transition(1.0, Easing::EASE_OUT.very_quick()),
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

    fn widget_icon(&self) -> Element<'_, Message> {
        let icon_path = self.list_icon[self.current_icon].clone();

        // icon size <= 240 x 240
        let size = (self.page_height * 0.5).min(260.0) * self.anim_icon_scale.value();

        widget::stack![
            widget::image(cache::get_cached_image_handle("menu_icon_bg.png").unwrap())
                .height(size * 1.3)
                .width(size * 1.3)
                .opacity(0.6),
            widget::container(
                widget::image(cache::get_cached_image_handle(&icon_path).unwrap())
                    .height(size)
                    .width(size)
            )
            .center(size * 1.3),
        ]
        .into()
    }

    fn update_list_scroll(&mut self) {
        let list_selected_y = 50.0 * self.current_item as f32;
        let list_offset_y = self.page_height * 0.6;
        self.anim_padding_y.update(iced_anim::Event::Target(
            list_selected_y * -1.0 + list_offset_y,
        ));
        self.anim_icon_scale.update(iced_anim::Event::Target(0.8));
    }
}

impl ViewPage for MenuPage {
    fn view(&self) -> iced::Element<'_, crate::ui::message::Message> {
        let background = widget::image(cache::get_cached_image_handle("bg.png").unwrap())
            .width(Fill)
            .height(Fill)
            .content_fit(iced::ContentFit::Cover);

        let widget_list = animation(
            &self.anim_padding_y,
            widget::container(self.widget_list())
                .padding(
                    Padding::new(self.page_width * 0.08)
                        .top(iced::Pixels::from(*self.anim_padding_y.value())),
                )
                .height(Fill)
                .clip(true), // .into()
        )
        .on_update(|e| Message::Menu(MenuMessage::UpdatePaddingY(e)));

        let widget_icon = animation(
            &self.anim_icon_scale,
            widget::container(self.widget_icon())
                .align_right(Fill)
                .padding(self.page_width * 0.1)
                .center_y(Fill),
        )
        .on_update(|e| Message::Menu(MenuMessage::UpdateIconScale(e)));

        widget::stack![background, widget_icon, widget_list].into()
    }

    fn update(
        &mut self,
        message: crate::ui::message::Message,
    ) -> iced::Task<crate::ui::message::Message> {
        match message {
            Message::Menu(MenuMessage::ConfirmSelect) => match self.current_item {
                2 => Task::done(Message::ActionPageJump(ViewPageName::Options)),
                3 => Task::done(Message::ActionQuit),
                _ => Task::none(),
            },

            Message::Menu(MenuMessage::UpdatePaddingY(event)) => {
                self.anim_padding_y.update(event);
                Task::none()
            }

            Message::Menu(MenuMessage::UpdateIconScale(event)) => {
                self.anim_icon_scale.update(event);

                if !self.anim_icon_scale.is_animating() {
                    self.anim_icon_scale.update(iced_anim::Event::Target(1.0));
                    self.current_icon = self.current_item;
                }

                Task::none()
            }

            Message::OnPageShow => {
                self.update_list_scroll();
                Task::none()
            }

            Message::OnWindowResize(size) => {
                self.page_width = size.width;
                self.page_height = size.height;
                self.update_list_scroll();
                Task::none()
            }

            Message::QuickKeyAction(key) => match key {
                QuickKey::KEY2 | QuickKey::KEYM => {
                    Task::done(Message::Menu(MenuMessage::ConfirmSelect))
                }
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

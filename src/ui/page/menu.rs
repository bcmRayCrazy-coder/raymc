use iced::{
    Element,
    Length::Fill,
    Padding, Task,
    widget::{self},
};
use iced_anim::{Animated, Easing, animation::animation};

use crate::{
    cache,
    ui::{
        app::{QuickKey, ViewPageName},
        message::{AudioMessage, MenuMessage, Message},
        page::page::ViewPage,
        widget::anim_list::AnimList,
    },
};

pub struct MenuPage<'a> {
    list_icon: Vec<String>,
    current_icon: usize,

    page_width: f32,
    page_height: f32,

    widget_anim_list: AnimList<'a, String>,

    anim_icon_scale: Animated<f32>,
    anim_page_transition: Animated<f32>,
}

impl<'a> MenuPage<'a> {
    pub fn new() -> Self {
        let list = vec![
            "Playlist".to_owned(),
            "Album".to_owned(),
            "Options".to_owned(),
            "Quit".to_owned(),
        ];
        Self {
            list_icon: vec![
                "icons/playlist.png".to_owned(),
                "icons/album.png".to_owned(),
                "icons/options.png".to_owned(),
                "icons/quit.png".to_owned(),
            ],
            current_icon: 0,

            page_width: 0.0,
            page_height: 0.0,

            widget_anim_list: AnimList::default()
                .list(list.clone())
                .on_update(|e| Message::Menu(MenuMessage::UpdateAnimList(e))),

            anim_icon_scale: Animated::transition(1.0, Easing::EASE_OUT.very_quick()),
            anim_page_transition: Animated::transition(0.0, Easing::EASE_IN.very_quick()),
        }
    }

    fn widget_list(&self) -> Element<'_, Message> {
        self.widget_anim_list.widget()
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

    fn jump_page(&mut self, message: Message) -> Task<Message> {
        self.anim_page_transition
            .update(iced_anim::Event::Target(0.0));
        Task::perform(
            tokio::time::sleep(self.anim_page_transition.duration()),
            |_| message,
        )
    }
}

impl ViewPage for MenuPage<'_> {
    fn view(&self) -> iced::Element<'_, Message> {
        let background = widget::image(cache::get_cached_image_handle("bg.png").unwrap())
            .width(Fill)
            .height(Fill)
            .content_fit(iced::ContentFit::Cover);

        let widget_list = animation(
            &self.anim_page_transition,
            widget::container(self.widget_list())
                .padding(
                    Padding::new(0.0)
                        .left(
                            self.page_width
                                * 0.08
                                * (self.anim_page_transition.value() * 2.0 - 1.0),
                        )
                        .top(self.page_height * 0.6),
                )
                .height(Fill),
        )
        .on_update(|e| Message::Menu(MenuMessage::UpdateAnimPageTransition(e)));

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

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Menu(MenuMessage::ConfirmSelect) => match self.widget_anim_list.current() {
                1 => self.jump_page(Message::ActionPageJump(ViewPageName::Album)),
                2 => self.jump_page(Message::ActionPageJump(ViewPageName::Options)),
                3 => Task::done(Message::ActionQuit),
                _ => Task::none(),
            },

            Message::Menu(MenuMessage::UpdateAnimList(event)) => {
                self.widget_anim_list.update(event);
                Task::none()
            }

            Message::Menu(MenuMessage::UpdateAnimPageTransition(event)) => {
                self.anim_page_transition.update(event);
                Task::none()
            }

            Message::Menu(MenuMessage::UpdateIconScale(event)) => {
                self.anim_icon_scale.update(event);

                if !self.anim_icon_scale.is_animating() {
                    self.anim_icon_scale.update(iced_anim::Event::Target(1.0));
                    self.current_icon = self.widget_anim_list.current();
                }

                Task::none()
            }

            Message::OnPageShow => {
                self.anim_page_transition
                    .update(iced_anim::Event::Target(1.0));
                Task::none()
            }

            Message::OnWindowResize(size) => {
                self.page_width = size.width;
                self.page_height = size.height;

                Task::none()
            }

            Message::QuickKeyAction(key) => match key {
                QuickKey::KEY2 | QuickKey::KEYM => {
                    Task::done(Message::Menu(MenuMessage::ConfirmSelect))
                }
                QuickKey::KEYL => {
                    if self.widget_anim_list.scroll_prev() {
                        self.anim_icon_scale.update(iced_anim::Event::Target(0.8));
                        return Task::done(Message::Audio(AudioMessage::PlayUi("chord")));
                    }
                    Task::none()
                }
                QuickKey::KEYR => {
                    if self.widget_anim_list.scroll_next() {
                        self.anim_icon_scale.update(iced_anim::Event::Target(0.8));
                        return Task::done(Message::Audio(AudioMessage::PlayUi("chord")));
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

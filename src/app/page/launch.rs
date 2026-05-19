use iced::{
    Element,
    Length::Fill,
    Task,
    widget::{self, text},
};

use crate::{
    app::{app::ViewPageName, message::Message, page::page::ViewPage},
    cache,
};

pub struct LaunchPage {}

impl LaunchPage {
    pub fn new() -> Self {
        Self {}
    }
}

impl ViewPage for LaunchPage {
    fn view(&self) -> Element<'_, Message> {
        let background = widget::image(
            // "assets/bg.png"
            cache::get_cached_image_handle("bg.png").unwrap(),
        )
        .width(Fill)
        .height(Fill)
        .content_fit(iced::ContentFit::Cover);

        widget::stack![
            background,
            widget::container(widget::row![
                widget::image(cache::get_cached_image_handle("icon.png").unwrap()).height(60),
                widget::column![
                    text("欢迎").size(30),
                    text("Ray Music Center").size(20),
                    widget::button("To Counter").on_press(Message::ViewPageManager(
                        crate::app::message::ViewPageManagerMessage::PageJump(
                            ViewPageName::Counter
                        )
                    ))
                ]
                .padding([0.0, 10.0])
            ])
            .center(Fill)
        ]
        .into()
    }

    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    fn name(&self) -> crate::app::app::ViewPageName {
        crate::app::app::ViewPageName::Launch
    }
}

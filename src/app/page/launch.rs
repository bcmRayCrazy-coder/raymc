use std::time::Duration;

use iced::{
    Element,
    Length::Fill,
    Task,
    widget::{self, text},
};

use crate::{
    app::{app::ViewPageName, message::Message, page::page::ViewPage},
    cache::{self, get_cached_image_handle_list},
};

pub struct LaunchPage {}

impl LaunchPage {
    pub fn new() -> Self {
        Self {}
    }

    fn widget_preload(&self) -> Element<'_, Message> {
        let mut widget_preload = widget::Stack::new().height(1);

        for img in get_cached_image_handle_list() {
            widget_preload = widget_preload.push(widget::image(img));
        }

        widget_preload.into()
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
            self.widget_preload(),
            background,
            widget::container(widget::row![
                widget::image(cache::get_cached_image_handle("icon.png").unwrap()).height(60),
                widget::column![
                    text("欢迎").size(30),
                    text("Ray Music Center").size(20),
                    widget::button("To Counter")
                        .on_press(Message::ActionPageJump(ViewPageName::Counter))
                ]
                .padding([0.0, 10.0])
            ])
            .center(Fill)
        ]
        .height(Fill)
        .width(Fill)
        .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OnPageShow => {
                println!("Page show");
                // Task::none()
                Task::perform(tokio::time::sleep(Duration::from_millis(10)), |_| {
                    Message::ActionPageJump(ViewPageName::Menu)
                })
            }
            _ => Task::none(),
        }
    }

    fn name(&self) -> crate::app::app::ViewPageName {
        crate::app::app::ViewPageName::Launch
    }
}

use iced::{
    Element,
    Length::Fill,
    Task,
    widget::{self, text},
};

use crate::app::{app::App, message::Message};

impl App {
    pub fn launch_view(&self) -> Element<'_, Message> {
        widget::container(widget::row![
            widget::image("res/icon.png").height(60),
            widget::column![text("欢迎").size(30), text("Ray Music Center").size(20)]
                .padding([0.0, 10.0])
        ])
        .center(Fill)
        .into()
    }

    pub fn launch_update(&mut self, message: Message) -> Task<Message> {
        Task::none()
    }
}

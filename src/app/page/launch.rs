use iced::{
    Element,
    Length::Fill,
    Task,
    widget::{self, Container, text},
};

use crate::{
    app::{app::App, message::Message},
    embed,
};

// TODO: Intergrate to ViewPage

impl App {
    pub fn launch_view(&self) -> Element<'_, Message> {
        // widget::image(widget::image::Handle::from_bytes(bytes))
        let background = widget::image(
            // "assets/bg.png"
            embed::get_image_handle("bg.png").unwrap(),
        )
        .width(Fill)
        .height(Fill)
        .content_fit(iced::ContentFit::Cover);

        widget::stack![
            background,
            widget::container(widget::row![
                widget::image(embed::get_image_handle("icon.png").unwrap()).height(60),
                widget::column![text("欢迎").size(30), text("Ray Music Center").size(20)]
                    .padding([0.0, 10.0])
            ])
            .center(Fill)
        ]
        .into()
    }

    pub fn launch_update(&mut self, message: Message) -> Task<Message> {
        Task::none()
    }
}

use iced::{
    Alignment, Element, Length, Renderer, Theme,
    widget::{self, image},
};

use crate::ui::message::Message;

#[derive(Clone)]
pub struct IconText<'a> {
    text: &'a str,
    icon: image::Handle,
    spacing: f32,
    text_size: Option<f32>,
    icon_size: Option<f32>,
    width: Option<Length>,
}

impl<'a> IconText<'a> {
    pub fn new(text: &'a str, icon: image::Handle) -> Self {
        Self {
            text,
            icon,
            spacing: 4.0,
            text_size: None,
            icon_size: None,
            width: None,
        }
    }

    pub fn text_size(mut self, text_size: f32) -> Self {
        self.text_size = Some(text_size);
        self
    }

    pub fn icon_size(mut self, icon_size: f32) -> Self {
        self.icon_size = Some(icon_size);
        self
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = Some(width);
        self
    }

    pub fn widget(&self) -> Element<'a, Message, Theme, Renderer> {
        let mut text = widget::text(self.text).size(20);
        let mut icon = widget::image(self.icon.clone()).width(32).height(32);

        if let Some(size) = self.text_size {
            text = text.size(size);
        }
        if let Some(size) = self.icon_size {
            icon = icon.width(size).height(size);
        }

        let mut w = widget::row![icon, text]
            .spacing(self.spacing)
            .align_y(Alignment::Center);

        if let Some(length) = self.width {
            w = w.width(length);
        }

        w.into()
    }
}

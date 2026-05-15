use iced::Element;

use crate::app::{
    app::{App, ViewPageName},
    message::Message,
};

// TODO: Intergrate to ViewPage

impl App {
    pub fn view(&self) -> Element<'_, Message> {
        match self.view_page {
            ViewPageName::Launch => self.launch_view(),

            ViewPageName::Counter => self.counter_view(),
        }
    }
}

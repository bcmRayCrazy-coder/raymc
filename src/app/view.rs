use iced::Element;

use crate::app::{
    app::{App, ViewPage},
    message::Message,
};

impl App {
    pub fn view(&self) -> Element<'_, Message> {
        match self.view_page {
            ViewPage::Launch => self.launch_view(),

            ViewPage::Counter => self.counter_view(),
        }
    }
}

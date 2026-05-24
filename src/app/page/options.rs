use iced::{Length::Fill, Task, widget};

use crate::app::{
    app::{QuickKey, ViewPageName},
    message::Message,
    page::page::ViewPage,
};

pub struct OptionsPage {}

impl OptionsPage {
    pub fn new() -> Self {
        Self {}
    }
}

impl ViewPage for OptionsPage {
    fn view(&self) -> iced::Element<'_, crate::app::message::Message> {
        widget::container(widget::text("Not Implement Yet...\nPress KEY0 to go back"))
            .center(Fill)
            .into()
    }

    fn update(
        &mut self,
        message: crate::app::message::Message,
    ) -> iced::Task<crate::app::message::Message> {
        match message {
            Message::QuickKeyAction(QuickKey::KEY0) => Task::done(Message::ActionPageBack),
            _ => Task::none(),
        }
    }

    fn name(&self) -> crate::app::app::ViewPageName {
        ViewPageName::Options
    }
}

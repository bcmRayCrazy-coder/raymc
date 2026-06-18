use std::collections::HashMap;

use iced::{Length::Fill, Task, widget};

use crate::ui::{
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
    fn view(&self) -> iced::Element<'_, crate::ui::message::Message> {
        widget::container(widget::text("Not Implement Yet...\nPress KEY0 to go back"))
            .center(Fill)
            .into()
    }

    fn update(
        &mut self,
        message: crate::ui::message::Message,
    ) -> iced::Task<crate::ui::message::Message> {
        match message {
            Message::QuickKeyAction(QuickKey::KEY0) => Task::done(Message::ActionPageBack),
            _ => Task::none(),
        }
    }

    fn name(&self) -> crate::ui::app::ViewPageName {
        ViewPageName::Options
    }

    fn keys_hint(&self) -> std::collections::HashMap<QuickKey, String> {
        let mut map = HashMap::new();

        map.insert(QuickKey::KEY0, "Back".to_owned());

        map
    }
}

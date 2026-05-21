use crate::app::{app::ViewPageName, page::page::ViewPage};

pub struct MenuPage {}

impl MenuPage {
    pub fn new() -> Self {
        Self {}
    }
}

impl ViewPage for MenuPage {
    fn view(&self) -> iced::Element<'_, crate::app::message::Message> {
        todo!()
    }

    fn update(
        &mut self,
        message: crate::app::message::Message,
    ) -> iced::Task<crate::app::message::Message> {
        todo!()
    }

    fn name(&self) -> ViewPageName {
        ViewPageName::Menu
    }
}

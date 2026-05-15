use iced::Task;

use crate::app::message::Message;

#[derive(Debug, Clone)]
pub enum QuickKey {
    KEY0,
    KEY1,
    KEY2,
    KEYL,
    KEYR,
    KEYM,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ViewPageName {
    Launch,
    // For Test Purpose
    Counter,
}

pub struct App {
    pub val: f32,
    pub view_page: ViewPageName,
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        (
            App {
                val: 10.0,
                view_page: ViewPageName::Launch,
            },
            Task::none(),
        )
    }
}

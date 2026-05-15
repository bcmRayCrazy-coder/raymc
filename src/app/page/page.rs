use std::collections::HashMap;

use iced::{Element, Task};

use crate::app::{app::ViewPageName, message::Message};

pub trait ViewPage {
    fn initialize(&mut self);
    fn view(&self) -> Element<'_, Message>;
    fn update(&mut self, message: Message) -> Task<Message>;
    fn name(&self) -> ViewPageName;
}

pub struct ViewPageManager {
    pages: HashMap<ViewPageName, Box<dyn ViewPage>>,
    current_page: ViewPageName,
}

impl ViewPageManager {
    pub fn new() -> Self {
        Self {
            pages: HashMap::new(),
            current_page: ViewPageName::Launch,
        }
    }

    pub fn register(&mut self, page: impl ViewPage + 'static) {
        self.pages.insert(page.name(), Box::new(page));
    }
}

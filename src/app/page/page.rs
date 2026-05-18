use std::collections::HashMap;

use iced::{Element, Length::Fill, Task, widget};

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

    pub fn update(&mut self, message: Message) -> Task<Message> {
        let page = self.pages.get_mut(&self.current_page);
        match page {
            Some(page) => page.update(message),
            None => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let page = self.pages.get(&self.current_page);
        match page {
            Some(page) => page.view(),
            None => self.fallback_view(),
        }
    }

    fn fallback_view(&self) -> Element<'_, Message> {
        widget::container(widget::column![
            widget::text("Error: Page Not Found!"),
            widget::button("Go back")
        ])
        .center(Fill)
        .into()
    }
}

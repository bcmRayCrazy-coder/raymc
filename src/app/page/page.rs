use std::collections::HashMap;

use iced::{Element, Length::Fill, Task, widget};

use crate::app::{app::ViewPageName, message::Message};

pub trait ViewPage {
    fn view(&self) -> Element<'_, Message>;
    fn update(&mut self, message: Message) -> Task<Message>;
    fn name(&self) -> ViewPageName;
}

pub struct ViewPageManager {
    pages: HashMap<ViewPageName, Box<dyn ViewPage>>,
    current_page: ViewPageName,
    history_page: Vec<ViewPageName>,
}

impl ViewPageManager {
    pub fn new() -> Self {
        Self {
            pages: HashMap::new(),
            current_page: ViewPageName::Launch,
            history_page: Vec::new(),
        }
    }

    pub fn register(&mut self, page: impl ViewPage + 'static) {
        self.pages.insert(page.name(), Box::new(page));
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        let (prevent, task) = self.manager_update(&message);
        if prevent {
            return task;
        }
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

    fn manager_update(&mut self, message: &Message) -> (bool, Task<Message>) {
        match message {
            Message::ActionPageJump(view_page_name) => {
                self.history_page.push(self.current_page.clone());
                self.current_page = view_page_name.clone();
                (true, Task::done(Message::OnPageShow))
            }
            Message::ActionPageBack => {
                if let Some(last_page) = self.history_page.last() {
                    self.current_page = last_page.clone();
                    return (true, Task::done(Message::OnPageShow));
                }
                (true, Task::none())
            }
            _ => (false, Task::none()),
        }
    }
}

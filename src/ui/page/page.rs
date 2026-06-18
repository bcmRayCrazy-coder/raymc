use std::collections::HashMap;

use iced::{Element, Length::Fill, Padding, Task, alignment::Vertical::Bottom, widget, window};

use crate::ui::{
    app::{QuickKey, ViewPageName},
    message::Message,
    widget::keys_hint::KeysHint,
};

pub trait ViewPage {
    fn view(&self) -> Element<'_, Message>;
    fn update(&mut self, message: Message) -> Task<Message>;
    fn name(&self) -> ViewPageName;
    fn keys_hint(&self) -> HashMap<QuickKey, String>;
}

pub struct ViewPageManager {
    pages: HashMap<ViewPageName, Box<dyn ViewPage>>,
    current_page: ViewPageName,
    history_page: Vec<ViewPageName>,

    window_id: Option<window::Id>,
    window_closed: bool,

    widget_keys_hint: Box<KeysHint>,
}

impl ViewPageManager {
    pub fn new() -> Self {
        Self {
            pages: HashMap::new(),
            current_page: ViewPageName::Launch,
            history_page: Vec::new(),

            window_id: None,
            window_closed: false,

            widget_keys_hint: Box::new(KeysHint::default()),
        }
    }

    pub fn register(&mut self, page: impl ViewPage + 'static) {
        self.pages.insert(page.name(), Box::new(page));
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        let (prevent, manager_task) = self.manager_update(&message);
        if prevent {
            return manager_task;
        }
        let page = self.pages.get_mut(&self.current_page);
        let page_task = match page {
            Some(page) => page.update(message),
            None => Task::none(),
        };

        Task::batch([manager_task, page_task])
    }

    pub fn view(&self) -> Element<'_, Message> {
        let page = self.pages.get(&self.current_page);
        match page {
            Some(page) => widget::stack![page.view(), self.widget_keys_hint()].into(),
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

    /**
     * Return bool: True to Prevent message passing down to page
     */
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
            Message::ActionQuit => {
                self.window_closed = true;

                let mut task = Task::none();
                self.window_id.inspect(|id| task = window::close(*id));

                (true, task)
            }
            Message::ActionUpdateKeysHint => {
                let page = self.pages.get(&self.current_page);
                if let Some(page) = page {
                    self.widget_keys_hint.keys = page.keys_hint();
                } else {
                    self.widget_keys_hint.keys = HashMap::new();
                }
                (true, Task::none())
            }

            Message::OnPageShow => {
                return (false, Task::done(Message::ActionUpdateKeysHint));
            }

            Message::OnWindowOpen(window_id) => {
                self.window_id = Some(*window_id);

                if self.window_closed {
                    return (true, window::close(*window_id));
                }

                (false, Task::none())
            }

            _ => (false, Task::none()),
        }
    }

    fn widget_keys_hint(&self) -> Element<'_, Message> {
        widget::container(self.widget_keys_hint.widget())
            // .align_y(Bottom)
            .align_bottom(6)
            .height(Fill)
            .into()
    }
}

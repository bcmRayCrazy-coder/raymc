use iced::{
    Element, Event, Subscription, Task, event,
    keyboard::{self, key},
};

use crate::app::{
    message::Message,
    page::{counter::CounterPage, launch::LaunchPage, page::ViewPageManager},
};

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
    pub view_page_manager: ViewPageManager,
}

impl App {
    pub fn new() -> Self {
        App {
            view_page_manager: ViewPageManager::new(),
        }
    }

    pub fn boot() -> (Self, Task<Message>) {
        let mut app = Self::new();

        app.view_page_manager.register(CounterPage::new());
        app.view_page_manager.register(LaunchPage::new());

        (app, Task::none())
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        self.view_page_manager.update(message)
    }

    pub fn view(&self) -> Element<'_, Message> {
        self.view_page_manager.view()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status, _window| {
            if let Event::Keyboard(keyboard::Event::KeyReleased {
                key,
                modifiers: _modifiers,
                ..
            }) = event
            {
                match key {
                    key::Key::Character(c) if c == "1" => {
                        return Some(Message::QuickKeyAction(QuickKey::KEY0));
                    }
                    key::Key::Character(c) if c == "2" => {
                        return Some(Message::QuickKeyAction(QuickKey::KEY1));
                    }
                    _ => {}
                }
            }
            None
        })
    }
}

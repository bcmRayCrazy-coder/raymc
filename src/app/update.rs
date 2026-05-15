use iced::{
    Event, Subscription, Task, event,
    keyboard::{self, key},
};

use crate::app::{
    app::{App, QuickKey, ViewPageName},
    message::Message,
};

// TODO: Intergrate to ViewPage

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match self.view_page {
            ViewPageName::Launch => self.launch_update(message),

            ViewPageName::Counter => self.counter_update(message),
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status, _window| {
            if let Event::Keyboard(keyboard::Event::KeyReleased { key, modifiers, .. }) = event {
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

use std::sync::{Arc, Mutex};

use iced::{
    Element, Event, Subscription, Task, event,
    keyboard::{self, key},
    window,
};

use crate::{
    audio::{
        manager::AudioManager,
        play::{AudioPlay, AudioPlayType},
    },
    ui::{
        message::Message,
        page::{
            counter::CounterPage, launch::LaunchPage, menu::MenuPage, options::OptionsPage,
            page::ViewPageManager,
        },
    },
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
    Menu,
    Options,

    // For Test Purpose
    Counter,
}

pub struct App {
    pub view_page_manager: ViewPageManager,
    audio_manager: Arc<Mutex<AudioManager>>,
}

impl App {
    pub fn new() -> Self {
        App {
            view_page_manager: ViewPageManager::new(),
            audio_manager: Arc::new(Mutex::new(AudioManager::default())),
        }
    }

    pub fn boot() -> (Self, Task<Message>) {
        let mut app = Self::new();

        app.view_page_manager.register(CounterPage::new());
        app.view_page_manager.register(LaunchPage::new());
        app.view_page_manager.register(MenuPage::new());
        app.view_page_manager.register(OptionsPage::new());

        let audio_manager = app.audio_manager.clone();
        std::thread::spawn(move || {
            let mut audio_manager = audio_manager.lock().unwrap();
            let mut test_sample =
                AudioPlay::from_embed("test/test.wav", AudioPlayType::TEST).unwrap();
            test_sample.set_playing(true);
            audio_manager.mixer.lock().unwrap().add_track(test_sample);

            match audio_manager.start() {
                Ok(()) => println!("Audio start"),
                Err(err) => eprintln!("Unable to start audio! {:?}", err),
            }
        });

        (app, Task::done(Message::OnPageShow))
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        self.view_page_manager.update(message)
    }

    pub fn view(&self) -> Element<'_, Message> {
        self.view_page_manager.view()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status, window_id| {
            if let Event::Window(window_event) = &event {
                match window_event {
                    window::Event::Opened {
                        position: _,
                        size: _,
                    } => return Some(Message::OnWindowOpen(window_id)),
                    window::Event::Resized(size) => return Some(Message::OnWindowResize(*size)),
                    _ => {}
                }
            }

            if let Event::Keyboard(keyboard::Event::KeyReleased {
                key,
                modifiers: _modifiers,
                ..
            }) = &event
            {
                match key {
                    key::Key::Character(c) if c == "1" => {
                        return Some(Message::QuickKeyAction(QuickKey::KEY0));
                    }
                    key::Key::Character(c) if c == "2" => {
                        return Some(Message::QuickKeyAction(QuickKey::KEY1));
                    }
                    key::Key::Character(c) if c == "3" => {
                        return Some(Message::QuickKeyAction(QuickKey::KEY2));
                    }
                    key::Key::Character(c) if c == "4" => {
                        return Some(Message::QuickKeyAction(QuickKey::KEYL));
                    }
                    key::Key::Character(c) if c == "5" => {
                        return Some(Message::QuickKeyAction(QuickKey::KEYM));
                    }
                    key::Key::Character(c) if c == "6" => {
                        return Some(Message::QuickKeyAction(QuickKey::KEYR));
                    }
                    _ => {}
                }
            }
            None
        })
    }
}

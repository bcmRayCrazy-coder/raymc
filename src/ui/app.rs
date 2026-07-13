use iced::{
    Element, Event, Subscription, Task, event,
    keyboard::{self, key},
    window,
};

use crate::{
    audio::manager::AudioManager,
    media_control::manager::MediaControlManager,
    player::manager::PlayerManager,
    ui::{
        message::{Message, StateMessage},
        page::{
            album::AlbumPage, counter::CounterPage, launch::LaunchPage, menu::MenuPage,
            options::OptionsPage, page::ViewPageManager, player::PlayerPage,
        },
        state::AppState,
    },
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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
    Album,
    Player,

    // For Test Purpose
    Counter,
}

pub struct App {
    pub view_page_manager: ViewPageManager,
    pub audio_manager: AudioManager,
    pub player_manager: PlayerManager,
    pub media_control_manager: Option<MediaControlManager>,

    pub state: AppState,
}

impl App {
    pub fn new() -> Self {
        App {
            view_page_manager: ViewPageManager::new(),
            audio_manager: AudioManager::default(),
            player_manager: PlayerManager::default(),
            media_control_manager: MediaControlManager::new().ok(),

            state: AppState::default(),
        }
    }

    pub fn boot() -> (Self, Task<Message>) {
        let mut app = Self::new();

        app.view_page_manager.register(CounterPage::new());
        app.view_page_manager.register(LaunchPage::new());
        app.view_page_manager.register(MenuPage::new());
        app.view_page_manager.register(OptionsPage::new());
        app.view_page_manager.register(AlbumPage::new());
        app.view_page_manager.register(PlayerPage::new());

        Self::boot_audio(&mut app);

        (app, Task::done(Message::OnPageShow))
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Audio(audio_message) => self.update_audio(audio_message),
            Message::Player(player_message) => self.update_player(player_message),
            Message::MediaControl(media_control_message) => {
                self.update_media_control(media_control_message)
            }
            Message::State(state_message) => self.update_state(state_message),

            other => self.view_page_manager.update(other),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        self.view_page_manager.view()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            self.subscription_audio(),
            event::listen_with(|event, _status, window_id| {
                if let Event::Window(window_event) = &event {
                    match window_event {
                        window::Event::Opened {
                            position: _,
                            size: _,
                        } => return Some(Message::OnWindowOpen(window_id)),
                        window::Event::Resized(size) => {
                            return Some(Message::State(StateMessage::OnWindowResize(*size)));
                        }
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
            }),
        ])
    }
}

use std::collections::HashMap;

use iced::{Element, Length::Fill, Task, widget};

use crate::{
    cache,
    ui::{
        app::{QuickKey, ViewPageName},
        message::{AudioMessage, Message, PlayerMessage, PlayerPageMessage},
        page::page::ViewPage,
        state::AppState,
    },
};

pub struct PlayerPage {
    state: Box<AppState>,
}

impl PlayerPage {
    pub fn new() -> Self {
        Self {
            state: Box::new(AppState::default()),
        }
    }

    pub fn widget_play_button(&self) -> Element<'_, Message> {
        widget::image(
            cache::get_cached_image_handle(match self.state.is_playing {
                true => "icons/pause.png",
                false => "icons/play.png",
            })
            .unwrap(),
        )
        .width(50.0)
        .height(50.0)
        .into()
    }
}

impl ViewPage for PlayerPage {
    fn view(&self) -> Element<'_, Message> {
        let background = widget::image(cache::get_cached_image_handle("bg.png").unwrap())
            .width(Fill)
            .height(Fill)
            .content_fit(iced::ContentFit::Cover);

        widget::stack![background, self.widget_play_button()].into()
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::PlayerPage(PlayerPageMessage::TogglePlay) => {
                if self.state.is_playing {
                    Task::done(Message::Audio(AudioMessage::PlayerPause))
                } else {
                    Task::done(Message::Audio(AudioMessage::PlayerPlay))
                }
            }

            Message::UpdatePageState(new_state) => {
                self.state = new_state;
                Task::done(Message::ActionUpdateKeysHint)
            }

            Message::QuickKeyAction(key) => match key {
                QuickKey::KEY0 => Task::done(Message::ActionPageBack),
                QuickKey::KEY1 => Task::done(Message::PlayerPage(PlayerPageMessage::TogglePlay)),
                QuickKey::KEYL => Task::done(Message::Player(PlayerMessage::ListPrev)),
                QuickKey::KEYR => Task::done(Message::Player(PlayerMessage::ListNext)),
                _ => Task::none(),
            },

            _ => Task::none(),
        }
    }

    fn name(&self) -> ViewPageName {
        ViewPageName::Player
    }

    fn keys_hint(&self) -> HashMap<QuickKey, String> {
        let mut map = HashMap::new();

        map.insert(QuickKey::KEY0, "Back".to_owned());
        map.insert(QuickKey::KEY1, {
            match self.state.is_playing {
                true => "Pause".to_owned(),
                false => "Play".to_owned(),
            }
        });
        map.insert(QuickKey::KEYL, "Previous".to_owned());
        map.insert(QuickKey::KEYR, "Next".to_owned());

        map
    }
}

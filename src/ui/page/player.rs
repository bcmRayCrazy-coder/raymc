use std::collections::HashMap;

use iced::{Element, Length::Fill, Padding, Task, widget};

use crate::{
    cache,
    player::song::PlaySong,
    ui::{
        app::{QuickKey, ViewPageName},
        message::{AudioMessage, Message, PlayerMessage, PlayerPageMessage, StateMessage},
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

    pub fn page_song_empty(&self) -> Element<'_, Message> {
        widget::container(widget::text("Nothing to play").center())
            .center(Fill)
            .into()
    }

    pub fn page_song_some(&self, song: &PlaySong) -> Element<'_, Message> {
        widget::container(widget::column![
            widget::container(widget::text(format!("Now playing {}", song.name())).center())
                .center_x(Fill),
            widget::container(self.widget_play_button())
                .center_x(Fill)
                .padding(Padding::ZERO.vertical(10))
        ])
        .center(Fill)
        .into()
    }
}

impl ViewPage for PlayerPage {
    fn view(&self) -> Element<'_, Message> {
        let background = widget::image(cache::get_cached_image_handle("bg.png").unwrap())
            .width(Fill)
            .height(Fill)
            .content_fit(iced::ContentFit::Cover);

        let page = match self.state.current_song.as_ref() {
            None => self.page_song_empty(),
            Some(song) => self.page_song_some(song),
        };

        widget::stack![background, page].into()
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

            Message::OnPageShow => Task::done(Message::State(StateMessage::Fetch)),

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

        if self.state.current_song.is_some() {
            map.insert(QuickKey::KEY1, {
                match self.state.is_playing {
                    true => "Pause".to_owned(),
                    false => "Play".to_owned(),
                }
            });
        }

        map.insert(QuickKey::KEYL, "Previous".to_owned());
        map.insert(QuickKey::KEYR, "Next".to_owned());

        map
    }
}

use std::{fs, path::PathBuf};

use iced::{Element, Length::Fill, Padding, Task, widget};
use iced_anim::{Animated, Easing, animation::animation};

use crate::{
    cache, config,
    player::{
        album::{AlbumName, get_album_list},
        playlist::{Playlist, PlaylistTrait},
    },
    ui::{
        app::{QuickKey, ViewPageName},
        message::{AlbumMessage, AudioMessage, Message, PlayerMessage},
        page::page::ViewPage,
        widget::anim_list::AnimList,
    },
};

#[derive(Debug, Clone)]
pub enum AlbumState {
    Album,
    Song(AlbumName),
}

pub struct AlbumPage<'a> {
    album_list: Vec<String>,
    song_list: Option<Vec<String>>,
    album_state: AlbumState,

    page_width: f32,
    page_height: f32,

    widget_anim_album_list: AnimList<'a, String>,
    widget_anim_song_list: AnimList<'a, String>,

    anim_page_transition: Animated<f32>,
}

impl<'a> AlbumPage<'a> {
    pub fn new() -> Self {
        Self {
            album_list: vec!["Single".to_owned()],
            song_list: None,
            album_state: AlbumState::Album,

            page_width: 0.0,
            page_height: 0.0,

            widget_anim_album_list: AnimList::default()
                .on_update(|e| Message::Album(AlbumMessage::UpdateAnimAlbumList(e))),
            widget_anim_song_list: AnimList::default()
                .disabled(true)
                .on_update(|e| Message::Album(AlbumMessage::UpdateAnimSongList(e))),

            anim_page_transition: Animated::transition(0.0, Easing::EASE_IN.quick()),
        }
    }

    fn current_album_name(&self) -> AlbumName {
        if self.widget_anim_album_list.current() == 0 {
            AlbumName::Single
        } else {
            AlbumName::Album(self.album_list[self.widget_anim_album_list.current()].clone())
        }
    }

    fn album_dir(&self) -> PathBuf {
        let (album_dir, first_created) = config::get_app_subdir("album");
        println!("Page show. Album dir {:?}", album_dir);

        if first_created {
            println!("Write to {:?}", album_dir.join("PUT_SONGS_HERE.txt"));
            fs::write(album_dir.join("PUT_SONGS_HERE.txt"), "Put songs here").unwrap();
        }

        album_dir
    }

    fn jump_page(&mut self, message: Message) -> Task<Message> {
        self.anim_page_transition
            .update(iced_anim::Event::Target(0.0));
        Task::perform(
            tokio::time::sleep(self.anim_page_transition.duration()),
            |_| message,
        )
    }

    fn toggle_state(&mut self, state: &AlbumState) -> Task<Message> {
        self.album_state = state.clone();
        match state {
            AlbumState::Album => {
                self.widget_anim_album_list.disabled = false;
                self.widget_anim_song_list.disabled = true;
                Task::none()
            }
            AlbumState::Song(album_name) => {
                self.widget_anim_album_list.disabled = true;
                self.widget_anim_song_list.disabled = false;
                Task::done(Message::Album(AlbumMessage::LoadSongs(album_name.clone())))
            }
        }
    }
}

impl ViewPage for AlbumPage<'_> {
    fn view(&self) -> Element<'_, Message> {
        let background = widget::image(cache::get_cached_image_handle("bg.png").unwrap())
            .width(Fill)
            .height(Fill)
            .content_fit(iced::ContentFit::Cover);

        let widget_album_list = animation(
            &self.anim_page_transition,
            widget::container(self.widget_anim_album_list.widget())
                .padding(
                    Padding::new(0.0)
                        .left(
                            self.page_width
                                * 0.08
                                * (self.anim_page_transition.value() * 2.0 - 1.0),
                        )
                        .top(self.page_height * 0.6),
                )
                .height(Fill),
        )
        .on_update(|e| Message::Album(AlbumMessage::UpdateAnimPageTransition(e)));

        let widget_song_list = animation(
            &self.anim_page_transition,
            widget::container(self.widget_anim_song_list.widget())
                .padding(
                    Padding::new(0.0)
                        .left(
                            self.page_width
                                * (0.58 + (1.0 - self.anim_page_transition.value()) * 0.42),
                        )
                        .top(self.page_height * 0.6),
                )
                .height(Fill),
        )
        .on_update(|e| Message::Album(AlbumMessage::UpdateAnimPageTransition(e)));

        widget::stack![background, widget_album_list, widget_song_list]
            .height(Fill)
            .width(Fill)
            .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Album(AlbumMessage::UpdateAnimAlbumList(event)) => {
                self.widget_anim_album_list.update(event);
                Task::none()
            }

            Message::Album(AlbumMessage::UpdateAnimSongList(event)) => {
                self.widget_anim_song_list.update(event);
                Task::none()
            }

            Message::Album(AlbumMessage::UpdateAnimPageTransition(e)) => {
                self.anim_page_transition.update(e);
                Task::none()
            }

            Message::Album(AlbumMessage::ConfirmSelect) => {
                match self.album_state {
                    AlbumState::Album => {
                        self.toggle_state(&AlbumState::Song(self.current_album_name()))
                    }

                    AlbumState::Song(_) => {
                        Task::batch([
                            Task::done(Message::Player(PlayerMessage::InsertJumpNext(
                                self.current_album_name().get_dir(self.album_dir()).join(
                                    self.widget_anim_song_list.list
                                        [self.widget_anim_song_list.current()]
                                    .clone(),
                                ),
                            ))),
                            // TODO: Jump to player
                            Task::none(),
                        ])
                    }
                }
            }

            Message::Album(AlbumMessage::LoadAlbums(refresh)) => {
                self.album_list = vec!["Single".to_owned()];
                let mut user_album_list = get_album_list(&self.album_dir());
                self.album_list.append(&mut user_album_list);

                self.widget_anim_album_list.list = self.album_list.clone();
                self.widget_anim_album_list.reset_current();

                self.anim_page_transition
                    .update(iced_anim::Event::Target(1.0));

                if refresh {
                    Task::done(Message::Album(AlbumMessage::LoadSongs(
                        self.current_album_name(),
                    )))
                } else {
                    Task::none()
                }
            }

            Message::Album(AlbumMessage::LoadSongs(album_name)) => {
                let song_list = album_name.get_songs(self.album_dir());

                if song_list.len() > 0 {
                    self.song_list = Some(song_list);
                } else {
                    self.song_list = None;
                }

                self.widget_anim_song_list.list =
                    self.song_list.clone().unwrap_or(vec!["Empty".to_owned()]);
                self.widget_anim_song_list.reset_current();

                Task::none()
            }

            Message::OnPageShow => Task::done(Message::Album(AlbumMessage::LoadAlbums(true))),
            Message::OnWindowResize(size) => {
                self.page_width = size.width;
                self.page_height = size.height;

                Task::none()
            }

            Message::QuickKeyAction(key) => match key {
                QuickKey::KEY0 => match self.album_state {
                    AlbumState::Album => self.jump_page(Message::ActionPageBack),
                    AlbumState::Song(_) => self.toggle_state(&AlbumState::Album),
                },
                QuickKey::KEY2 => Task::done(Message::Album(AlbumMessage::ConfirmSelect)),
                QuickKey::KEYL => match self.album_state {
                    AlbumState::Album => {
                        if self.widget_anim_album_list.scroll_prev() {
                            return Task::batch([
                                Task::done(Message::Audio(AudioMessage::PlayUi("chord"))),
                                Task::done(Message::Album(AlbumMessage::LoadSongs(
                                    self.current_album_name(),
                                ))),
                            ]);
                        }
                        Task::none()
                    }
                    AlbumState::Song(_) => {
                        if self.widget_anim_song_list.scroll_prev() {
                            return Task::done(Message::Audio(AudioMessage::PlayUi("chord")));
                        }
                        Task::none()
                    }
                },
                QuickKey::KEYR => match self.album_state {
                    AlbumState::Album => {
                        if self.widget_anim_album_list.scroll_next() {
                            return Task::batch([
                                Task::done(Message::Audio(AudioMessage::PlayUi("chord"))),
                                Task::done(Message::Album(AlbumMessage::LoadSongs(
                                    self.current_album_name(),
                                ))),
                            ]);
                        }
                        Task::none()
                    }
                    AlbumState::Song(_) => {
                        if self.widget_anim_song_list.scroll_next() {
                            return Task::done(Message::Audio(AudioMessage::PlayUi("chord")));
                        }
                        Task::none()
                    }
                },
                _ => Task::none(),
            },

            _ => Task::none(),
        }
    }

    fn name(&self) -> ViewPageName {
        ViewPageName::Album
    }
}

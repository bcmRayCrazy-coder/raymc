use std::fs;

use iced::{Element, Length::Fill, Padding, Task, widget};
use iced_anim::{Animated, Easing, animation::animation};

use crate::{
    cache, config,
    ui::{
        app::{QuickKey, ViewPageName},
        message::{AlbumMessage, AudioMessage, Message},
        page::page::ViewPage,
        widget::anim_list::AnimList,
    },
};

pub enum AlbumState {}

pub struct AlbumPage {
    album_list: Vec<String>,

    page_width: f32,
    page_height: f32,

    widget_anim_list: AnimList,

    anim_page_transition: Animated<f32>,
}

impl AlbumPage {
    pub fn new() -> Self {
        Self {
            album_list: vec!["Single".to_owned()],

            page_width: 0.0,
            page_height: 0.0,

            widget_anim_list: AnimList::new()
                .on_update(|e| Message::Album(AlbumMessage::UpdateAnimList(e))),

            anim_page_transition: Animated::transition(0.0, Easing::EASE_IN.quick()),
        }
    }

    fn jump_page(&mut self, message: Message) -> Task<Message> {
        self.anim_page_transition
            .update(iced_anim::Event::Target(0.0));
        Task::perform(
            tokio::time::sleep(self.anim_page_transition.duration()),
            |_| message,
        )
    }
}

impl ViewPage for AlbumPage {
    fn view(&self) -> Element<'_, Message> {
        let background = widget::image(cache::get_cached_image_handle("bg.png").unwrap())
            .width(Fill)
            .height(Fill)
            .content_fit(iced::ContentFit::Cover);

        let widget_list = animation(
            &self.anim_page_transition,
            widget::container(self.widget_anim_list.widget())
                .padding(
                    Padding::new(
                        self.page_width * 0.08 * (self.anim_page_transition.value() * 2.0 - 1.0),
                    )
                    .top(self.page_height * 0.6),
                )
                .height(Fill),
        )
        .on_update(|e| Message::Album(AlbumMessage::UpdateAnimPageTransition(e)));

        widget::stack![background, widget_list]
            .height(Fill)
            .width(Fill)
            .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Album(AlbumMessage::UpdateAnimList(event)) => {
                self.widget_anim_list.update(event);
                Task::none()
            }

            Message::Album(AlbumMessage::UpdateAnimPageTransition(e)) => {
                self.anim_page_transition.update(e);
                Task::none()
            }

            Message::OnPageShow => {
                let (album_dir, first_created) = config::get_app_subdir("album");
                // println!("Page show. Album dir {:?}", album_dir);

                if first_created {
                    println!("Write to {:?}", album_dir.join("PUT_SONGS_HERE.txt"));
                    fs::write(album_dir.join("PUT_SONGS_HERE.txt"), "Put songs here").unwrap();
                }

                self.album_list = vec!["Single".to_owned()];

                if let Ok(entries) = fs::read_dir(album_dir) {
                    let mut unknown_id_counter = 0;
                    for entry in entries {
                        if let Ok(entry) = entry
                            && let Ok(file_type) = entry.file_type()
                            && file_type.is_dir()
                        {
                            // println!("Album {}", entry.file_name().to_str().unwrap_or("Unknown"));
                            self.album_list.push(match entry.file_name().to_str() {
                                Some(name) => name.to_owned(),
                                None => {
                                    let name = format!("Unkown album #{}", unknown_id_counter);
                                    unknown_id_counter += 1;
                                    name
                                }
                            });
                        }
                    }
                    self.widget_anim_list = self.widget_anim_list.list(self.album_list.clone());
                }

                self.anim_page_transition
                    .update(iced_anim::Event::Target(1.0));
                Task::none()
            }

            Message::OnWindowResize(size) => {
                self.page_width = size.width;
                self.page_height = size.height;

                Task::none()
            }

            Message::QuickKeyAction(key) => match key {
                // QuickKey::KEY0 => Task::done(Message::ActionPageBack),
                QuickKey::KEY0 => self.jump_page(Message::ActionPageBack),
                QuickKey::KEY2 => Task::done(Message::Album(AlbumMessage::ConfirmSelect)),
                QuickKey::KEYL => {
                    if self.widget_anim_list.scroll_prev() {
                        return Task::done(Message::Audio(AudioMessage::PlayUi("chord")));
                    }
                    Task::none()
                }
                QuickKey::KEYR => {
                    if self.widget_anim_list.scroll_next() {
                        return Task::done(Message::Audio(AudioMessage::PlayUi("chord")));
                    }
                    Task::none()
                }
                _ => Task::none(),
            },

            _ => Task::none(),
        }
    }

    fn name(&self) -> ViewPageName {
        ViewPageName::Album
    }
}

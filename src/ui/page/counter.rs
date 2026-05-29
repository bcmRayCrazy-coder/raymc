use iced::{Element, Task, widget};
use iced_anim::{AnimationBuilder, Easing};

use crate::{
    cache::{self},
    ui::{
        app::{QuickKey, ViewPageName},
        message::{CounterMessage, Message},
        page::page::ViewPage,
    },
};

pub struct CounterPage {
    val: f32,
    page_show_msg: bool,
}

impl CounterPage {
    pub fn new() -> Self {
        Self {
            val: 10.0,
            page_show_msg: false,
        }
    }
}

impl ViewPage for CounterPage {
    fn view(&self) -> Element<'_, Message> {
        let animated_text = AnimationBuilder::new(self.val, move |val| {
            widget::text(format!("Text with size {}", self.val))
                .size(val)
                .into()
        })
        .animates_layout(true)
        .animation(Easing::EASE);

        let animated_size_text = AnimationBuilder::new(self.val, |val| {
            widget::text(format!("Actual size {}", val)).into()
        })
        .animates_layout(true)
        .animation(Easing::EASE);

        let mut widget_page = widget::Row::new()
            .spacing(10)
            .push(
                widget::button("Go to Launch Page")
                    .on_press(Message::ActionPageJump(ViewPageName::Launch)),
            )
            .push(widget::button("Go Back").on_press(Message::ActionPageBack));

        if self.page_show_msg {
            widget_page = widget_page.push(
                widget::button("Hide page show message")
                    .on_press(Message::Counter(CounterMessage::HideMsg)),
            );
        }

        let widget_animation = widget::column![
            widget::row![
                widget::image(cache::get_cached_image_handle("icon.png").unwrap()).height(60),
                widget::button("Inc").on_press(Message::Counter(CounterMessage::Increment)),
                widget::button("Dec").on_press(Message::Counter(CounterMessage::Decreasement)),
                animated_size_text,
            ]
            .spacing(10),
            animated_text
        ];

        widget::container(
            widget::column![
                widget_page,
                widget_animation,
                // widget_preload
            ]
            .spacing(10),
        )
        .padding(10)
        .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::QuickKeyAction(key) => match key {
                QuickKey::KEY0 => {
                    self.val += 40.0;
                }
                QuickKey::KEY1 => {
                    if self.val >= 50.0 {
                        self.val -= 40.0;
                    }
                }
                _ => {}
            },

            Message::OnPageShow => {
                self.page_show_msg = true;
            }

            Message::Counter(CounterMessage::HideMsg) => {
                self.page_show_msg = false;
            }
            Message::Counter(CounterMessage::Increment) => {
                self.val += 10.0;
            }
            Message::Counter(CounterMessage::Decreasement) => {
                if self.val >= 20.0 {
                    self.val -= 10.0;
                }
            }

            _ => {}
        }
        Task::none()
    }

    fn name(&self) -> ViewPageName {
        ViewPageName::Counter
    }
}

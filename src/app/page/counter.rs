use iced::{Element, Task, widget};
use iced_anim::{AnimationBuilder, Easing};

use crate::app::{
    app::{App, QuickKey, ViewPage},
    message::Message,
};

impl App {
    pub fn counter_view(&self) -> Element<'_, Message> {
        let animated_text = AnimationBuilder::new(self.val, |val| {
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
        widget::container(
            widget::column![
                widget::row![
                    widget::button("Inc").on_press(Message::CounterIncrement),
                    widget::button("Dec").on_press(Message::CounterDecreasement),
                    widget::button("Go to Launch Page")
                        .on_press(Message::PageJump(ViewPage::Launch)),
                    animated_size_text,
                ]
                .spacing(10),
                animated_text
            ]
            .spacing(10),
        )
        .padding(10)
        .into()
    }

    pub fn counter_update(&mut self, message: Message) -> Task<Message> {
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
            Message::PageJump(next_page) => {
                self.handle_page_jump(next_page);
            }

            Message::CounterIncrement => {
                self.val += 10.0;
            }
            Message::CounterDecreasement => {
                if self.val >= 20.0 {
                    self.val -= 10.0;
                }
            }
        }
        Task::none()
    }
}

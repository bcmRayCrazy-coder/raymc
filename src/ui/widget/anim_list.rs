use std::{fmt::Display, sync::Arc};

use iced::{
    Alignment, Color, Element,
    Length::Fill,
    Padding, Renderer, Theme,
    widget::{self},
};
use iced_anim::{Animated, AnimationBuilder, Easing, Event, animation::animation};

use crate::ui::message::Message;

#[derive(Debug, Clone, PartialEq)]
pub enum AnimListEvent {
    UpdatePaddingY(Event<f32>),
}

#[derive(Clone)]
pub struct AnimList<'a, T>
// where
// T: Clone,
{
    pub list: Vec<T>,
    pub current: usize,
    pub spacing: f32,
    pub disabled: bool,
    anim_padding_y: Animated<f32>,

    display: Arc<dyn Fn(T, f32) -> Element<'a, Message, Theme, Renderer> + Send + Sync>,

    on_update: Option<Arc<dyn Fn(AnimListEvent) -> Message + Send + Sync>>,
    evulate_y: fn(f32) -> f32,
}

impl<'a, T> AnimList<'a, T>
where
    T: Clone,
{
    fn widget_list(&self) -> Element<'_, Message, Theme, Renderer> {
        let mut widget_list = widget::Column::new();

        for (index, item) in self.list.iter().enumerate() {
            let item_transition: f32 = if !self.disabled && index == self.current {
                1.0
            } else {
                0.0
            };
            let fn_display = self.display.clone();
            let animated_item =
                AnimationBuilder::new(item_transition, move |item_transition_val| {
                    fn_display(item.clone(), item_transition_val)
                })
                .animates_layout(true)
                .animation(Easing::EASE_IN_OUT.quick());

            widget_list = widget_list
                .push(animated_item)
                .push(widget::space().height(self.spacing));
        }

        widget_list.into()
    }

    pub fn widget(&self) -> Element<'_, Message, Theme, Renderer> {
        let on_update = self.on_update.clone();

        animation(
            &self.anim_padding_y,
            widget::container(
                widget::container(self.widget_list())
                    .align_x(Alignment::Start)
                    .padding(
                        Padding::new(0.0).top(iced::Pixels::from(*self.anim_padding_y.value())),
                    )
                    .height(Fill),
            )
            .height(Fill),
        )
        .on_update(move |e| match &on_update {
            Some(f) => f(AnimListEvent::UpdatePaddingY(e)),
            None => Message::None,
        })
        .into()
    }

    pub fn update(&mut self, event: AnimListEvent) {
        match event {
            AnimListEvent::UpdatePaddingY(e) => self.anim_padding_y.update(e),
        }
    }

    pub fn scroll_next(&mut self) -> bool {
        if !self.disabled && self.current < (self.list.len() - 1) {
            self.current += 1;
            self.update_scroll();
            return true;
        }
        false
    }

    pub fn scroll_prev(&mut self) -> bool {
        if !self.disabled && self.current >= 1 {
            self.current -= 1;
            self.update_scroll();
            return true;
        }
        false
    }

    fn update_scroll(&mut self) {
        let highlight_y = (self.evulate_y)(self.current as f32);
        self.anim_padding_y
            .update(iced_anim::Event::Target(highlight_y * -1.0));
    }

    pub fn reset_current(&mut self) {
        self.current = 0;
        self.update_scroll();
    }

    pub fn list(mut self, list: Vec<T>) -> Self {
        self.list = list;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_update<U: Fn(AnimListEvent) -> Message + Send + Sync + 'static>(
        mut self,
        build_message: U,
    ) -> Self {
        self.on_update = Some(Arc::new(build_message));
        self
    }

    pub fn current(&self) -> &T {
        &self.list[self.current]
    }

    pub fn current_index(&self) -> usize {
        self.current
    }
}

impl<T> Default for AnimList<'_, T>
where
    T: Display,
{
    fn default() -> Self {
        // let style_default = (30.0, Color::from_rgba(0.85, 0.85, 0.85, 0.68));
        // let style_highlight = (50.0, Color::from_rgba(0.9, 0.9, 0.9, 0.98));
        Self {
            list: Vec::new(),
            current: 0,
            spacing: 22.0,
            disabled: false,

            anim_padding_y: Animated::transition(0.0, Easing::EASE_IN_OUT.quick()),

            display: Arc::new(|item, transition_val| {
                widget::text(format!("{}", item))
                    .height(transition_val * 20.0 + 30.0)
                    .size(transition_val * 20.0 + 30.0)
                    .color(Color::from_rgba(
                        0.85 + transition_val * 0.05,
                        0.85 + transition_val * 0.05,
                        0.85 + transition_val * 0.05,
                        0.68 + transition_val * 0.3,
                    ))
                    .wrapping(widget::text::Wrapping::None)
                    .into()
            }),
            on_update: None,
            evulate_y: |i| (22.0 + 30.0) * i,
        }
    }
}

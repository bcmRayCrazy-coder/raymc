use std::sync::Arc;

use iced::{
    Color, Element,
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
    list: Vec<T>,
    current: usize,
    anim_padding_y: Animated<f32>,

    display: Arc<dyn Fn(T, f32) -> Element<'a, Message, Theme, Renderer> + Send + Sync>,

    on_update: Option<Arc<dyn Fn(AnimListEvent) -> Message + Send + Sync>>,
    evulate_y: fn(f32) -> f32,
    // style_default: (f32, Color),
    // style_highlight: (f32, Color),
}

impl<'a, T> AnimList<'a, T>
where
    T: Clone,
{
    // pub fn new() -> Self {
    //     Self {
    //         list: vec!["".to_owned()],
    //         current: 0,
    //         anim_padding_y: Animated::transition(0.0, Easing::EASE_IN_OUT.quick()),

    //         on_update: None,
    //         evulate_y: |i| (10.0 + 38.7) * i,
    //         style_default: (30.0, Color::from_rgba(0.85, 0.85, 0.85, 0.68)),
    //         style_highlight: (50.0, Color::from_rgba(0.9, 0.9, 0.9, 0.98)),
    //     }
    // }

    fn widget_list(&self) -> Element<'_, Message, Theme, Renderer> {
        let mut widget_list = widget::Column::new();

        for (index, item) in self.list.iter().enumerate() {
            // let item_style = if index == self.current {
            //     self.style_highlight.clone()
            // } else {
            //     self.style_default.clone()
            // };
            let item_transition: f32 = if index == self.current { 1.0 } else { 0.0 };
            let fn_display = self.display.clone();
            let animated_item =
                AnimationBuilder::new(item_transition, move |item_transition_val| {
                    // widget::text(item.clone())
                    //     .size(item_size_val)
                    //     .color(item_color_val)
                    //     .into()
                    // self.display(item.clone(), item_transition_val)
                    fn_display(item.clone(), item_transition_val)
                })
                .animates_layout(true)
                .animation(Easing::EASE_IN_OUT.quick());

            widget_list = widget_list
                .push(animated_item)
                .push(widget::space().height(10));
        }

        widget_list.into()
    }

    pub fn widget(&self) -> Element<'_, Message, Theme, Renderer> {
        let on_update = self.on_update.clone();

        animation(
            &self.anim_padding_y,
            widget::container(self.widget_list())
                .padding(
                    Padding::new(0.0)
                        .bottom(0)
                        .top(iced::Pixels::from(*self.anim_padding_y.value())),
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
        if self.current < (self.list.len() - 1) {
            self.current += 1;
            self.update_scroll();
            return true;
        }
        false
    }

    pub fn scroll_prev(&mut self) -> bool {
        if self.current >= 1 {
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

    pub fn list(mut self, list: Vec<T>) -> Self {
        self.list = list;
        self
    }

    pub fn on_update<U: Fn(AnimListEvent) -> Message + Send + Sync + 'static>(
        mut self,
        build_message: U,
    ) -> Self {
        self.on_update = Some(Arc::new(build_message));
        self
    }

    // pub fn style_default(mut self, size: f32, color: Color) -> Self {
    //     self.style_default = (size, color);
    //     self
    // }

    // pub fn style_highlight(mut self, size: f32, color: Color) -> Self {
    //     self.style_highlight = (size, color);
    //     self
    // }

    pub fn current(&self) -> usize {
        self.current
    }
}

impl Default for AnimList<'_, String> {
    fn default() -> Self {
        // let style_default = (30.0, Color::from_rgba(0.85, 0.85, 0.85, 0.68));
        // let style_highlight = (50.0, Color::from_rgba(0.9, 0.9, 0.9, 0.98));
        Self {
            list: vec!["".to_owned()],
            current: 0,
            anim_padding_y: Animated::transition(0.0, Easing::EASE_IN_OUT.quick()),

            display: Arc::new(|item, transition_val| {
                widget::text(item)
                    .size(transition_val * 20.0 + 30.0)
                    .color(Color::from_rgba(
                        0.85 + transition_val * 0.05,
                        0.85 + transition_val * 0.05,
                        0.85 + transition_val * 0.05,
                        0.68 + transition_val * 0.3,
                    ))
                    .into()
            }),
            on_update: None,
            evulate_y: |i| (10.0 + 38.7) * i,
        }
    }
}

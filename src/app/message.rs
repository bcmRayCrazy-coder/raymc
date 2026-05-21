use iced::Size;
use iced_anim::Event;

use crate::app::app::{QuickKey, ViewPageName};

#[derive(Debug, Clone)]
pub enum Message {
    QuickKeyAction(QuickKey),

    // Page
    OnPageShow,

    ActionPageJump(ViewPageName),
    ActionPageBack,

    // Window
    OnWindowResize(Size),

    Counter(CounterMessage),
    Menu(MenuMessage),
}

#[derive(Debug, Clone)]

pub enum MenuMessage {
    UpdatePaddingY(Event<f32>),
}

#[derive(Debug, Clone)]
pub enum CounterMessage {
    HideMsg,
    Increment,
    Decreasement,
}

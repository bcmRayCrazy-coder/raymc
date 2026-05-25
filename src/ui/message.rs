use iced::{Size, window};
use iced_anim::Event;

use crate::ui::app::{QuickKey, ViewPageName};

#[derive(Debug, Clone)]
pub enum Message {
    QuickKeyAction(QuickKey),

    // Page
    OnPageShow,

    ActionPageJump(ViewPageName),
    ActionPageBack,
    ActionQuit,

    // Window
    OnWindowOpen(window::Id),
    OnWindowResize(Size),

    Counter(CounterMessage),
    Menu(MenuMessage),
}

#[derive(Debug, Clone)]

pub enum MenuMessage {
    UpdateIconScale(Event<f32>),
    UpdatePaddingY(Event<f32>),
    ConfirmSelect,
}

#[derive(Debug, Clone)]
pub enum CounterMessage {
    HideMsg,
    Increment,
    Decreasement,
}

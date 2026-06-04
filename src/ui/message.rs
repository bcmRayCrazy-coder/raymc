use iced::{Size, window};
use iced_anim::Event;

use crate::ui::{
    app::{QuickKey, ViewPageName},
    widget::anim_list::AnimListEvent,
};

#[derive(Debug, Clone)]
pub enum Message {
    QuickKeyAction(QuickKey),

    // Audio
    Audio(AudioMessage),

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
    Album(AlbumMessage),

    None,
}

#[derive(Debug, Clone)]
pub enum AudioMessage {
    PlayUi(&'static str),
}

#[derive(Debug, Clone)]
pub enum MenuMessage {
    UpdateIconScale(Event<f32>),
    UpdateAnimPageTransition(Event<f32>),
    UpdateAnimList(AnimListEvent),
    ConfirmSelect,
}

#[derive(Debug, Clone)]
pub enum AlbumMessage {
    UpdateAnimList(AnimListEvent),
    UpdateAnimPageTransition(Event<f32>),
    ConfirmSelect,
}

#[derive(Debug, Clone)]
pub enum CounterMessage {
    HideMsg,
    Increment,
    Decreasement,
}

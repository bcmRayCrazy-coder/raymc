use iced::{Size, window};
use iced_anim::Event;

use crate::ui::{
    app::{QuickKey, ViewPageName},
    page::album::AlbumName,
    widget::anim_list::AnimListEvent,
};

#[derive(Debug, Clone)]
pub enum Message {
    QuickKeyAction(QuickKey),

    // Audio
    Audio(AudioMessage),

    // Player
    Player(PlayerMessage),

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
pub enum PlayerMessage {
    LoopNext,
    ListNext,
    ListPrev,
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
    UpdateAnimAlbumList(AnimListEvent),
    UpdateAnimSongList(AnimListEvent),
    UpdateAnimPageTransition(Event<f32>),
    ConfirmSelect,

    LoadAlbums,
    LoadSongs(AlbumName),
}

#[derive(Debug, Clone)]
pub enum CounterMessage {
    HideMsg,
    Increment,
    Decreasement,
}

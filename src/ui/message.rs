use std::path::PathBuf;

use iced::{Size, futures::channel::mpsc, window};
use iced_anim::Event;

use crate::{
    audio::track::AudioTrackType,
    player::album::AlbumName,
    ui::{
        app::{QuickKey, ViewPageName},
        widget::anim_list::AnimListEvent,
    },
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
    AudioMpscReady(mpsc::Sender<Message>),

    TrackEnd(AudioTrackType),

    PlayUi(&'static str),
    UpdatePlayerSong,
    PlayerPlay,
    PlayerPause,
}

#[derive(Debug, Clone)]
pub enum PlayerMessage {
    LoopNext,
    ListNext,
    ListPrev,

    PlayEnd,

    InsertJumpNext(PathBuf),
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

    /* bool - refresh after loaded */
    LoadAlbums(bool),
    LoadSongs(AlbumName),
}

#[derive(Debug, Clone)]
pub enum CounterMessage {
    HideMsg,
    Increment,
    Decreasement,
}

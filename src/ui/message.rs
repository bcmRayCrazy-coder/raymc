use std::path::PathBuf;

use iced::{Size, futures::channel::mpsc, window};
use iced_anim::Event;

use crate::{
    audio::track::AudioTrackType,
    player::{album::AlbumName, song::PlaySong},
    ui::{
        app::{QuickKey, ViewPageName},
        state::AppState,
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

    // State
    State(StateMessage),
    UpdatePageState(Box<AppState>),

    // Page
    OnPageShow,

    ActionPageJump(ViewPageName),
    ActionPageBack,
    ActionQuit,
    ActionUpdateKeysHint,

    // Window
    OnWindowOpen(window::Id),

    Counter(CounterMessage),
    Menu(MenuMessage),
    Album(AlbumMessage),
    PlayerPage(PlayerPageMessage),

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

    InsertJumpNext(PlaySong),
    InsertJumpNextAlbum(AlbumName, PathBuf),
}

#[derive(Debug, Clone)]
pub enum StateMessage {
    Fetch,

    OnWindowResize(Size),
    OnPlayStateChanged(bool),
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
pub enum PlayerPageMessage {
    TogglePlay,
}

#[derive(Debug, Clone)]
pub enum CounterMessage {
    HideMsg,
    Increment,
    Decreasement,
}

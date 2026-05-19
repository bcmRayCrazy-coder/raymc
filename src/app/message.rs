use crate::app::app::{QuickKey, ViewPageName};

#[derive(Debug, Clone)]
pub enum Message {
    QuickKeyAction(QuickKey),
    ViewPageManager(ViewPageManagerMessage),

    PageShow,

    Counter(CounterMessage)
}

#[derive(Debug, Clone)]
pub enum ViewPageManagerMessage {
    PageJump(ViewPageName),
    PageBack,
}

#[derive(Debug,Clone)]
pub enum CounterMessage {
    HideMsg,
    Increment,
    Decreasement
}

use crate::app::app::{QuickKey, ViewPageName};

#[derive(Debug, Clone)]
pub enum Message {
    QuickKeyAction(QuickKey),

    OnPageShow,
    ActionPageJump(ViewPageName),
    ActionPageBack,

    Counter(CounterMessage),
}

#[derive(Debug, Clone)]
pub enum CounterMessage {
    HideMsg,
    Increment,
    Decreasement,
}

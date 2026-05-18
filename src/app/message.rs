use crate::app::app::{QuickKey, ViewPageName};

#[derive(Debug, Clone)]
pub enum Message {
    QuickKeyAction(QuickKey),
    // PageJump(ViewPageName),
    ViewPageManager(ViewPageManagerMessage),

    CounterIncrement,
    CounterDecreasement,
}

#[derive(Debug, Clone)]
pub enum ViewPageManagerMessage {
    PageJump(ViewPageName),
    PageBack,
}

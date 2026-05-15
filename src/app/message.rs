use crate::app::app::{QuickKey, ViewPageName};

#[derive(Debug, Clone)]
pub enum Message {
    QuickKeyAction(QuickKey),
    PageJump(ViewPageName),

    CounterIncrement,
    CounterDecreasement,
}

use crate::app::app::{QuickKey, ViewPage};

#[derive(Debug, Clone)]
pub enum Message {
    QuickKeyAction(QuickKey),
    PageJump(ViewPage),

    CounterIncrement,
    CounterDecreasement,
}

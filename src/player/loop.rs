#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum PlayerLoopMode {
    SingleLoop,
    SingleOnce,
    ListOnce,
    ListSorted,
    ListRandom,
}

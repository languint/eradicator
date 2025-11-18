use crate::modes::Gamemode;

pub enum QueueType {
    Party,
    Matchmake,
}

pub enum QueueStatus {
    GamemodeSelecting,
    WaitingForServer,
    Teleporting,
}

pub struct MatchmakingStatus {
    pub queue_type: Option<QueueType>,
    pub queue_status: Option<QueueStatus>,
    pub gamemode: Option<Gamemode>,
}
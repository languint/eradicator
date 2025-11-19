use eradicator_core::{defs::Location, matchmaking::MatchmakingStatus};

pub struct AppState {
    pub location: Option<Location>,
    pub matchmaking_status: MatchmakingStatus,
}
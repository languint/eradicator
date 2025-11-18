use crate::towers::Tower;

pub struct NrOf;
impl NrOf {
    pub const TOWERS: usize = 51;
    pub const PATHS: usize = 2;
    pub const PATH_UPGRADE_COUNT: usize = 5;

    pub const LOADOUT_SLOTS: usize = 6;
}

pub enum Location {
    Lobby,
    Game,
}

pub type PlayerLoadout = [Option<Tower>; NrOf::LOADOUT_SLOTS];

pub const EMPTY_LOADOUT: PlayerLoadout = [None, None, None, None, None, None];

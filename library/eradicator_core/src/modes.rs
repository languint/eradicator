pub enum StandardMode {
    Easy,
    Intermediate,
    Elite,
    Expert,
    Endless,
    Nightmare,
    Ultra,
}

pub enum EventMode {
    XMASNormal,
    XMASNightmare,
    Chirstmas1Normal,
    Chirstmas1Nightmare,
    Chirstmas2Normal,
    Chirstmas2Nightmare,
    Halloween1Normal,
    Halloween1Nightmare,
    Halloween2Normal,
    Halloween2Nightmare,
    Halloween3Normal,
    Halloween3Nightmare,
    Halloween4Normal,
    Halloween4Nightmare,
    TowerBattlesNormal,
    TowerBattlesNightmare,
}

pub enum Gamemode {
    Standard(StandardMode),
    Event(EventMode)
}
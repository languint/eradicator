#[derive(Debug, Clone)]
pub enum StandardMode {
    Easy,
    Intermediate,
    Elite,
    Expert,
    Endless,
    Nightmare,
    Ultra,
}

#[derive(Debug, Clone)]
pub enum EventMode {
    XMASNormal,
    XMASNightmare,
    Christmas1Normal,
    Christmas1Nightmare,
    Christmas2Normal,
    Christmas2Nightmare,
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

#[derive(Debug, Clone)]
pub enum Gamemode {
    Standard(StandardMode),
    Event(EventMode),
}

impl Default for Gamemode {
    fn default() -> Self {
        Self::Standard(StandardMode::Easy)
    }
}

impl TryFrom<&str> for Gamemode {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Easy" => Ok(Gamemode::Standard(StandardMode::Easy)),
            "Intermediate" => Ok(Gamemode::Standard(StandardMode::Intermediate)),
            "Elite" => Ok(Gamemode::Standard(StandardMode::Elite)),
            "Expert" => Ok(Gamemode::Standard(StandardMode::Expert)),
            "Endless" => Ok(Gamemode::Standard(StandardMode::Endless)),
            "Nightmare" => Ok(Gamemode::Standard(StandardMode::Nightmare)),
            "Ultra" => Ok(Gamemode::Standard(StandardMode::Ultra)),
            "XMASNormal" => Ok(Gamemode::Event(EventMode::XMASNormal)),
            "XMASNightmare" => Ok(Gamemode::Event(EventMode::XMASNightmare)),
            "Christmas1Normal" => Ok(Gamemode::Event(EventMode::Christmas1Normal)),
            "Christmas1Nightmare" => Ok(Gamemode::Event(EventMode::Christmas1Nightmare)),
            "Christmas2Normal" => Ok(Gamemode::Event(EventMode::Christmas2Normal)),
            "Christmas2Nightmare" => Ok(Gamemode::Event(EventMode::Christmas2Nightmare)),
            "Halloween1Normal" => Ok(Gamemode::Event(EventMode::Halloween1Normal)),
            "Halloween1Nightmare" => Ok(Gamemode::Event(EventMode::Halloween1Nightmare)),
            "Halloween2Normal" => Ok(Gamemode::Event(EventMode::Halloween2Normal)),
            "Halloween2Nightmare" => Ok(Gamemode::Event(EventMode::Halloween2Nightmare)),
            "Halloween3Normal" => Ok(Gamemode::Event(EventMode::Halloween3Normal)),
            "Halloween3Nightmare" => Ok(Gamemode::Event(EventMode::Halloween3Nightmare)),
            "Halloween4Normal" => Ok(Gamemode::Event(EventMode::Halloween4Normal)),
            "Halloween4Nightmare" => Ok(Gamemode::Event(EventMode::Halloween4Nightmare)),
            "TowerBattlesNormal" => Ok(Gamemode::Event(EventMode::TowerBattlesNormal)),
            "TowerBattlesNightmare" => Ok(Gamemode::Event(EventMode::TowerBattlesNightmare)),
            _ => Err(format!("{value} is not a valid Gamemode!")),
        }
    }
}

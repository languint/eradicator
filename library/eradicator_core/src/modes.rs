use std::fmt;

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

impl fmt::Display for StandardMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Easy => write!(f, "Easy"),
            Self::Intermediate => write!(f, "Intermediate"),
            Self::Elite => write!(f, "Elite"),
            Self::Expert => write!(f, "Expert"),
            Self::Endless => write!(f, "Endless"),
            Self::Nightmare => write!(f, "Nightmare"),
            Self::Ultra => write!(f, "Ultra"),
        }
    }
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

impl fmt::Display for EventMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::XMASNormal => write!(f, "XMASNormal"),
            Self::XMASNightmare => write!(f, "XMASNightmare"),
            Self::Christmas1Normal => write!(f, "Christmas1Normal"),
            Self::Christmas1Nightmare => write!(f, "Christmas1Nightmare"),
            Self::Christmas2Normal => write!(f, "Christmas2Normal"),
            Self::Christmas2Nightmare => write!(f, "Christmas2Nightmare"),
            Self::Halloween1Normal => write!(f, "Halloween1Normal"),
            Self::Halloween1Nightmare => write!(f, "Halloween1Nightmare"),
            Self::Halloween2Normal => write!(f, "Halloween2Normal"),
            Self::Halloween2Nightmare => write!(f, "Halloween2Nightmare"),
            Self::Halloween3Normal => write!(f, "Halloween3Normal"),
            Self::Halloween3Nightmare => write!(f, "Halloween3Nightmare"),
            Self::Halloween4Normal => write!(f, "Halloween4Normal"),
            Self::Halloween4Nightmare => write!(f, "Halloween4Nightmare"),
            Self::TowerBattlesNormal => write!(f, "TowerBattlesNormal"),
            Self::TowerBattlesNightmare => write!(f, "TowerBattlesNightmare"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Gamemode {
    Standard(StandardMode),
    Event(EventMode),
}

impl fmt::Display for Gamemode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Standard(s) => s.fmt(f),
            Self::Event(e) => e.fmt(f),
        }
    }
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

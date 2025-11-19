// mode=Easy
// map=Baseplate
// loadout=Shotgunner, Juggernaut, EDJ
// ---
// Shotgunner 100 1000
// Shotgunner 100 1292
// Juggernaut 422 2041
// Juggernaut 232 2311
// EDJ 312 120
// ---
// place 1
// place 2
// upgrade 1 BOTTOM
// upgrade 2 BOTTOM
// @200{ upgrade 1 TOP }
// @200{ upgrade 2 TOP }
// @375{ upgrade 1 TOP }
// @1950{ upgrade 1 TOP }
// @2400{ upgrade 1 TOP }
// @325{upgrade 1 BOTTOM }
// @375{ upgrade 2 TOP }
// @1950{ upgrade 2 TOP }
// @2400{ upgrade 2 TOP }
// @325{upgrade 2 BOTTOM }

use crate::{defs::PlayerLoadout, maps::Maps, modes::Gamemode, towers::Tower};

#[derive(Debug, Default)]
pub struct StrategyHeader {
    pub mode: Gamemode,
    pub map: Maps,
    pub loadout: PlayerLoadout,
}

pub type StrategyTowerLocationTable = Vec<(String, (u32, u32))>;

#[derive(Debug)]
pub struct StrategyActions;

#[derive(Debug)]
pub struct Strategy {
    pub header: StrategyHeader,
    pub tower_location_table: StrategyTowerLocationTable,
    pub actions: StrategyActions,
}

impl Default for Strategy {
    fn default() -> Self {
        Self {
            header: StrategyHeader::default(),
            tower_location_table: vec![],
            actions: StrategyActions {},
        }
    }
}

#[derive(Debug)]
pub enum StrategyParsingError {
    MismatchedSections(String),
    InvalidHeader(String),
    UnexpectedEOF,
}

#[derive(Debug)]
pub struct StrategyParser<'a> {
    pub src: &'a str,
    pub lines: Vec<&'a str>,
    pub loc: usize,
}

impl<'a> StrategyParser<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            lines: src.split('\n').collect(),
            loc: 0,
        }
    }

    fn advance(&mut self) -> bool {
        if self.src.len() > self.loc + 1 {
            self.loc += 1;
            return true;
        }

        false
    }

    fn is_at_end(&self) -> bool {
        self.loc == self.lines.len()
    }
}

impl<'a> StrategyParser<'a> {
    pub fn parse(&mut self) -> Result<Strategy, StrategyParsingError> {
        let mut sections: Vec<Vec<&str>> = vec![];

        while !self.is_at_end() {
            let mut section: Vec<&str> = vec![];

            while let Some(line) = self.lines.get(self.loc) {
                if line.trim() == "---" {
                    self.advance();
                    break;
                }

                section.push(line);
                self.advance();
            }

            sections.push(section)
        }

        if sections.len() != 3 {
            return Err(StrategyParsingError::MismatchedSections(format!(
                "Expected 3 sections, got {}",
                sections.len()
            )));
        }

        let mut strategy = Strategy::default();

        strategy.header = self.parse_header(&sections[0])?;

        Ok(strategy)
    }

    fn parse_header(
        &mut self,
        section: &Vec<&str>,
    ) -> Result<StrategyHeader, StrategyParsingError> {
        let mut header: StrategyHeader = StrategyHeader::default();

        for (index, line) in section.iter().enumerate() {
            let parts: Vec<&str> = line.trim().split('=').collect();

            if parts.len() != 2 {
                return Err(StrategyParsingError::InvalidHeader(format!(
                    "Line {index} does not have 2 parts, got {}",
                    parts.len()
                )));
            }

            match parts[0] {
                "mode" => {
                    if let Ok(mode) = Gamemode::try_from(parts[1]) {
                        header.mode = mode;
                    } else {
                        return Err(StrategyParsingError::InvalidHeader(format!(
                            "Unknown mode: `{}`",
                            parts[1]
                        )));
                    }
                }
                "map" => {
                    if let Ok(map) = Maps::try_from(parts[1]) {
                        header.map = map;
                    } else {
                        return Err(StrategyParsingError::InvalidHeader(format!(
                            "Unknown map: `{}`",
                            parts[1]
                        )));
                    }
                }
                "loadout" => {
                    for (index, tower_part) in parts[1].split(',').into_iter().enumerate() {
                        if let Ok(tower) = Tower::try_from(tower_part.trim()) {
                            header.loadout[index] = Some(tower)
                        }
                    }
                }
                _ => {
                    return Err(StrategyParsingError::InvalidHeader(format!(
                        "Unknown key: `{}`",
                        parts[0]
                    )));
                }
            }
        }

        Ok(header)
    }
}

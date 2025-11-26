use core::fmt;

use crate::{
    defs::{PlayerLoadout, UpgradePath},
    maps::Maps,
    modes::Gamemode,
    towers::Tower,
};

#[derive(Debug, Default)]
pub struct StrategyHeader {
    pub mode: Gamemode,
    pub map: Maps,
    pub loadout: PlayerLoadout,
}

pub type StrategyTowerLocationTable = Vec<(Tower, (u32, u32))>;

pub type StrategyActionBlock = Vec<StrategyAction>;

#[derive(Debug)]
pub enum StrategyAction {
    Place(u32),
    Upgrade(u32, UpgradePath),
    AtCash(u32, Box<StrategyActionBlock>),
}

#[derive(Debug)]
pub struct StrategyActions {
    pub actions: Vec<StrategyAction>,
}

impl StrategyActions {
    pub fn push(&mut self, action: StrategyAction) {
        self.actions.push(action);
    }
}

impl Default for StrategyActions {
    fn default() -> Self {
        Self { actions: vec![] }
    }
}

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
            actions: StrategyActions::default(),
        }
    }
}

#[derive(Debug)]
pub enum StrategyParsingError {
    MismatchedSections(String),
    InvalidHeader(String),
    InvalidTowerLocationTable(String),
    InvalidActions(String),
    UnexpectedEOF,
}

impl fmt::Display for StrategyParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MismatchedSections(s) => write!(f, "Mismatched sections: {s}"),
            Self::InvalidHeader(s) => write!(f, "Invalid header: {s}"),
            Self::InvalidTowerLocationTable(s) => write!(f, "Invalid tower location table: {s}"),
            Self::InvalidActions(s) => write!(f, "Invalid actions: {s}"),
            Self::UnexpectedEOF => write!(f, "Unexpected EOF"),
        }
    }
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
        strategy.tower_location_table = self.parse_tower_location_table(&sections[1])?;
        strategy.actions = self.parse_actions(&sections[2])?;

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

    fn parse_tower_location_table(
        &self,
        section: &Vec<&str>,
    ) -> Result<StrategyTowerLocationTable, StrategyParsingError> {
        let mut table: StrategyTowerLocationTable = StrategyTowerLocationTable::default();

        for (index, line) in section.iter().enumerate() {
            let parts: Vec<&str> = line.trim().split_whitespace().collect();

            if parts.len() != 3 {
                return Err(StrategyParsingError::InvalidTowerLocationTable(format!(
                    "Line {index} does not have 3 parts, got {}",
                    parts.len()
                )));
            }

            let tower = Tower::try_from(parts[0]).map_err(|_| {
                StrategyParsingError::InvalidTowerLocationTable(format!(
                    "Line {index} has an unknown tower: {}",
                    parts[0]
                ))
            })?;

            let x = parts[1].parse::<u32>().map_err(|_| {
                StrategyParsingError::InvalidTowerLocationTable(format!(
                    "Line {index} has invalid X parameter: {}",
                    parts[1]
                ))
            })?;
            let y = parts[2].parse::<u32>().map_err(|_| {
                StrategyParsingError::InvalidTowerLocationTable(format!(
                    "Line {index} has invalid Y parameter: {}",
                    parts[2]
                ))
            })?;

            table.push((tower, (x, y)));
        }

        Ok(table)
    }
}

impl<'a> StrategyParser<'a> {
    fn parse_action_block(
        &self,
        lines: &[&str],
    ) -> Result<StrategyActionBlock, StrategyParsingError> {
        let mut actions: StrategyActionBlock = vec![];

        for (index, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed == "}" {
                continue;
            }

            let parts: Vec<&str> = trimmed.split_whitespace().collect();

            match parts[0] {
                "place" => {
                    if parts.len() != 2 {
                        return Err(StrategyParsingError::InvalidActions(format!(
                            "Line {index}: `place` expects 1 argument"
                        )));
                    }
                    let id = parts[1].parse::<u32>().map_err(|_| {
                        StrategyParsingError::InvalidActions(format!(
                            "Line {index}: invalid tower id `{}`",
                            parts[1]
                        ))
                    })?;
                    actions.push(StrategyAction::Place(id));
                }

                "upgrade" => {
                    if parts.len() != 3 {
                        return Err(StrategyParsingError::InvalidActions(format!(
                            "Line {index}: `upgrade` expects 2 arguments"
                        )));
                    }
                    let id = parts[1].parse::<u32>().map_err(|_| {
                        StrategyParsingError::InvalidActions(format!(
                            "Line {index}: invalid tower id `{}`",
                            parts[1]
                        ))
                    })?;

                    let path = UpgradePath::try_from(parts[2]).map_err(|_| {
                        StrategyParsingError::InvalidActions(format!(
                            "Line {index}: invalid upgrade path `{}`",
                            parts[2]
                        ))
                    })?;

                    actions.push(StrategyAction::Upgrade(id, path));
                }

                other => {
                    return Err(StrategyParsingError::InvalidActions(format!(
                        "Unknown action `{other}` on line {index}"
                    )));
                }
            }
        }

        Ok(actions)
    }

    pub fn parse_actions(
        &self,
        section: &Vec<&str>,
    ) -> Result<StrategyActions, StrategyParsingError> {
        let mut actions = StrategyActions::default();
        let mut i = 0;

        while i < section.len() {
            let line = section[i].trim();
            if line.is_empty() {
                i += 1;
                continue;
            }

            if line.starts_with('@') {
                let after_at = &line[1..];
                let (cash_str, _) =
                    after_at.split_at(after_at.find(|c: char| c == '{').unwrap_or(after_at.len()));

                let cash = cash_str.trim().parse::<u32>().map_err(|_| {
                    StrategyParsingError::InvalidActions(format!(
                        "Invalid @cash value: `{cash_str}`"
                    ))
                })?;

                let mut block_lines: Vec<&str> = vec![];

                if line.contains('{') {
                    let mut open_braces = 0usize;
                    for c in line.chars() {
                        if c == '{' {
                            open_braces += 1;
                        } else if c == '}' {
                            open_braces -= 1;
                        }
                    }

                    if line.contains('}') && open_braces == 0 {
                        let inside =
                            line[line.find('{').unwrap() + 1..line.rfind('}').unwrap()].trim();
                        if !inside.is_empty() {
                            block_lines.extend(
                                inside
                                    .split(';')
                                    .map(|v| v.trim())
                                    .filter(|v| !v.is_empty()),
                            );
                        }
                        i += 1;
                    } else {
                        i += 1;
                        open_braces = 1;
                        while i < section.len() && open_braces > 0 {
                            let l = section[i];
                            for c in l.chars() {
                                if c == '{' {
                                    open_braces += 1;
                                }
                                if c == '}' {
                                    open_braces -= 1;
                                }
                            }
                            if open_braces > 0 {
                                block_lines.push(l);
                            }
                            i += 1;
                        }
                    }
                } else {
                    return Err(StrategyParsingError::InvalidActions(
                        "Expected `{` after @cash".into(),
                    ));
                }

                let parsed_block = self.parse_action_block(&block_lines)?;
                actions.push(StrategyAction::AtCash(cash, Box::new(parsed_block)));

                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();

            match parts[0] {
                "place" => {
                    if parts.len() != 2 {
                        return Err(StrategyParsingError::InvalidActions(format!(
                            "`place` expects 1 arg"
                        )));
                    }
                    actions.push(StrategyAction::Place(parts[1].parse::<u32>().map_err(
                        |_| StrategyParsingError::InvalidActions("Invalid tower id".into()),
                    )?));
                }

                "upgrade" => {
                    if parts.len() != 3 {
                        return Err(StrategyParsingError::InvalidActions(format!(
                            "`upgrade` expects 2 args"
                        )));
                    }
                    let id = parts[1].parse::<u32>().map_err(|_| {
                        StrategyParsingError::InvalidActions("Invalid tower id".into())
                    })?;
                    let path = UpgradePath::try_from(parts[2]).map_err(|_| {
                        StrategyParsingError::InvalidActions("Invalid upgrade path".into())
                    })?;
                    actions.push(StrategyAction::Upgrade(id, path));
                }

                other => {
                    return Err(StrategyParsingError::InvalidActions(format!(
                        "Unknown action `{other}`"
                    )));
                }
            }

            i += 1;
        }

        Ok(actions)
    }
}

// TODO: Expand to include all maps from the TDX wiki.
#[derive(Debug, Clone)]
pub enum Maps {
    Baseplate,
    DesertedIsland,
}

impl Default for Maps {
    fn default() -> Self {
        Maps::Baseplate
    }
}

impl TryFrom<&str> for Maps {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Baseplate" => Ok(Maps::Baseplate),
            "Deserted Island" => Ok(Maps::DesertedIsland),
            _ => Err(format!("Unknown map `{}`", value)),
        }
    }
}

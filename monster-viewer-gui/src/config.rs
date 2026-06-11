use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Config {
    pub always_on_top: bool,
    pub background_opacity: u8,
    pub highlight_changes: bool,
    pub visibility: Visibility,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            always_on_top: true,
            background_opacity: 80,
            highlight_changes: true,
            visibility: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Visibility {
    pub health: bool,
    pub attack: bool,
    pub defense: bool,
    pub parts: bool,
    pub status: bool,
}

impl Default for Visibility {
    fn default() -> Self {
        Self {
            health: true,
            attack: true,
            defense: true,
            parts: true,
            status: true,
        }
    }
}

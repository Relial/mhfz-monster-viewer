use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Config {
    pub always_on_top: bool,
    pub background_opacity: u8,
    pub highlight_changes: bool,
    pub visibility: Visibility,
    pub timer_direction: TimerDirection,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            always_on_top: true,
            background_opacity: 80,
            highlight_changes: true,
            visibility: Default::default(),
            timer_direction: TimerDirection::CountUp,
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
    pub quest_time: bool,
}

impl Default for Visibility {
    fn default() -> Self {
        Self {
            health: true,
            attack: true,
            defense: true,
            parts: true,
            status: true,
            quest_time: true,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TimerDirection {
    CountUp,
    CountDown,
}

impl std::fmt::Display for TimerDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TimerDirection::CountUp => "Count up",
            TimerDirection::CountDown => "Count down",
        };
        write!(f, "{s}")
    }
}

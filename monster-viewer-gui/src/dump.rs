use serde::{Deserialize, Serialize};

use crate::game_data::HitzoneValues;

#[derive(Serialize, Deserialize)]
pub struct Monsters(Vec<Monster>);

#[derive(Serialize, Deserialize)]
pub struct Monster {
    pub id: u8,
    pub unique_states: Vec<MonsterState>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MonsterState {
    pub parts: Vec<Part>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Part {
    pub part_idx: u16,
    pub hzv_idx: u16,
    pub hzvs: HitzoneValues,
}

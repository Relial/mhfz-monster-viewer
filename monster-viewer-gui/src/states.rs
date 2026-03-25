use rapidhash::RapidHashSet;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::{
    game_data::{HitzoneValues, MonsterPart},
    ui::HzvColumn,
};

#[derive(Serialize, Deserialize)]
pub struct MonstersWithStates(Vec<MonsterStates>);

impl MonstersWithStates {
    pub fn new() -> Self {
        let monsters = (1..=0xB0)
            .map(|i| MonsterStates {
                id: i,
                unique_states: RapidHashSet::default(),
            })
            .collect();
        Self(monsters)
    }

    pub fn handle_new(&mut self, monster_id: u8, parts: &[MonsterPart]) {
        let Some(states) = self.get_mut(monster_id) else {
            error!("Couldn't get monster seen states from ID");
            return;
        };
        let converted = parts.iter().map(Part::from_game_data).collect();
        if states.unique_states.insert(converted) {
            info!("Added new state");
        }
    }

    pub fn get(&self, monster_id: u8) -> Option<&MonsterStates> {
        self.0.get(monster_id as usize - 1)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    fn get_mut(&mut self, monster_id: u8) -> Option<&mut MonsterStates> {
        self.0.get_mut(monster_id as usize - 1)
    }
}

#[derive(Serialize, Deserialize)]
pub struct MonsterStates {
    pub id: u8,
    pub unique_states: RapidHashSet<Vec<Part>>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Part {
    pub part_idx: u16,
    pub hzv_idx: u16,
    pub hzvs: HitzoneValues,
}

impl Part {
    pub fn from_game_data(part: &MonsterPart) -> Self {
        Self {
            part_idx: part.part_idx,
            hzv_idx: part.hzv_idx,
            hzvs: part.hzvs,
        }
    }

    pub fn table_display(&self, column: HzvColumn) -> String {
        match column {
            HzvColumn::Part => self.part_idx.to_string(),
            HzvColumn::Hzv => self.hzv_idx.to_string(),
            HzvColumn::Count => "???".to_string(),
            HzvColumn::Health => "???".to_string(),
            HzvColumn::Cut => self.hzvs.cut.to_string(),
            HzvColumn::Impact => self.hzvs.impact.to_string(),
            HzvColumn::Shot => self.hzvs.shot.to_string(),
            HzvColumn::Fire => self.hzvs.fire.to_string(),
            HzvColumn::Water => self.hzvs.water.to_string(),
            HzvColumn::Ice => self.hzvs.ice.to_string(),
            HzvColumn::Thunder => self.hzvs.thunder.to_string(),
            HzvColumn::Dragon => self.hzvs.dragon.to_string(),
            HzvColumn::Stun => self.hzvs.stun.to_string(),
        }
    }
}

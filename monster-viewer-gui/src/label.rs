use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Labels(Vec<MonsterLabels>);

#[derive(Serialize, Deserialize, Clone)]
pub struct MonsterLabels([Part; 9]);

#[derive(Serialize, Deserialize, Clone)]
pub struct Part {
    pub label: String,
    hzvs: Vec<HzvLabel>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HzvLabel {
    pub label: String,
    hzv_idx: usize,
}

impl Labels {
    pub fn monster_mut(&mut self, idx: usize) -> Option<&mut MonsterLabels> {
        self.0.get_mut(idx)
    }

    pub fn monster(&self, idx: usize) -> Option<&MonsterLabels> {
        self.0.get(idx)
    }
}

impl MonsterLabels {
    pub fn part_mut(&mut self, idx: usize) -> &mut Part {
        &mut self.0[idx]
    }

    pub fn part(&self, idx: usize) -> &Part {
        &self.0[idx]
    }
}

impl Part {
    pub fn new(part_idx: usize) -> Self {
        Self {
            label: part_idx.to_string(),
            hzvs: Vec::new(),
        }
    }

    pub fn get_or_insert_hzv(&mut self, hzv_idx: usize) -> &mut String {
        // Polonius...
        if let Some(i) = self.hzvs.iter().position(|hzv| hzv.hzv_idx == hzv_idx) {
            &mut self.hzvs[i].label
        } else {
            // There's an unstable feature to do this in one call but eh
            self.hzvs.push(HzvLabel {
                label: hzv_idx.to_string(),
                hzv_idx,
            });
            &mut self.hzvs.last_mut().unwrap().label
        }
    }

    pub fn get_hzv(&self, hzv_idx: usize) -> Option<&str> {
        self.hzvs
            .iter()
            .find(|hzv| hzv.hzv_idx == hzv_idx)
            .map(|hzv| hzv.label.as_ref())
    }
}

impl Default for Labels {
    fn default() -> Self {
        Self(vec![MonsterLabels::default(); 176])
    }
}
impl Default for MonsterLabels {
    fn default() -> Self {
        Self([
            Part::new(0),
            Part::new(1),
            Part::new(2),
            Part::new(3),
            Part::new(4),
            Part::new(5),
            Part::new(6),
            Part::new(7),
            Part::new(8),
        ])
    }
}

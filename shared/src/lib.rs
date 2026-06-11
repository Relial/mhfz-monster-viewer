use glam::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ViewerMessage {
    pub time_limit: u32,
    pub time_remaining: u32,
    pub data: MonsterData,
}

#[derive(Serialize, Deserialize)]
pub enum MonsterData {
    Monsters(Vec<Monster>),
    DamageInstance(DamageInstance),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Monster {
    pub struct_idx: u16,
    pub monster_id: u8,
    pub current_health: u16,
    pub max_health: u16,
    pub attack_multi: f32,
    pub defense_multi: f32,
    pub parts: Vec<MonsterPart>,
    pub status: AllStatus,
}

impl PartialEq for Monster {
    fn eq(&self, other: &Self) -> bool {
        self.struct_idx == other.struct_idx && self.monster_id == other.monster_id
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct MonsterPart {
    pub part_idx: u16,
    pub hzv_idx: u16,
    pub part_health: i16,
    pub hzvs: HitzoneValues,
    pub hitzone_count: usize,
}

impl PartialEq for MonsterPart {
    fn eq(&self, other: &Self) -> bool {
        self.part_idx == other.part_idx && self.hzv_idx == other.hzv_idx && self.hzvs == other.hzvs
    }
}

impl Eq for MonsterPart {}

impl std::hash::Hash for MonsterPart {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.part_idx.hash(state);
        self.hzv_idx.hash(state);
        self.hzvs.hash(state);
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct HitzoneValues {
    pub _unk: u8,
    pub cut: u8,
    pub impact: u8,
    pub shot: u8,
    pub fire: i8,
    pub water: i8,
    pub ice: i8,
    pub thunder: i8,
    pub dragon: i8,
    pub stun: u8,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct HitzoneInfo {
    pub unk0: u16,
    pub second_vector_indicator: u16,
    pub hzv_idx: u16,
    pub part_idx: u16,
    pub flags: [u8; 4],
    pub scale: f32,
    pub vec1: Vec3,
    pub vec2: Vec3,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct AllStatus {
    pub poison: Poison,
    pub paralysis: Status,
    pub sleep: Status,
    pub stun: Status,
    pub tranq: Status,
    pub blast: Status,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Poison {
    pub base: Status,
    pub duration: Option<u16>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Status {
    pub threshold: i16,
    pub current: i16,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct DamageInstance {
    pub monster_id: u8,
    pub struct_idx: u16,
    pub hitzone: HitzoneInfo,
}

use serde::Serialize;
use tracing::info;

use crate::{
    address::Addresses,
    hzv::{HitzoneInfo, HitzoneValues, get_hzv_info, get_hzvs},
};

#[derive(Clone, Copy)]
pub struct MonsterStruct(*const u8);

#[derive(Serialize)]
pub struct Monster {
    pub struct_idx: u16,
    pub monster_id: u8,
    pub current_health: u16,
    pub max_health: u16,
    pub attack_multi: f32,
    pub defense_multi: f32,
    pub parts: Vec<MonsterPart>,
}

impl Monster {
    pub fn from_struct(addresses: &Addresses, monster_struct: MonsterStruct) -> Self {
        let mut parts: Vec<MonsterPart> = Vec::new();
        let struct_idx = monster_struct.struct_idx();
        let monster_id = monster_struct.monster_id();
        let check1 = monster_struct.hitzone_check_1();
        let check2 = monster_struct.hitzone_check_2();
        let mut skip = false;
        for hzv_info in get_hzv_info(addresses, monster_struct) {
            if skip {
                skip = false;
                continue;
            }
            let info = unsafe { hzv_info.read() };
            match info.is_real(check1, check2) {
                crate::hzv::HitzoneValidity::Valid => {
                    if let Some(hzvs) = get_hzvs(addresses, hzv_info, monster_struct) {
                        if info.scale == 0. {
                            info!("0 scale entry: {:?}", info);
                        }
                        if let Some(existing_part) = parts.iter_mut().find(|part| {
                            part.part_idx == info.part_idx && part.hzv_idx == info.hzv_idx
                        }) {
                            existing_part.hitzone_count +=
                                1 + info.second_vector_indicator as usize;
                        } else {
                            let part_health = monster_struct.part_health(info.part_idx as usize);
                            parts.push(MonsterPart {
                                part_idx: info.part_idx,
                                hzv_idx: info.hzv_idx,
                                part_health,
                                hzvs,
                                hitzone_count: 1 + info.second_vector_indicator as usize,
                            });
                        }
                    }
                }
                crate::hzv::HitzoneValidity::Invalid => continue,
                crate::hzv::HitzoneValidity::InvalidSkipNext => {
                    skip = true;
                    continue;
                }
            }
        }

        let current_health = monster_struct.current_health(addresses);
        let max_health = monster_struct.max_health(addresses);
        let attack_multi = monster_struct.attack_multi();
        let defense_multi = monster_struct.defense_multi();
        parts.sort_by_key(|part| (part.part_idx, part.hzv_idx));

        Self {
            struct_idx,
            monster_id,
            current_health,
            max_health,
            attack_multi,
            defense_multi,
            parts,
        }
    }
}

#[derive(Serialize, Clone, Copy)]
pub struct MonsterPart {
    pub part_idx: u16,
    pub hzv_idx: u16,
    pub part_health: i16,
    pub hzvs: HitzoneValues,
    pub hitzone_count: usize,
}

impl MonsterStruct {
    pub fn new(ptr: *const u8) -> Self {
        Self(ptr)
    }

    pub fn inner(&self) -> *const u8 {
        self.0
    }

    pub fn monster_id(&self) -> u8 {
        unsafe { self.0.wrapping_byte_add(0x3).read() }
    }

    pub fn struct_idx(&self) -> u16 {
        unsafe { (self.0.wrapping_byte_add(0xC) as *const u16).read() }
    }

    pub fn part_health(&self, part_idx: usize) -> i16 {
        unsafe { (self.0.wrapping_byte_add(0x348 + part_idx * 8) as *const i16).read() }
    }

    pub fn attack_multi(&self) -> f32 {
        unsafe { (self.0.wrapping_byte_add(0x898) as *const f32).read() }
    }

    pub fn defense_multi(&self) -> f32 {
        unsafe { (self.0.wrapping_byte_add(0x89C) as *const f32).read() }
    }

    pub fn current_health(&self, addresses: &Addresses) -> u16 {
        unsafe {
            let health = (self.0.wrapping_byte_add(0x624) as *const u16).read();
            let idx = self.struct_idx();
            let key = (addresses.encryption1 as *const u16).read();
            let p = (addresses.encryption2 - idx as usize * 2) as *const u16;
            !(p.read()) ^ health.rotate_right(5) ^ key ^ addresses.encryption3
        }
    }

    pub fn max_health(&self, addresses: &Addresses) -> u16 {
        unsafe {
            let health = (self.0.wrapping_byte_add(0x6BC) as *const u16).read();
            let idx = self.struct_idx();
            let key = (addresses.encryption1 as *const u16).read();
            let p = (addresses.encryption2 - idx as usize * 2) as *const u16;
            !(p.read()) ^ health.rotate_right(3) ^ key ^ addresses.encryption3
        }
    }

    pub fn hitzone_check_1(&self) -> u8 {
        unsafe { self.0.wrapping_byte_add(0xCB0).read() }
    }

    pub fn hitzone_check_2(&self) -> u8 {
        unsafe { self.0.wrapping_byte_add(0xAB3).read() }
    }
}

#[derive(Serialize, Clone, Copy)]
pub struct DamageInstance {
    pub monster_id: u8,
    pub struct_idx: u16,
    pub hitzone: HitzoneInfo,
}

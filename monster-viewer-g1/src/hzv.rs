use std::arch::asm;

use glam::Vec3;
use serde::Serialize;

use crate::{address::Addresses, monster::MonsterStruct};

#[repr(C)]
#[derive(Clone, Copy, Debug, Serialize)]
pub struct HitzoneInfo {
    unk0: u16,
    pub second_vector_indicator: u16,
    pub hzv_idx: u16,
    pub part_idx: u16,
    flags: [u8; 4],
    pub scale: f32,
    pub vec1: Vec3,
    pub vec2: Vec3,
}

impl HitzoneInfo {
    pub fn is_real(&self, check1: u8, _check2: u8, check3: u16) -> HitzoneValidity {
        if self.unk0 == 0x7D {
            if self.second_vector_indicator & check3 == 0 {
                HitzoneValidity::Invalid
            } else {
                HitzoneValidity::InvalidSkipNextN(self.hzv_idx)
            }
        } else if (check1 != 0 || self.flags[2] & 0x12 == 0)
            && (self.flags[2] & 0x8 == 0 || (check1 != 0 && self.flags[3] & 1 == 0))
            && self.flags[1] & 0x4 == 0
            && (check1 != 0 || self.flags[0] & 0x4 == 0)
            && self.flags[1] & 0x12 == 0
        // || check2 == 0 relevant if some variable is a specific value, seems to be tied to the attack you use but idk
        {
            HitzoneValidity::Valid
        } else {
            HitzoneValidity::Invalid
        }
    }
}

pub enum HitzoneValidity {
    Valid,
    Invalid,
    InvalidSkipNextN(u16),
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
pub struct HitzoneValues {
    _unk: u8,
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

pub fn get_hzvs(
    addresses: &Addresses,
    hzv_info: *const HitzoneInfo,
    monster: MonsterStruct,
) -> Option<HitzoneValues> {
    let get_hzvs_addr = addresses.get_hzvs_func;
    unsafe {
        let hzv_index = hzv_info.read().hzv_idx;
        let values_addr: u32;
        asm!(
            "call {get_hzvs}",
            in("eax") monster.inner(),
            in("edi") hzv_index,
            get_hzvs = in(reg) get_hzvs_addr,
            lateout("eax") values_addr,
            out("ecx") _,
            out("edx") _,
        );
        let ptr = values_addr as *const HitzoneValues;
        if !ptr.is_null() {
            Some(ptr.read())
        } else {
            None
        }
    }
}

pub fn get_hzvs_taikun(
    addresses: &Addresses,
    hzv_info: *const HitzoneInfo,
    monster: MonsterStruct,
) -> Option<HitzoneValues> {
    let get_hzvs_addr = addresses.get_hzvs_taikun_func;
    unsafe {
        let values_addr: u32;
        asm!(
            "call {get_hzvs}",
            in("eax") monster.inner(),
            in("ecx") hzv_info,
            get_hzvs = in(reg) get_hzvs_addr,
            lateout("eax") values_addr,
            out("edx") _,
        );
        let ptr = values_addr as *const HitzoneValues;
        if !ptr.is_null() {
            Some(ptr.read())
        } else {
            None
        }
    }
}

pub fn get_hzv_info(addresses: &Addresses, monster: MonsterStruct) -> Vec<*const HitzoneInfo> {
    let get_hzv_info_addr = addresses.get_hzv_info_func;
    let mut info = Vec::new();
    unsafe {
        let info_addr: u32;
        asm!(
            "call {get_info}",
            in("eax") monster.inner(),
            get_info = in(reg) get_hzv_info_addr,
            lateout("eax") info_addr,
            out("ecx") _,
            out("edx") _,
        );
        let ptr = info_addr as *const *const HitzoneInfo;
        if ptr.is_null() {
            return info;
        }
        let ptr = ptr.read();
        if ptr.is_null() {
            return info;
        }
        let mut i = 0;
        loop {
            let ptr = ptr.wrapping_byte_add(0x28 * i);
            i += 1;
            let check = (ptr as *const u16).read();
            if check == 0xFFFF {
                break;
            }
            info.push(ptr);
        }
    }
    info
}

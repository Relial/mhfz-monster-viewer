use std::arch::asm;

use shared::{HitzoneInfo, HitzoneValues};

use crate::{address::Addresses, monster::MonsterStruct};

pub trait HitzoneInfoImpl {
    fn is_real(&self, check1: u8, check2: u8, check3: u16) -> HitzoneValidity;
}

impl HitzoneInfoImpl for HitzoneInfo {
    fn is_real(&self, check1: u8, _check2: u8, check3: u16) -> HitzoneValidity {
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

pub fn get_hzvs(
    addresses: &Addresses,
    hzv_info: *const HitzoneInfo,
    monster: MonsterStruct,
) -> Option<HitzoneValues> {
    let get_hzvs_addr = addresses.get_hzvs_func;
    unsafe {
        let values_addr: u32;
        asm!(
            "push {monster}",
            "call {get_hzvs}",
            "add esp, 0x4",
            monster = in(reg) monster.inner(),
            in("eax") hzv_info,
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

pub fn get_hzv_info(addresses: &Addresses, monster: MonsterStruct) -> Vec<*const HitzoneInfo> {
    let get_hzv_info_addr = addresses.get_hzv_info_func;
    let mut info = Vec::new();
    unsafe {
        let info_addr: u32;
        asm!(
            "push 0x0",
            "call {get_info}",
            "add esp, 0x4",
            in("eax") monster.inner(),
            get_info = in(reg) get_hzv_info_addr,
            lateout("eax") info_addr,
            out("ecx") _,
            out("edx") _,
        );
        let ptr = info_addr as *const HitzoneInfo;
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

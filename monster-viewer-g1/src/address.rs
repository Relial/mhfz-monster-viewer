use core::slice;
use std::thread::sleep;
use std::time::Duration;

use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::core::{PCSTR, s};

use crate::monster::MonsterStruct;

#[derive(Clone, Copy)]
pub struct Addresses {
    pub dll: usize,
    pub quest_func: usize,
    pub damage_calc_get_hzv: usize,
    pub get_hzv_info_func: usize,
    pub get_hzvs_func: usize,
    pub get_hzvs_taikun_func: usize,
    pub encryption1: usize,
    pub encryption2: usize,
    pub encryption3: u16,
    player_structs: usize,
    monster_structs: usize,
    player_info: usize,
    quest_info: usize,
}

pub fn find_main_dll() -> Addresses {
    loop {
        if let Ok(handle) = unsafe { GetModuleHandleA(PCSTR(s!("mhfo.dll").as_ptr())) } {
            return Addresses::new_lge(handle.0.addr());
        }
        sleep(Duration::from_millis(100));
    }
}

impl Addresses {
    fn new_lge(dll: usize) -> Self {
        Self {
            dll,
            quest_func: dll + 0x308127,
            damage_calc_get_hzv: dll + 0x31C2F0,
            get_hzv_info_func: dll + 0x2D0EC0,
            get_hzvs_func: dll + 0x2DF750,
            get_hzvs_taikun_func: dll + 0x591530,
            encryption1: dll + 0xD871E4,
            encryption2: dll + 0x5B38DEE,
            encryption3: 0xDB70,
            player_structs: dll + 0x4C2FE90,
            monster_structs: dll + 0x5B0E2AC,
            player_info: dll + 0x56A9FA8,
            quest_info: dll + 0x56a9fac,
        }
    }

    pub fn monster_structs(&self) -> Option<Vec<MonsterStruct>> {
        let player_info_ptr = unsafe { (self.player_info as *const *const u8).read() };
        let structs_ptr = unsafe { (self.monster_structs as *const *const u8).read() };
        if structs_ptr.is_null() || player_info_ptr.is_null() {
            return None;
        }
        let player_info = unsafe { player_info_ptr.read() };
        if player_info == 2 || player_info == 3 {
            Some(
                (0..40)
                    .flat_map(|i| {
                        let ptr = structs_ptr.wrapping_byte_add(0xD80 * i);
                        if monster_exists(ptr) {
                            Some(MonsterStruct::new(ptr))
                        } else {
                            None
                        }
                    })
                    .collect(),
            )
        } else {
            None
        }
    }

    fn own_player_idx(&self) -> Option<usize> {
        unsafe {
            let ptr = (self.player_info as *const *const u8).read();
            if !ptr.is_null() {
                let idx = ptr.wrapping_byte_add(0xAF8).read() as usize;
                Some(idx)
            } else {
                None
            }
        }
    }

    pub fn own_player_addr(&self) -> Option<usize> {
        let idx = self.own_player_idx()?;
        Some(self.player_structs + (idx * 0xF40))
    }

    pub fn quest_info(&self) -> Option<QuestInfo> {
        QuestInfo::from_addr(self.quest_info)
    }
}

fn monster_exists(ptr: *const u8) -> bool {
    let flags = unsafe { slice::from_raw_parts(ptr, 2) };
    flags[0] == 1 && flags[1] == 1
}

pub struct QuestInfo(*const u8);

impl QuestInfo {
    fn from_addr(addr: usize) -> Option<Self> {
        let ptr = unsafe { (addr as *const *const u8).read() };
        if ptr.is_null() { None } else { Some(Self(ptr)) }
    }

    pub fn time_limit(&self) -> Option<u32> {
        unsafe {
            let base_quest_ptr = ((self.0.wrapping_byte_add(0x7C)) as *const *const u32).read();
            if base_quest_ptr.is_null() {
                return None;
            }
            Some(base_quest_ptr.wrapping_byte_add(0x20).read())
        }
    }

    pub fn time_remaining(&self) -> u32 {
        unsafe { (self.0.wrapping_byte_add(0x10) as *const u32).read() }
    }
}

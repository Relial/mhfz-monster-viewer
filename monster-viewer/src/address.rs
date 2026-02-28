use core::slice;
use std::thread::sleep;
use std::time::Duration;

use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::core::{PCSTR, s};

use crate::monster::MonsterStruct;

#[derive(Clone, Copy)]
pub enum MHFOInfo {
    LowGrade(Addresses),
    HighGrade(Addresses),
}

#[derive(Clone, Copy)]
pub struct Addresses {
    pub dll: usize,
    pub quest_func: usize,
    pub damage_calc_get_hzv: usize,
    pub get_hzv_info_func: usize,
    pub get_hzvs_func: usize,
    pub encryption1: usize,
    pub encryption2: usize,
    pub encryption3: u16,
    pub keyboard_values: usize,
    player_structs: usize,
    monster_structs: usize,
    player_info: usize,
}

impl MHFOInfo {
    pub fn find_main_dll() -> Self {
        loop {
            if let Ok(handle) = unsafe { GetModuleHandleA(PCSTR(s!("mhfo.dll").as_ptr())) } {
                return Self::LowGrade(Addresses::new_lge(handle.0.addr()));
            } else if let Ok(handle) =
                unsafe { GetModuleHandleA(PCSTR(s!("mhfo-hd.dll").as_ptr())) }
            {
                return Self::HighGrade(Addresses::new_hge(handle.0.addr()));
            }
            sleep(Duration::from_millis(100));
        }
    }

    pub fn addresses(&self) -> Addresses {
        match self {
            MHFOInfo::LowGrade(addresses) => *addresses,
            MHFOInfo::HighGrade(addresses) => *addresses,
        }
    }
}

impl Addresses {
    fn new_lge(dll: usize) -> Self {
        Self {
            dll,
            quest_func: dll + 0x880360,
            damage_calc_get_hzv: dll + 0x8A0743,
            get_hzv_info_func: dll + 0x82C1D0,
            get_hzvs_func: dll + 0x8407D0,
            encryption1: dll + 0x1A52B5C,
            encryption2: dll + 0x617CEEE,
            encryption3: 0xB7A0,
            keyboard_values: dll + 0x4F83824,
            player_structs: dll + 0x5033B90,
            monster_structs: dll + 0x614058C,
            player_info: dll + 0x5BC830C,
        }
    }

    fn new_hge(dll: usize) -> Self {
        Self {
            dll,
            quest_func: dll + 0x89BE10,
            damage_calc_get_hzv: dll + 0x8BC2A3,
            get_hzv_info_func: dll + 0x846CA0,
            get_hzvs_func: dll + 0x85B2D0,
            encryption1: dll + 0x1A422C4,
            encryption2: dll + 0xEDB768E,
            encryption3: 0x5EC0,
            keyboard_values: dll + 0xDBBB3D4,
            player_structs: dll + 0xDC6B750,
            monster_structs: dll + 0xED7AD2C,
            player_info: dll + 0xE7FFF3C,
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
                        let ptr = structs_ptr.wrapping_byte_add(0xEF0 * i);
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
                let idx = ptr.wrapping_byte_add(0x23F8).read() as usize;
                Some(idx)
            } else {
                None
            }
        }
    }

    pub fn own_player_addr(&self) -> Option<usize> {
        let idx = self.own_player_idx()?;
        Some(self.player_structs + (idx * 0x1050))
    }
}

fn monster_exists(ptr: *const u8) -> bool {
    let flags = unsafe { slice::from_raw_parts(ptr, 2) };
    flags[0] == 1 && flags[1] == 1
}

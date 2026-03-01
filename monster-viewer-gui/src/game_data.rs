use std::time::Instant;

use glam::Vec3;
use serde::Deserialize;

use crate::ui::HzvColumn;

#[derive(Deserialize, Clone)]
pub struct Monster {
    pub struct_idx: u16,
    pub monster_id: u8,
    pub current_health: u16,
    pub max_health: u16,
    pub attack_multi: f32,
    pub defense_multi: f32,
    pub parts: Vec<MonsterPart>,
}

#[derive(Clone, Copy, Deserialize)]
pub struct MonsterPart {
    pub part_idx: u16,
    pub hzv_idx: u16,
    pub part_health: i16,
    pub hzvs: HitzoneValues,
    pub hitzone_count: usize,
}

impl MonsterPart {
    pub fn table_display(&self, column: HzvColumn) -> String {
        match column {
            HzvColumn::Part => self.part_idx.to_string(),
            HzvColumn::Hzv => self.hzv_idx.to_string(),
            HzvColumn::Count => self.hitzone_count.to_string(),
            HzvColumn::Health => self.part_health.to_string(),
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

    pub fn get_changes(&self, other: &Self) -> Option<[bool; 13]> {
        if self.part_idx != other.part_idx {
            return Some([true; 13]);
        }
        let mut changes = [false; 13];
        if self.hzv_idx != other.hzv_idx {
            changes[1] = true;
        }
        if self.hitzone_count != other.hitzone_count {
            changes[2] = true;
        }
        if self.part_health != other.part_health {
            changes[3] = true;
        }
        if self.hzvs.cut != other.hzvs.cut {
            changes[4] = true;
        }
        if self.hzvs.impact != other.hzvs.impact {
            changes[5] = true;
        }
        if self.hzvs.shot != other.hzvs.shot {
            changes[6] = true;
        }
        if self.hzvs.fire != other.hzvs.fire {
            changes[7] = true;
        }
        if self.hzvs.water != other.hzvs.water {
            changes[8] = true;
        }
        if self.hzvs.ice != other.hzvs.ice {
            changes[9] = true;
        }
        if self.hzvs.thunder != other.hzvs.thunder {
            changes[10] = true;
        }
        if self.hzvs.dragon != other.hzvs.dragon {
            changes[11] = true;
        }
        if self.hzvs.stun != other.hzvs.stun {
            changes[12] = true;
        }
        if changes.iter().all(|c| !c) {
            None
        } else {
            Some(changes)
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub struct HitzoneValues {
    _unk: u8,
    pub cut: i8,
    pub impact: i8,
    pub shot: i8,
    pub fire: i8,
    pub water: i8,
    pub ice: i8,
    pub thunder: i8,
    pub dragon: i8,
    pub stun: u8,
}

impl PartialEq for Monster {
    fn eq(&self, other: &Self) -> bool {
        self.struct_idx == other.struct_idx && self.monster_id == other.monster_id
    }
}

impl PartialEq for MonsterPart {
    fn eq(&self, other: &Self) -> bool {
        self.part_idx == other.part_idx
            && self.hzv_idx == other.hzv_idx
            && self.hzvs == other.hzvs
            && self.hitzone_count == other.hitzone_count
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Deserialize)]
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

#[derive(Deserialize, Clone, Copy, Debug)]
pub struct DamageInstance {
    pub monster_id: u8,
    pub struct_idx: u16,
    pub hitzone: HitzoneInfo,
}

pub fn monster_name(id: u8) -> &'static str {
    match id {
        0x01 => "Rathian",
        0x02 => "Fatalis",
        0x03 => "Kelbi",
        0x04 => "Mosswine",
        0x05 => "Bullfango",
        0x06 => "Yian Kut-Ku",
        0x07 => "Lao-Shan Lung",
        0x08 => "Cephadrome",
        0x09 => "Felyne",
        0x0A => "Veggie Elder",
        0x0B => "Rathalos",
        0x0C => "Aptonoth",
        0x0D => "Genprey",
        0x0E => "Diablos",
        0x0F => "Khezu",
        0x10 => "Velociprey",
        0x11 => "Gravios",
        0x12 => "ID 18",
        0x13 => "Vespoid",
        0x14 => "Gypceros",
        0x15 => "Plesioth",
        0x16 => "Basarios",
        0x17 => "Melynx",
        0x18 => "Hornetaur",
        0x19 => "Apceros",
        0x1A => "Monoblos",
        0x1B => "Velocidrome",
        0x1C => "Gendrome",
        0x1D => "Rocks",
        0x1E => "Ioprey",
        0x1F => "Iodrome",
        0x20 => "Poogie",
        0x21 => "Kirin",
        0x22 => "Cephalos",
        0x23 => "Giaprey/Giadrome",
        0x24 => "Crimson Fatalis",
        0x25 => "Pink Rathian",
        0x26 => "Blue Yian Kut-Ku",
        0x27 => "Purple Gypceros",
        0x28 => "Yian Garuga",
        0x29 => "Silver Rathalos",
        0x2A => "Gold Rathian",
        0x2B => "Black Diablos",
        0x2C => "White Monoblos",
        0x2D => "Red Khezu",
        0x2E => "Green Plesioth",
        0x2F => "Black Gravios",
        0x30 => "Daimyo Hermitaur",
        0x31 => "Azure Rathalos",
        0x32 => "Ashen Lao-Shan Lung",
        0x33 => "Blangonga",
        0x34 => "Congalala",
        0x35 => "Rajang",
        0x36 => "Kushala Daora",
        0x37 => "Shen Gaoren",
        0x38 => "Great Thunderbug",
        0x39 => "Shakalaka",
        0x3A => "Yama Tsukami",
        0x3B => "Chameleos",
        0x3C => "Rusted Kushala Daora",
        0x3D => "Blango",
        0x3E => "Conga",
        0x3F => "Remobra",
        0x40 => "Lunastra",
        0x41 => "Teostra",
        0x42 => "Hermitaur",
        0x43 => "Shogun Ceanataur",
        0x44 => "Bulldrome",
        0x45 => "Anteka",
        0x46 => "Popo",
        0x47 => "White Fatalis",
        0x48 => "Yama Tsukami",
        0x49 => "Ceanataur",
        0x4A => "Hynocatrice",
        0x4B => "Lavasioth",
        0x4C => "Tigrex",
        0x4D => "Akantor",
        0x4E => "Bright Hypnocatrice",
        0x4F => "Red Lavasioth",
        0x50 => "Espinas",
        0x51 => "Orange Espinas",
        0x52 => "White Hypnocatrice",
        0x53 => "Akura Vashimu",
        0x54 => "Akura Jebia",
        0x55 => "Berukyurosu",
        0x56 => "Cactus",
        0x57 => "Gorge Objects",
        0x58 => "Gorge Rocks",
        0x59 => "Pariapuria",
        0x5A => "White Espinas",
        0x5B => "Kamu Orugaron",
        0x5C => "Nono Orugaron",
        0x5D => "Raviente",
        0x5E => "Dyuragaua",
        0x5F => "Doragyurosu",
        0x60 => "Gurenzeburu",
        0x61 => "Burukku",
        0x62 => "Alpelo",
        0x63 => "Rukodiora",
        0x64 => "Unknown",
        0x65 => "Gogomoa",
        0x66 => "Kokomoa",
        0x67 => "Taikun Zamuza",
        0x68 => "Abiorugu",
        0x69 => "Kuarusepusu",
        0x6A => "Odibatorasu",
        0x6B => "Disufiroa",
        0x6C => "Rebidiora",
        0x6D => "Anorupatisu",
        0x6E => "Hyujikiki",
        0x6F => "Midogaron",
        0x70 => "Giaorugu",
        0x71 => "Mi Ru",
        0x72 => "Farunokku",
        0x73 => "Pokaradon",
        0x74 => "Shantien",
        0x75 => "Pokara",
        0x76 => "Dummy",
        0x77 => "Goruganosu",
        0x78 => "Aruganosu",
        0x79 => "Baruragaru",
        0x7A => "Zerureusu",
        0x7B => "Gougarf",
        0x7C => "Uruki",
        0x7D => "Forokururu",
        0x7E => "Meraginasu",
        0x7F => "Diorekkusu",
        0x80 => "Garuba Daora",
        0x81 => "Inagami",
        0x82 => "Varusaburosu",
        0x83 => "Poborubarumu",
        0x84 => "Duremudira",
        0x85 => "ID 133",
        0x86 => "Felyne",
        0x87 => "Blue NPC",
        0x88 => "ID 136",
        0x89 => "Cactus",
        0x8A => "Veggie Elder",
        0x8B => "Gureadomosu",
        0x8C => "Harudomerugu",
        0x8D => "Toridcless",
        0x8E => "Gasurabazura",
        0x8F => "Kusubami",
        0x90 => "Yama Kurai",
        0x91 => "Duremudira",
        0x92 => "Zinogre",
        0x93 => "Deviljho",
        0x94 => "Brachydios",
        0x95 => "Berserk Raviente",
        0x96 => "Toa Tesukatora",
        0x97 => "Barioth",
        0x98 => "Uragaan",
        0x99 => "Stygian Zinogre",
        0x9A => "Guanzorumu",
        0x9B => "Deviljho",
        0x9C => "ID 156",
        0x9D => "Egyurasu",
        0x9E => "Voljang",
        0x9F => "Nargacuga",
        0xA0 => "Keoaruboru",
        0xA1 => "Zenaserisu",
        0xA2 => "Gore Magala",
        0xA3 => "Musou Nargacuga",
        0xA4 => "Shagaru Magala",
        0xA5 => "Amatsu",
        0xA6 => "Elzelion",
        0xA7 => "Musou Duremudira",
        0xA8 => "Rocks",
        0xA9 => "Seregios",
        0xAA => "Bogabadorumu",
        0xAB => "Blue Barrel",
        0xAC => "Musou Bogabadorumu",
        0xAD => "Costumed Uruki",
        0xAE => "Musou Zerureusu",
        0xAF => "PSO2 Rappy",
        0xB0 => "King Shakalaka",
        _ => "Unknown ID",
    }
}

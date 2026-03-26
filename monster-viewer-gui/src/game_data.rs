use glam::Vec3;
use serde::{Deserialize, Serialize};

use crate::ui::HzvColumn;

pub const MONSTER_COUNT: usize = 176;
pub const MONSTER_NAMES: [&str; MONSTER_COUNT] = [
    "Rathian",
    "Fatalis",
    "Kelbi",
    "Mosswine",
    "Bullfango",
    "Yian Kut-Ku",
    "Lao-Shan Lung",
    "Cephadrome",
    "Felyne",
    "Veggie Elder",
    "Rathalos",
    "Aptonoth",
    "Genprey",
    "Diablos",
    "Khezu",
    "Velociprey",
    "Gravios",
    "ID 18",
    "Vespoid",
    "Gypceros",
    "Plesioth",
    "Basarios",
    "Melynx",
    "Hornetaur",
    "Apceros",
    "Monoblos",
    "Velocidrome",
    "Gendrome",
    "Rocks",
    "Ioprey",
    "Iodrome",
    "Poogie",
    "Kirin",
    "Cephalos",
    "Giaprey",
    "Crimson Fatalis",
    "Pink Rathian",
    "Blue Yian Kut-Ku",
    "Purple Gypceros",
    "Yian Garuga",
    "Silver Rathalos",
    "Gold Rathian",
    "Black Diablos",
    "White Monoblos",
    "Red Khezu",
    "Green Plesioth",
    "Black Gravios",
    "Daimyo Hermitaur",
    "Azure Rathalos",
    "Ashen Lao-Shan Lung",
    "Blangonga",
    "Congalala",
    "Rajang",
    "Kushala Daora",
    "Shen Gaoren",
    "Great Thunderbug",
    "Shakalaka",
    "Yama Tsukami",
    "Chameleos",
    "Rusted Kushala Daora",
    "Blango",
    "Conga",
    "Remobra",
    "Lunastra",
    "Teostra",
    "Hermitaur",
    "Shogun Ceanataur",
    "Bulldrome",
    "Anteka",
    "Popo",
    "Old Fatalis",
    "Yama Tsukami",
    "Ceanataur",
    "Hypnocatrice",
    "Lavasioth",
    "Tigrex",
    "Akantor",
    "Vivid Hypnocatrice",
    "Red Lavasioth",
    "Espinas",
    "Flaming Espinas",
    "Pale Hypnocatrice",
    "Aqra Vashim",
    "Aqra Jebia",
    "Belkuros",
    "Cactus",
    "Canyon Objects",
    "Canyon Rocks",
    "Pariapuria",
    "Pearl Espinas",
    "Kamu Orgaron",
    "Nono Orgaron",
    "Laviente",
    "Duragaua",
    "Draguros",
    "Grenzebul",
    "Bulluk",
    "Alpelo",
    "Rukodiora",
    "Unknown",
    "Gogomoa",
    "Kokomoa",
    "Taikun Zamuza",
    "Abiorg",
    "Quarzeps",
    "Odibataurus",
    "Vorsphyroa",
    "Levidiora",
    "Anorpatis",
    "Hyujikiki",
    "Midogaron",
    "Giaorg",
    "Mi Ru",
    "Falnocatrice",
    "Pokaradon",
    "Shantien",
    "Pokara",
    "Dummy",
    "Aurisioth",
    "Argasioth",
    "Barlagual",
    "Zerulos",
    "Gougarf",
    "Uruki",
    "Foroclore",
    "Melaginas",
    "Diorex",
    "Garbha Daora",
    "Inagami",
    "Varsablos",
    "Pobolbarm",
    "Dolemdira",
    "ID 133",
    "Felyne",
    "Blue NPC",
    "ID 136",
    "Cactus",
    "Veggie Elder",
    "Greadios",
    "Haldmerg",
    "Toridcless",
    "Gasrabazra",
    "Kusubami",
    "Yama Kurai",
    "Dolemdira",
    "Zinogre",
    "Deviljho",
    "Brachydios",
    "Berserk Laviente",
    "Toastra",
    "Barioth",
    "Uragaan",
    "Stygian Zinogre",
    "Guanzorm",
    "Feral Deviljho",
    "ID 156",
    "Equibra",
    "Voljang",
    "Nargacuga",
    "Keo'arbor",
    "Zenaseris",
    "Gore Magala",
    "Blinking Nargacuga",
    "Shagaru Magala",
    "Amatsu",
    "Elzelion",
    "Prideful Dolemdira",
    "Rocks",
    "Seregios",
    "Bogabador",
    "Blue Barrel",
    "Razing Bogabador",
    "Costumed Uruki",
    "Radiant Zerulos",
    "PSO2 Rappy",
    "King Shakalaka",
];
pub const MONSTERS_ALPHABETICAL: [(u8, &str); MONSTER_COUNT] = [
    (104, "Abiorg"),
    (77, "Akantor"),
    (98, "Alpelo"),
    (165, "Amatsu"),
    (109, "Anorpatis"),
    (69, "Anteka"),
    (25, "Apceros"),
    (12, "Aptonoth"),
    (84, "Aqra Jebia"),
    (83, "Aqra Vashim"),
    (120, "Argasioth"),
    (50, "Ashen Lao-Shan Lung"),
    (119, "Aurisioth"),
    (49, "Azure Rathalos"),
    (151, "Barioth"),
    (121, "Barlagual"),
    (22, "Basarios"),
    (85, "Belkuros"),
    (149, "Berserk Laviente"),
    (43, "Black Diablos"),
    (47, "Black Gravios"),
    (61, "Blango"),
    (51, "Blangonga"),
    (163, "Blinking Nargacuga"),
    (171, "Blue Barrel"),
    (135, "Blue NPC"),
    (38, "Blue Yian Kut-Ku"),
    (170, "Bogabador"),
    (148, "Brachydios"),
    (68, "Bulldrome"),
    (5, "Bullfango"),
    (97, "Bulluk"),
    (86, "Cactus"),
    (137, "Cactus"),
    (87, "Canyon Objects"),
    (88, "Canyon Rocks"),
    (73, "Ceanataur"),
    (8, "Cephadrome"),
    (34, "Cephalos"),
    (59, "Chameleos"),
    (62, "Conga"),
    (52, "Congalala"),
    (173, "Costumed Uruki"),
    (36, "Crimson Fatalis"),
    (48, "Daimyo Hermitaur"),
    (147, "Deviljho"),
    (14, "Diablos"),
    (127, "Diorex"),
    (132, "Dolemdira"),
    (145, "Dolemdira"),
    (95, "Draguros"),
    (118, "Dummy"),
    (94, "Duragaua"),
    (166, "Elzelion"),
    (157, "Equibra"),
    (80, "Espinas"),
    (114, "Falnocatrice"),
    (2, "Fatalis"),
    (9, "Felyne"),
    (134, "Felyne"),
    (155, "Feral Deviljho"),
    (81, "Flaming Espinas"),
    (125, "Foroclore"),
    (128, "Garbha Daora"),
    (142, "Gasrabazra"),
    (28, "Gendrome"),
    (13, "Genprey"),
    (112, "Giaorg"),
    (35, "Giaprey"),
    (101, "Gogomoa"),
    (42, "Gold Rathian"),
    (162, "Gore Magala"),
    (123, "Gougarf"),
    (17, "Gravios"),
    (139, "Greadios"),
    (56, "Great Thunderbug"),
    (46, "Green Plesioth"),
    (96, "Grenzebul"),
    (154, "Guanzorm"),
    (20, "Gypceros"),
    (140, "Haldmerg"),
    (66, "Hermitaur"),
    (24, "Hornetaur"),
    (74, "Hypnocatrice"),
    (110, "Hyujikiki"),
    (18, "ID 18"),
    (133, "ID 133"),
    (136, "ID 136"),
    (156, "ID 156"),
    (129, "Inagami"),
    (31, "Iodrome"),
    (30, "Ioprey"),
    (91, "Kamu Orgaron"),
    (3, "Kelbi"),
    (160, "Keo'arbor"),
    (15, "Khezu"),
    (176, "King Shakalaka"),
    (33, "Kirin"),
    (102, "Kokomoa"),
    (54, "Kushala Daora"),
    (143, "Kusubami"),
    (7, "Lao-Shan Lung"),
    (75, "Lavasioth"),
    (93, "Laviente"),
    (108, "Levidiora"),
    (64, "Lunastra"),
    (126, "Melaginas"),
    (23, "Melynx"),
    (113, "Mi Ru"),
    (111, "Midogaron"),
    (26, "Monoblos"),
    (4, "Mosswine"),
    (159, "Nargacuga"),
    (92, "Nono Orgaron"),
    (106, "Odibataurus"),
    (71, "Old Fatalis"),
    (175, "PSO2 Rappy"),
    (82, "Pale Hypnocatrice"),
    (89, "Pariapuria"),
    (90, "Pearl Espinas"),
    (37, "Pink Rathian"),
    (21, "Plesioth"),
    (131, "Pobolbarm"),
    (117, "Pokara"),
    (115, "Pokaradon"),
    (32, "Poogie"),
    (70, "Popo"),
    (167, "Prideful Dolemdira"),
    (39, "Purple Gypceros"),
    (105, "Quarzeps"),
    (174, "Radiant Zerulos"),
    (53, "Rajang"),
    (11, "Rathalos"),
    (1, "Rathian"),
    (172, "Razing Bogabador"),
    (45, "Red Khezu"),
    (79, "Red Lavasioth"),
    (63, "Remobra"),
    (29, "Rocks"),
    (168, "Rocks"),
    (99, "Rukodiora"),
    (60, "Rusted Kushala Daora"),
    (169, "Seregios"),
    (164, "Shagaru Magala"),
    (57, "Shakalaka"),
    (116, "Shantien"),
    (55, "Shen Gaoren"),
    (67, "Shogun Ceanataur"),
    (41, "Silver Rathalos"),
    (153, "Stygian Zinogre"),
    (103, "Taikun Zamuza"),
    (65, "Teostra"),
    (76, "Tigrex"),
    (150, "Toastra"),
    (141, "Toridcless"),
    (100, "Unknown"),
    (152, "Uragaan"),
    (124, "Uruki"),
    (130, "Varsablos"),
    (10, "Veggie Elder"),
    (138, "Veggie Elder"),
    (27, "Velocidrome"),
    (16, "Velociprey"),
    (19, "Vespoid"),
    (78, "Vivid Hypnocatrice"),
    (158, "Voljang"),
    (107, "Vorsphyroa"),
    (44, "White Monoblos"),
    (144, "Yama Kurai"),
    (58, "Yama Tsukami"),
    (72, "Yama Tsukami"),
    (40, "Yian Garuga"),
    (6, "Yian Kut-Ku"),
    (161, "Zenaseris"),
    (122, "Zerulos"),
    (146, "Zinogre"),
];

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
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
    if (1..=MONSTER_COUNT).contains(&(id as usize)) {
        MONSTER_NAMES[(id - 1) as usize]
    } else {
        "Unknown ID"
    }
}

impl PartialEq for Monster {
    fn eq(&self, other: &Self) -> bool {
        self.struct_idx == other.struct_idx && self.monster_id == other.monster_id
    }
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

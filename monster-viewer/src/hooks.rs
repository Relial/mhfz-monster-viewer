use std::sync::mpsc::Sender;

use anyhow::{Result, anyhow};
use ilhook::x86::CallbackOption;
use ilhook::x86::{ClosureHookPoint, HookFlags, Registers, hook_closure_jmp_back};
use tracing::info;

use crate::MonsterData;
use crate::hzv::HitzoneInfo;
use crate::monster::DamageInstance;
use crate::monster::MonsterStruct;
use crate::{address::Addresses, monster::Monster};

fn hook_quest_func<'a>(
    addresses: Addresses,
    tx: Sender<MonsterData>,
) -> Result<ClosureHookPoint<'a>> {
    let on_call = move |_| {
        if let Some(monster_structs) = addresses.monster_structs() {
            let mut monsters: Vec<Monster> = Vec::new();
            for monster_struct in monster_structs {
                let monster = Monster::from_struct(&addresses, monster_struct);
                monsters.push(monster);
            }
            if !monsters.is_empty() {
                let _ = tx.send(MonsterData::Monsters(monsters));
            }
        }
    };
    unsafe {
        hook_closure_jmp_back(
            addresses.quest_func,
            on_call,
            CallbackOption::None,
            HookFlags::empty(),
        )
        .map_err(|e| anyhow!("{e}"))
    }
}

fn hook_damage_calc<'a>(
    addresses: Addresses,
    tx: Sender<MonsterData>,
) -> Result<ClosureHookPoint<'a>> {
    let on_call = move |reg: *mut Registers| unsafe {
        if let Some(own_player) = addresses.own_player_addr() {
            let damage_source_addr = (((*reg).ebp - 0x84) as *const usize).read();
            if damage_source_addr == own_player {
                let hitzone_info = ((*reg).eax as *const HitzoneInfo).read();
                let damaged_monster = MonsterStruct::new((*reg).edi as *const u8);
                let damage_instance = DamageInstance {
                    monster_id: damaged_monster.monster_id(),
                    struct_idx: damaged_monster.struct_idx(),
                    hitzone: hitzone_info,
                };
                let _ = tx.send(MonsterData::DamageInstance(damage_instance));
            }
        }
    };
    unsafe {
        hook_closure_jmp_back(
            addresses.damage_calc_get_hzv,
            on_call,
            CallbackOption::None,
            HookFlags::empty(),
        )
        .map_err(|e| anyhow!("{e}"))
    }
}

pub fn init<'a>(
    addresses: &Addresses,
    tx: Sender<MonsterData>,
) -> Result<Vec<ClosureHookPoint<'a>>> {
    Ok(vec![
        hook_quest_func(*addresses, tx.clone())?,
        hook_damage_calc(*addresses, tx)?,
    ])
}

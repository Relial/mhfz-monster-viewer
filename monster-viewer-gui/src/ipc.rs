use std::{
    env::current_exe,
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream},
    path::PathBuf,
    sync::mpsc::Sender,
    thread,
    time::{Duration, Instant},
};

use anyhow::{Result, anyhow};
use dll_syringe::{Syringe, process::OwnedProcess};
use egui::Context;
use serde::Deserialize;
use tracing::info;

use crate::{
    game_data::{DamageInstance, Monster},
    ui::{Highlight, HighlightID, HzvColumn},
};

#[derive(Deserialize)]
pub enum MonsterData {
    Monsters(Vec<Monster>),
    DamageInstance(DamageInstance),
}

struct GameConnection {
    stream: TcpStream,
    buf: [u8; 1024],
}

impl GameConnection {
    fn new() -> Result<Self> {
        Ok(Self {
            stream: connect()?,
            buf: [0; 1024],
        })
    }
}

pub fn handle_game_connection(ui_ctx: Context, ipc_tx: Sender<(MonsterData, Vec<Highlight>)>) {
    loop {
        if let Ok(mut connection) = GameConnection::new() {
            let mut previous_monsters: Vec<Monster> = Vec::new();
            while let Ok((mut monster_data, _)) =
                postcard::from_io::<MonsterData, _>((&mut connection.stream, &mut connection.buf))
            {
                let mut send_to_ui = false;
                let mut highlights = Vec::new();
                match &mut monster_data {
                    MonsterData::Monsters(monsters) => {
                        let now = Instant::now();
                        for (monster_i, monster) in monsters.iter_mut().enumerate() {
                            if let Some(prev) = previous_monsters.get(monster_i)
                                && *prev == *monster
                            {
                                if prev.attack_multi != monster.attack_multi
                                    || prev.defense_multi != monster.defense_multi
                                    || prev.current_health != monster.current_health
                                    || prev.max_health != monster.max_health
                                {
                                    send_to_ui = true;
                                }
                                for (part_i, part) in monster.parts.iter_mut().enumerate() {
                                    if let Some(prev_part) = prev.parts.get(part_i) {
                                        if let Some(changes) = part.get_changes(prev_part) {
                                            for (i, change) in changes.iter().enumerate() {
                                                if *change {
                                                    highlights.push(Highlight {
                                                        id: HighlightID {
                                                            monster_struct_idx: monster.struct_idx,
                                                            part_idx: part.part_idx,
                                                            hzv_idx: part.hzv_idx,
                                                            column: HzvColumn::from_repr(i)
                                                                .unwrap(),
                                                        },
                                                        triggered: now,
                                                    });
                                                }
                                            }
                                            send_to_ui = true;
                                        }
                                    } else {
                                        send_to_ui = true;
                                    }
                                }
                            } else {
                                send_to_ui = true;
                            }
                        }
                        if send_to_ui {
                            previous_monsters = monsters.clone();
                        }
                    }
                    MonsterData::DamageInstance(_) => {
                        send_to_ui = true;
                    }
                }

                if send_to_ui {
                    if ipc_tx.send((monster_data, highlights)).is_err() {
                        break;
                    }
                    ui_ctx.request_repaint();
                }
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
}

fn inject() -> Result<()> {
    let target_process =
        OwnedProcess::find_first_by_name("mhf.exe").ok_or(anyhow!("MHFZ not running"))?;
    info!("Found target process");

    let syringe = Syringe::for_process(target_process);
    let dll_path = if cfg!(debug_assertions) {
        PathBuf::from("./target/i686-pc-windows-msvc/debug/monster_viewer.dll")
    } else {
        let mut path = current_exe()?;
        path.pop();
        path.push("monster_viewer.dll");
        path
    };
    let _injected = syringe.find_or_inject(dll_path)?;
    info!("Injected successfully");
    Ok(())
}

fn connect() -> Result<TcpStream> {
    inject()?;
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6802);
    TcpStream::connect_timeout(&socket_addr, Duration::from_secs(1)).map_err(|e| anyhow!("{e}"))
}

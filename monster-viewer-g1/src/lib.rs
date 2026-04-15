use std::{
    ffi::c_void,
    net::TcpListener,
    sync::mpsc::{self, Receiver},
};

use anyhow::Result;
use mimalloc::MiMalloc;
use serde::Serialize;
use tracing::{info, warn};
use windows::Win32::{
    Foundation::{CloseHandle, HINSTANCE, HMODULE},
    System::{
        Console::{AllocConsole, FreeConsole},
        LibraryLoader::FreeLibraryAndExitThread,
        SystemServices::DLL_PROCESS_ATTACH,
        Threading::{CreateThread, THREAD_CREATION_FLAGS},
    },
};

mod address;
mod hooks;
mod hzv;
mod monster;

use crate::{address::find_main_dll, monster::{DamageInstance, Monster}};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Serialize)]
pub enum MonsterData {
    Monsters(Vec<Monster>),
    DamageInstance(DamageInstance),
}

fn handle_ui_connection(game_rx: Receiver<MonsterData>) -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6802")?;
    let (mut stream, addr) = listener.accept()?;
    info!("Connection accepted: {addr}");
    while let Ok(monster_data) = game_rx.recv() {
        if postcard::to_io(&monster_data, &mut stream).is_err() {
            break;
        }
    }
    Ok(())
}

fn fallible() -> Result<()> {
    if cfg!(debug_assertions) {
        unsafe { AllocConsole()? };
        tracing_subscriber::fmt()
            .without_time()
            .with_ansi(false)
            .init();
        info!("Debug console enabled");
    }
    let addresses = find_main_dll();
    info!("Found dll address: {:X?}", addresses.dll);

    let (game_tx, game_rx) = mpsc::channel();
    let _hooks = hooks::init(&addresses, game_tx)?;
    info!("Started successfully.");
    handle_ui_connection(game_rx)?;
    info!("Lost UI connection. Exiting.");

    // Hooks are disabled when dropped
    Ok(())
}

extern "system" fn main(lp_parameter: *mut c_void) -> u32 {
    if let Err(e) = fallible() {
        warn!("Something went wrong: {e}");
    }
    unsafe {
        FreeLibraryAndExitThread(HMODULE(lp_parameter), 0);
    }
    1 // Never reached
}

#[unsafe(no_mangle)]
extern "system" fn DllMain(hinst: HINSTANCE, fdw_reason: u32, _lpv_reserved: *mut ()) -> bool {
    if fdw_reason == DLL_PROCESS_ATTACH {
        unsafe {
            if let Ok(handle) = CreateThread(
                None,
                0,
                Some(main),
                Some(hinst.0),
                THREAD_CREATION_FLAGS(0),
                None,
            ) {
                let _ = CloseHandle(handle);
            }
        }
    }
    true
}

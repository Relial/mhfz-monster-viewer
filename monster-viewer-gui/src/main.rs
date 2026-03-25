#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env::current_exe;

use anyhow::{Result, anyhow};
use egui::{self, Pos2, Vec2};
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod states;
mod game_data;
mod ipc;
mod label;
mod save;
mod ui;

const DEFAULT_SIZE: Vec2 = Vec2::new(550.0, 400.0);
const DEFAULT_POS: Pos2 = Pos2::new(0.0, 200.0);

use crate::{save::Save, ui::{MonsterStatesView, Viewer}};

fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    // eframe has the window position but every method returns 0, 0 instead so need this crap so cool
    // could use win32 but nothing else strictly depends on windows so no thanks
    let persistence_path = current_exe()?
        .parent()
        .ok_or(anyhow!("Couldn't get executable directory"))?
        .join("window.ron");

    let save = Save::load().unwrap_or_default();
    let seen_states = MonsterStatesView::new(save.states);
    let mut options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size(DEFAULT_SIZE)
            .with_position(DEFAULT_POS)
            .with_resizable(true)
            .with_decorations(false)
            .with_transparent(true)
            .with_icon(egui::viewport::IconData::default()),
        persistence_path: Some(persistence_path),
        ..Default::default()
    };
    if save.settings.always_on_top {
        options.viewport.window_level = Some(egui::WindowLevel::AlwaysOnTop);
    }
    eframe::run_native(
        "Monster viewer",
        options,
        Box::new(move |ctx| {
            let style = egui::Style {
                interaction: egui::style::Interaction {
                    tooltip_delay: 0.2,
                    ..Default::default()
                },
                wrap_mode: Some(egui::TextWrapMode::Extend),
                ..Default::default()
            };
            ctx.egui_ctx.set_style(style);
            ctx.egui_ctx.set_theme(egui::Theme::Dark);
            Ok(Box::new(Viewer::new(
                ctx.egui_ctx.clone(),
                save.settings,
                save.labels,
                save.columns,
                seen_states,
            )))
        }),
    )
    .map_err(|e| anyhow!("{e}"))
}

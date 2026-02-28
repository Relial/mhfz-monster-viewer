#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::{Result, anyhow};
use egui::{self, Color32, Stroke, Vec2, Visuals, style::Selection};
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod game_data;
mod ipc;
mod label;
mod save;
mod ui;

use crate::ui::Viewer;

fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    let initial_size = Vec2::new(550., 400.);
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size(initial_size)
            .with_always_on_top()
            .with_position([0., 200.])
            .with_resizable(true)
            .with_decorations(false)
            .with_transparent(true)
            .with_icon(egui::viewport::IconData::default()),
        ..Default::default()
    };
    eframe::run_native(
        "Monster viewer",
        options,
        Box::new(|ctx| {
            let style = egui::Style {
                interaction: egui::style::Interaction {
                    tooltip_delay: 0.2,
                    ..Default::default()
                },
                // visuals: Visuals {
                //     selection: Selection {
                //         bg_fill: Color32::TRANSPARENT,
                //         stroke: Stroke::new(1.0, Color32::GRAY),
                //     },
                //     ..Default::default()
                // },
                wrap_mode: Some(egui::TextWrapMode::Extend),
                ..Default::default()
            };
            ctx.egui_ctx.set_style(style);
            ctx.egui_ctx.set_theme(egui::Theme::Dark);
            Ok(Box::new(Viewer::new(ctx.egui_ctx.clone())))
        }),
    )
    .map_err(|e| anyhow!("{e}"))
}

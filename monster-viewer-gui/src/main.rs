#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::{Result, anyhow};
use egui::{self, Pos2, Vec2};
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod game_data;
mod ipc;
mod label;
mod save;
mod ui;

const DEFAULT_SIZE: Vec2 = Vec2::new(550.0, 400.0);
const DEFAULT_POS: Pos2 = Pos2::new(0.0, 200.0);

use crate::{
    label::Labels,
    save::load_settings,
    ui::{Settings, TABLE_COLUMNS, Viewer},
};

fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    let (settings, labels, columns, window_size, window_pos) =
        if let Ok((mut saved_settings, labels)) = load_settings() {
            saved_settings.default_column_widths_colors();
            (
                saved_settings.settings,
                labels,
                saved_settings.columns,
                saved_settings.window_size.unwrap_or(DEFAULT_SIZE),
                saved_settings.window_pos.unwrap_or(DEFAULT_POS),
            )
        } else {
            let settings = Settings {
                always_on_top: true,
                background_opacity: 204,
                highlight_changes: true,
            };
            let labels = Labels::default();
            (settings, labels, TABLE_COLUMNS, DEFAULT_SIZE, DEFAULT_POS)
        };
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size(window_size)
            .with_always_on_top()
            .with_position(window_pos)
            .with_resizable(true)
            .with_decorations(false)
            .with_transparent(true)
            .with_icon(egui::viewport::IconData::default()),
        ..Default::default()
    };
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
                settings,
                labels,
                columns,
            )))
        }),
    )
    .map_err(|e| anyhow!("{e}"))
}

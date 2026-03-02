use std::{
    env::current_exe,
    fs::File,
    io::{BufReader, BufWriter},
};

use anyhow::{Result, anyhow};
use egui::{Pos2, Vec2};
use serde::{Deserialize, Serialize};

use crate::{
    label::Labels,
    ui::{self, TABLE_COLUMNS, TableColumn, Viewer},
};

const SETTINGS_FILENAME: &str = "settings.json";
const LABELS_FILENAME: &str = "labels.json";

#[derive(Serialize, Deserialize)]
#[serde(rename = "Settings")]
pub struct SavedSettings {
    pub settings: ui::Settings,
    pub columns: [TableColumn; 13],
    pub window_size: Option<Vec2>,
    pub window_pos: Option<Pos2>,
}

impl SavedSettings {
    pub fn default_column_widths_colors(&mut self) {
        for (def, save) in TABLE_COLUMNS.iter().zip(&mut self.columns) {
            save.width = def.width;
            save.color = def.color;
        }
    }
}

pub fn load_settings() -> Result<(SavedSettings, Labels)> {
    let exe_path = current_exe()?;
    let exe_dir = exe_path
        .parent()
        .ok_or(anyhow!("Couldn't find exe directory"))?;
    let settings_path = exe_dir.join(SETTINGS_FILENAME);
    let labels_path = exe_dir.join(LABELS_FILENAME);
    let settings_file = File::open(settings_path)?;
    let labels_file = File::open(labels_path)?;
    let settings_reader = BufReader::new(settings_file);
    let labels_reader = BufReader::new(labels_file);
    let settings = serde_json::from_reader(settings_reader)?;
    let labels = serde_json::from_reader(labels_reader)?;
    Ok((settings, labels))
}

pub fn save_settings(
    viewer: &Viewer,
    window_size: Option<Vec2>,
    window_pos: Option<Pos2>,
) -> Result<()> {
    let settings = SavedSettings {
        settings: viewer.settings,
        columns: viewer.columns,
        window_size,
        window_pos,
    };
    let exe_path = current_exe()?;
    let exe_dir = exe_path
        .parent()
        .ok_or(anyhow!("Couldn't find exe directory"))?;
    let settings_path = exe_dir.join(SETTINGS_FILENAME);
    let labels_path = exe_dir.join(LABELS_FILENAME);
    let settings_file = File::create(settings_path)?;
    let labels_file = File::create(labels_path)?;
    let settings_writer = BufWriter::new(settings_file);
    let labels_writer = BufWriter::new(labels_file);
    serde_json::to_writer(settings_writer, &settings)?;
    serde_json::to_writer(labels_writer, &viewer.labels)?;
    Ok(())
}

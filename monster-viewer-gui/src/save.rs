use std::{
    env::current_exe,
    fs::File,
    io::{BufReader, BufWriter},
};

use anyhow::{Result, anyhow};
use serde::Deserialize;

use crate::{
    label::Labels,
    ui::{Settings, TABLE_COLUMNS, TableColumn, Viewer},
};

const SETTINGS_FILENAME: &str = "settings.json";

#[derive(Deserialize)]
pub struct Saved {
    pub settings: Settings,
    pub columns: [TableColumn; 13],
    pub labels: Labels,
}

impl Saved {
    pub fn default_widths(&mut self) {
        for (def, save) in TABLE_COLUMNS.iter().zip(&mut self.columns) {
            save.width = def.width;
        }
    }
}

pub fn load_settings() -> Option<Saved> {
    let path = current_exe().ok()?.parent()?.join(SETTINGS_FILENAME);
    let file = File::open(path).ok()?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).ok()
}

pub fn save_settings(viewer: &Viewer) -> Result<()> {
    let path = current_exe()?
        .parent()
        .ok_or(anyhow!("Couldn't find exe directory"))?
        .join(SETTINGS_FILENAME);
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, viewer)?;
    Ok(())
}

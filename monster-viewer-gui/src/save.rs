use std::{
    env::current_exe,
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

use crate::{
    states::MonstersWithStates,
    label::Labels,
    ui::{Settings, TABLE_COLUMNS, TableColumn, Viewer},
};

const SETTINGS_FILENAME: &str = "settings.json";
const LABELS_FILENAME: &str = "labels.json";
const STATES_FILENAME: &str = "seen_states.json";

#[derive(Serialize, Deserialize)]
#[serde(rename = "Settings")]
pub struct Save {
    pub settings: Settings,
    pub columns: [TableColumn; 13],
    pub labels: Labels,
    pub states: MonstersWithStates,
}

impl Save {
    pub fn load() -> Result<Self> {
        let exe_path = current_exe()?;
        let exe_dir = exe_path
            .parent()
            .ok_or(anyhow!("Couldn't find exe directory"))?;
        let settings_path = exe_dir.join(SETTINGS_FILENAME);
        let labels_path = exe_dir.join(LABELS_FILENAME);
        let states_path = exe_dir.join(STATES_FILENAME);
        let (settings, mut columns) =
            load_settings(&settings_path).unwrap_or_else(|_| (Settings::default(), TABLE_COLUMNS));
        let labels = load_labels(&labels_path).unwrap_or_else(|_| Labels::default());
        let states =
            load_monster_states(&states_path).unwrap_or_else(|_| MonstersWithStates::new());
        for (def, save) in TABLE_COLUMNS.iter().zip(&mut columns) {
            save.width = def.width;
            save.color = def.color;
        }
        let save = Save {
            settings,
            columns,
            labels,
            states,
        };
        Ok(save)
    }
}

fn load_settings(path: &Path) -> Result<(Settings, [TableColumn; 13])> {
    let settings_file = File::open(path)?;
    let settings_reader = BufReader::new(settings_file);
    let (settings, columns) = serde_json::from_reader(settings_reader)?;
    Ok((settings, columns))
}

pub fn load_labels(path: &Path) -> Result<Labels> {
    let labels_file = File::open(path)?;
    let labels_reader = BufReader::new(labels_file);
    let labels = serde_json::from_reader(labels_reader)?;
    Ok(labels)
}

pub fn load_monster_states(path: &Path) -> Result<MonstersWithStates> {
    let states_file = File::open(path)?;
    let states_reader = BufReader::new(states_file);
    let states = serde_json::from_reader(states_reader)?;
    Ok(states)
}

pub fn save_settings(viewer: &Viewer) -> Result<()> {
    let exe_path = current_exe()?;
    let exe_dir = exe_path
        .parent()
        .ok_or(anyhow!("Couldn't find exe directory"))?;
    let settings_path = exe_dir.join(SETTINGS_FILENAME);
    let labels_path = exe_dir.join(LABELS_FILENAME);
    let states_path = exe_dir.join(STATES_FILENAME);
    let settings_file = File::create(settings_path)?;
    let labels_file = File::create(labels_path)?;
    let states_file = File::create(states_path)?;
    let settings_writer = BufWriter::new(settings_file);
    let labels_writer = BufWriter::new(labels_file);
    let states_writer = BufWriter::new(states_file);
    serde_json::to_writer_pretty(settings_writer, &(viewer.settings, viewer.columns))?;
    serde_json::to_writer(labels_writer, &viewer.labels)?;
    serde_json::to_writer(states_writer, &viewer.states.seen_states)?;
    Ok(())
}

impl Default for Save {
    fn default() -> Self {
        Self {
            settings: Default::default(),
            columns: TABLE_COLUMNS,
            labels: Default::default(),
            states: MonstersWithStates::new(),
        }
    }
}

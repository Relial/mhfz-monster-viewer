use std::{
    borrow::Borrow,
    fmt::Display,
    sync::mpsc::{self, Receiver},
    thread,
    time::{Duration, Instant},
};

use circular_buffer::CircularBuffer;
use egui::{
    self, Align, Button, CollapsingHeader, Color32, CornerRadius, CursorIcon, Direction, Frame,
    Layout, Margin, Pos2, Rect, ResizeDirection, RichText, Sense, Shadow, Stroke, Vec2,
    ViewportCommand, WindowLevel, scroll_area::ScrollSource, vec2,
};
use egui_extras::{Column, TableBuilder};
use num_format::{Locale, ToFormattedString};
use rapidhash::RapidHashSet;
use serde::{Deserialize, Serialize};
use sort_const::const_quicksort;
use strum::FromRepr;

use crate::{
    game_data::{DamageInstance, MONSTER_COUNT, MONSTERS_ALPHABETICAL, Monster, monster_name},
    ipc::{MonsterData, handle_game_connection},
    label::Labels,
    save::save_settings,
    states::{MonsterStates, MonstersWithStates},
};

const FIRE: Color32 = Color32::from_rgb(255, 72, 2);
const WATER: Color32 = Color32::from_rgb(146, 235, 255);
const ICE: Color32 = Color32::from_rgb(173, 206, 247);
const THUNDER: Color32 = Color32::from_rgb(255, 254, 3);
const DRAGON: Color32 = Color32::from_rgb(107, 114, 182);

pub const TABLE_COLUMNS: [TableColumn; 13] = [
    TableColumn::new(HzvColumn::Part, Color32::GRAY, 80.0),
    TableColumn::new(HzvColumn::Hzv, Color32::GRAY, 80.0),
    TableColumn::new(HzvColumn::Count, Color32::GRAY, 35.0),
    TableColumn::new(HzvColumn::Health, Color32::GRAY, 40.0),
    TableColumn::new(HzvColumn::Cut, Color32::WHITE, 25.0),
    TableColumn::new(HzvColumn::Impact, Color32::WHITE, 33.0),
    TableColumn::new(HzvColumn::Shot, Color32::WHITE, 28.0),
    TableColumn::new(HzvColumn::Fire, FIRE, 25.0),
    TableColumn::new(HzvColumn::Water, WATER, 25.0),
    TableColumn::new(HzvColumn::Ice, ICE, 25.0),
    TableColumn::new(HzvColumn::Thunder, THUNDER, 25.0),
    TableColumn::new(HzvColumn::Dragon, DRAGON, 25.0),
    TableColumn::new(HzvColumn::Stun, Color32::GRAY, 30.0),
];

const HIGHLIGHT_FADE: Duration = Duration::from_secs(2);
const HIGHLIGHT_REFRESH: Duration = Duration::from_millis(50);

const PANEL_FRAME: Frame = Frame {
    inner_margin: Margin::same(6),
    fill: Color32::TRANSPARENT,
    stroke: Stroke::NONE,
    corner_radius: CornerRadius::ZERO,
    outer_margin: Margin::ZERO,
    shadow: Shadow::NONE,
};

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, FromRepr)]
pub enum HzvColumn {
    Part,
    Hzv,
    Count,
    Health,
    Cut,
    Impact,
    Shot,
    Fire,
    Water,
    Ice,
    Thunder,
    Dragon,
    Stun,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct TableColumn {
    kind: HzvColumn,
    enabled: bool,
    #[serde(skip)]
    pub color: Color32,
    #[serde(skip)]
    pub width: f32,
}

impl TableColumn {
    const fn new(kind: HzvColumn, color: Color32, column_width: f32) -> Self {
        Self {
            kind,
            enabled: true,
            color,
            width: column_width,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Settings {
    pub always_on_top: bool,
    pub background_opacity: u8,
    pub highlight_changes: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            always_on_top: true,
            background_opacity: 204,
            highlight_changes: true,
        }
    }
}

#[derive(Clone, Copy, Eq)]
pub struct Highlight {
    pub id: HighlightID,
    pub triggered: Instant,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct HighlightID {
    pub monster_struct_idx: u16,
    pub part_idx: u16,
    pub hzv_idx: u16,
    pub column: HzvColumn,
}

pub struct MonsterStatesView {
    pub seen_states: MonstersWithStates,
    selected_monster: u8,
}

impl MonsterStatesView {
    pub fn new(seen_states: MonstersWithStates) -> Self {
        Self {
            seen_states,
            selected_monster: 0x68, // Abiorg, first in alphabetical order
        }
    }
}

struct PartHzvHitStats {
    part_idx: u16,
    hzv_idx: u16,
    hits: u64,
}

impl PartHzvHitStats {
    fn new(damage_instance: &DamageInstance) -> Self {
        Self {
            part_idx: damage_instance.hitzone.part_idx,
            hzv_idx: damage_instance.hitzone.hzv_idx,
            hits: 1,
        }
    }
}

struct MonsterHitStats {
    monster_id: u8,
    hits: Vec<PartHzvHitStats>,
}

impl MonsterHitStats {
    fn new(damage_instance: &DamageInstance) -> Self {
        Self {
            monster_id: damage_instance.monster_id,
            hits: vec![PartHzvHitStats::new(damage_instance)],
        }
    }

    fn increment_or_add(&mut self, damage_instance: &DamageInstance) {
        if let Some(part_stats) = self.hits.iter_mut().find(|stats| {
            stats.part_idx == damage_instance.hitzone.part_idx
                && stats.hzv_idx == damage_instance.hitzone.hzv_idx
        }) {
            part_stats.hits += 1;
        } else {
            self.hits.push(PartHzvHitStats::new(damage_instance));
            self.hits.sort_by_key(|hit| (hit.part_idx, hit.hzv_idx));
        }
    }
}

struct HitLogStats {
    monsters: Vec<MonsterHitStats>,
}

impl HitLogStats {
    fn new() -> Self {
        Self {
            monsters: Vec::new(),
        }
    }

    fn add_hit(&mut self, damage_instance: &DamageInstance) {
        match self
            .monsters
            .binary_search_by_key(&damage_instance.monster_id, |mon_stats| {
                mon_stats.monster_id
            }) {
            Ok(stats_idx) => {
                self.monsters[stats_idx].increment_or_add(damage_instance);
            }
            Err(stats_idx) => {
                self.monsters
                    .insert(stats_idx, MonsterHitStats::new(damage_instance));
            }
        }
    }
}

pub struct Viewer {
    pub settings: Settings,
    ipc_rx: Receiver<(MonsterData, Vec<Highlight>)>,
    monsters: Vec<Monster>,
    hit_log: Box<CircularBuffer<1000, DamageInstance>>,
    hit_log_stats: HitLogStats,
    pub columns: [TableColumn; 13],
    pub labels: Labels,
    highlights: RapidHashSet<Highlight>,
    pub states: MonsterStatesView,
}

impl eframe::App for Viewer {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(PANEL_FRAME.fill(Color32::from_rgba_unmultiplied(
                12,
                12,
                12,
                self.settings.background_opacity,
            )))
            .show(ctx, |ui| {
                let viewer_rect = ui.max_rect();
                let background_resp = ui.interact(viewer_rect, "Move area".into(), Sense::DRAG);
                if background_resp.drag_started_by(egui::PointerButton::Primary) {
                    ctx.send_viewport_cmd(ViewportCommand::StartDrag);
                }

                handle_zoom(ctx);

                egui::TopBottomPanel::top("Top panel")
                    .frame(Frame::NONE)
                    .show_separator_line(false)
                    .resizable(false)
                    .show_inside(ui, |ui| {
                        self.controls(ui, ctx);
                    });
                egui::CentralPanel::default()
                    .frame(Frame::NONE)
                    .show_inside(ui, |ui| {
                        egui::ScrollArea::both()
                            .scroll_source(ScrollSource {
                                scroll_bar: true,
                                drag: false,
                                mouse_wheel: true,
                            })
                            .show(ui, |ui| {
                                ui.take_available_space();
                                self.receive_data();
                                self.filter_columns(ui);
                                let enabled_columns: Vec<TableColumn> = self
                                    .columns
                                    .iter()
                                    .filter(|col| col.enabled)
                                    .copied()
                                    .collect();
                                self.history(ui, &enabled_columns);
                                self.monster_viewer(ui, ctx, &enabled_columns);
                            });
                    });
                let window_rect = ctx.viewport_rect().shrink(1.);
                handle_window_resize(ui, ctx, window_rect);
            });

        if ctx.input(|i| i.viewport().close_requested()) {
            let _ = save_settings(self);
        }
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
}

impl Viewer {
    pub fn new(
        ctx: egui::Context,
        settings: Settings,
        labels: Labels,
        columns: [TableColumn; 13],
        states_view: MonsterStatesView,
    ) -> Self {
        let (ipc_tx, ipc_rx) = mpsc::channel();
        let viewer = Self {
            settings,
            ipc_rx,
            monsters: Vec::new(),
            hit_log: CircularBuffer::boxed(),
            columns,
            labels,
            highlights: RapidHashSet::default(),
            states: states_view,
            hit_log_stats: HitLogStats::new(),
        };
        thread::spawn(|| {
            handle_game_connection(ctx, ipc_tx);
        });
        viewer
    }

    fn controls(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.);
            let button_height = Vec2::splat(20.);
            let close_resp = ui.add(Button::new("❌").min_size(button_height));
            if close_resp.clicked() {
                ctx.send_viewport_cmd(ViewportCommand::Close);
            }

            ui.visuals_mut().button_frame = true;
            ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                ui.label("Background opacity: ");
                ui.add_sized(
                    [20., 20.],
                    egui::DragValue::new(&mut self.settings.background_opacity)
                        .custom_formatter(|n, _| {
                            let actual = (n / 255. * 100.).round().clamp(0., 100.) as usize;
                            actual.to_string()
                        })
                        .custom_parser(|s| {
                            let n: f64 = s.parse().ok()?;
                            let actual = (n / 100. * 255.).round().clamp(0., 255.);
                            Some(actual)
                        })
                        .range(0..=255),
                );
                let on_top_resp = ui.checkbox(&mut self.settings.always_on_top, "Always on top");
                if on_top_resp.clicked() {
                    let level = if self.settings.always_on_top {
                        WindowLevel::AlwaysOnTop
                    } else {
                        WindowLevel::Normal
                    };
                    ctx.send_viewport_cmd(ViewportCommand::WindowLevel(level));
                }
                ui.checkbox(&mut self.settings.highlight_changes, "Highlight changes");
            });
        });
    }

    fn receive_data(&mut self) {
        while let Ok((monster_data, highlights)) = self.ipc_rx.try_recv() {
            match monster_data {
                MonsterData::Monsters(monsters) => {
                    for monster in &monsters {
                        self.states
                            .seen_states
                            .handle_new(monster.monster_id, &monster.parts);
                    }
                    self.monsters = monsters;
                    if self.settings.highlight_changes {
                        for new in highlights {
                            self.highlights.replace(new);
                        }
                    }
                }
                MonsterData::DamageInstance(damage_instance) => {
                    self.hit_log.push_front(damage_instance);
                    self.hit_log_stats.add_hit(&damage_instance);
                }
            }
        }
    }

    fn filter_columns(&mut self, ui: &mut egui::Ui) {
        CollapsingHeader::new("Filter columns").show(ui, |ui| {
            egui::Grid::new("Filters").show(ui, |ui| {
                for columns in self.columns.chunks_mut(7) {
                    for column in columns {
                        ui.checkbox(
                            &mut column.enabled,
                            RichText::new(column.kind.to_string()).color(column.color),
                        );
                    }
                    ui.end_row();
                }
            });
        });
    }

    fn history(&mut self, ui: &mut egui::Ui, enabled_columns: &[TableColumn]) {
        CollapsingHeader::new("History").show(ui, |ui| {
            self.damage_history(ui);
            self.hitzone_state_history(ui, enabled_columns);
        });
    }

    fn damage_history(&mut self, ui: &mut egui::Ui) {
        CollapsingHeader::new("Stats").show(ui, |ui| {
            if ui.button("Reset stats").clicked() {
                self.hit_log_stats.monsters.clear();
            }
            for monster in &self.hit_log_stats.monsters {
                CollapsingHeader::new(monster_name(monster.monster_id)).show(ui, |ui| {
                    TableBuilder::new(ui)
                        .striped(true)
                        .cell_layout(Layout::centered_and_justified(Direction::TopDown))
                        .columns(Column::auto(), 3)
                        .header(18.0, |mut header| {
                            header.col(|ui| {
                                ui.label("Part");
                            });
                            header.col(|ui| {
                                ui.label("HZV");
                            });
                            header.col(|ui| {
                                ui.label("Hits");
                            });
                        })
                        .body(|body| {
                            body.rows(18.0, monster.hits.len(), |mut row| {
                                let stats = &monster.hits[row.index()];
                                let Some(labels) = self.labels.monster(monster.monster_id as usize)
                                else {
                                    return;
                                };
                                let part = labels.part(stats.part_idx as usize);
                                row.col(|ui| {
                                    ui.label(&part.label);
                                });
                                row.col(|ui| {
                                    if let Some(label) = part.get_hzv(stats.hzv_idx as usize) {
                                        ui.label(label);
                                    } else {
                                        ui.label(stats.hzv_idx.to_string());
                                    }
                                });
                                row.col(|ui| {
                                    ui.label(stats.hits.to_string());
                                });
                            });
                        });
                });
            }
        });
        CollapsingHeader::new("Attacks").show(ui, |ui| {
            TableBuilder::new(ui)
                .striped(true)
                .cell_layout(Layout::centered_and_justified(Direction::TopDown))
                .max_scroll_height(205.)
                .columns(Column::auto(), 6)
                .header(18.0, |mut header| {
                    header.col(|ui| {
                        ui.label("Monster");
                    });
                    header.col(|ui| {
                        ui.label("Part");
                    });
                    header.col(|ui| {
                        ui.label("HZV");
                    });
                    header.col(|ui| {
                        ui.label("Scale");
                    });
                    header.col(|ui| {
                        ui.label("Vector 1");
                    });
                    header.col(|ui| {
                        ui.label("Vector 2");
                    });
                })
                .body(|body| {
                    body.rows(18.0, self.hit_log.len(), |mut row| {
                        let hit = self.hit_log[row.index()];
                        let Some(labels) = self.labels.monster(hit.monster_id as usize) else {
                            return;
                        };
                        let part = labels.part(hit.hitzone.part_idx as usize);
                        row.col(|ui| {
                            ui.label(monster_name(hit.monster_id));
                        });
                        row.col(|ui| {
                            let label = &part.label;
                            ui.label(label);
                        });
                        row.col(|ui| {
                            if let Some(label) = part.get_hzv(hit.hitzone.hzv_idx as usize) {
                                ui.label(label);
                            } else {
                                ui.label(hit.hitzone.hzv_idx.to_string());
                            }
                        });
                        row.col(|ui| {
                            ui.label(hit.hitzone.scale.to_string());
                        });
                        row.col(|ui| {
                            ui.label(hit.hitzone.vec1.to_string());
                        });
                        row.col(|ui| {
                            if hit.hitzone.second_vector_indicator == 1 {
                                ui.label(hit.hitzone.vec2.to_string());
                            }
                        });
                    });
                });
        });
    }

    fn hitzone_state_history(&mut self, ui: &mut egui::Ui, enabled_columns: &[TableColumn]) {
        CollapsingHeader::new("HZVs").show(ui, |ui| {
            let current_monster_name = monster_name(self.states.selected_monster);
            egui::ComboBox::from_id_salt("Hitzone states monster selection")
                .selected_text(current_monster_name)
                .show_ui(ui, |ui| {
                    for (i, name) in MONSTERS_ALPHABETICAL {
                        ui.selectable_value(&mut self.states.selected_monster, i, name);
                    }
                });
            let Some(selected) = self.states.seen_states.get(self.states.selected_monster) else {
                ui.label("Bad monster idx in hitzone state history");
                return;
            };
            let enabled: Vec<_> = enabled_columns
                .iter()
                .filter(|column| {
                    column.kind != HzvColumn::Count && column.kind != HzvColumn::Health
                })
                .collect();
            for (i, parts) in selected.unique_states.iter().enumerate() {
                CollapsingHeader::new(format!("Entry {}", i)).show(ui, |ui| {
                    let Some(labels) = self.labels.monster(self.states.selected_monster as usize)
                    else {
                        ui.label("Error: Bad monster ID");
                        return;
                    };

                    TableBuilder::new(ui)
                        .striped(true)
                        .vscroll(false)
                        .columns(Column::auto(), enabled.len())
                        .header(18.0, |mut header| {
                            for column in &enabled {
                                header.col(|ui| {
                                    ui.with_layout(
                                        Layout::centered_and_justified(Direction::TopDown),
                                        |ui| {
                                            ui.colored_label(column.color, column.kind.to_string());
                                        },
                                    );
                                });
                            }
                        })
                        .body(|mut body| {
                            for part in parts {
                                body.row(18.0, |mut row| {
                                    for column in &enabled {
                                        row.col(|ui| {
                                            match &column.kind {
                                                HzvColumn::Part => {
                                                    ui.with_layout(
                                                        Layout::top_down(Align::Center),
                                                        |ui| {
                                                            let label = &labels
                                                                .part(part.part_idx as usize)
                                                                .label;
                                                            ui.colored_label(column.color, label);
                                                        },
                                                    );
                                                }
                                                HzvColumn::Hzv => {
                                                    ui.with_layout(
                                                        Layout::top_down(Align::Center),
                                                        |ui| {
                                                            let label = labels
                                                                .part(part.part_idx as usize)
                                                                .get_hzv(part.hzv_idx as usize)
                                                                .unwrap_or("???");
                                                            ui.colored_label(column.color, label);
                                                        },
                                                    );
                                                }
                                                _ => {
                                                    ui.with_layout(
                                                        Layout::centered_and_justified(
                                                            Direction::TopDown,
                                                        ),
                                                        |ui| {
                                                            let label =
                                                                part.table_display(column.kind);
                                                            ui.colored_label(column.color, label);
                                                        },
                                                    );
                                                }
                                            };
                                        });
                                    }
                                });
                            }
                        });
                });
            }
        });
    }

    fn monster_viewer(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        enabled_columns: &[TableColumn],
    ) {
        let mut damaged_rect = None;
        let mut widest_part: f32 = 0.0;
        let mut widest_hzv: f32 = 0.0;
        let paint_highlight = |ui: &mut egui::Ui, triggered: Instant| -> bool {
            let t = triggered.elapsed().div_duration_f64(HIGHLIGHT_FADE);
            if t > 1.0 {
                return false;
            }
            let bg_alpha = ((1. - t) * 255.).round().clamp(0., 255.) as u8;
            let highlight_color = Color32::from_rgba_unmultiplied(0, 92, 128, bg_alpha);
            let rect = ui.max_rect();
            ui.painter().rect_filled(rect, 0.0, highlight_color);
            ctx.request_repaint_after(HIGHLIGHT_REFRESH);
            true
        };
        for (i, monster) in self.monsters.iter().enumerate() {
            CollapsingHeader::new(format!(
                "Entry {} ({})",
                monster.struct_idx,
                monster_name(monster.monster_id)
            ))
            .default_open(i == 0)
            .show(ui, |ui| {
                let Some(labels) = self.labels.monster_mut(monster.monster_id as usize) else {
                    ui.label("Error: Bad monster ID");
                    return;
                };
                let effective_current =
                    (monster.current_health as f32 / monster.defense_multi).round() as u64;
                let effective_max =
                    (monster.max_health as f32 / monster.defense_multi).round() as u64;
                ui.label(format!(
                    "Health: {}/{} (effective {}/{})",
                    monster.current_health.to_formatted_string(&Locale::en),
                    monster.max_health.to_formatted_string(&Locale::en),
                    effective_current.to_formatted_string(&Locale::en),
                    effective_max.to_formatted_string(&Locale::en)
                ));
                ui.label(format!("Attack multiplier: {:.4}", monster.attack_multi));
                ui.label(format!("Defense multiplier: {:.4}", monster.defense_multi));
                ui.separator();
                if monster.parts.is_empty() {
                    return;
                }
                let mut builder = TableBuilder::new(ui).striped(true).vscroll(false);

                // All the auto sizing options cause row interact rects to span
                // 2 rows for some reason so this is the workaround
                for column in enabled_columns {
                    builder = builder.column(Column::exact(column.width));
                }
                builder
                    .header(18.0, |mut header| {
                        for column in enabled_columns {
                            header.col(|ui| {
                                ui.with_layout(
                                    Layout::centered_and_justified(Direction::TopDown),
                                    |ui| {
                                        let resp = ui.colored_label(column.color, column.kind.to_string());
                                        if column.kind == HzvColumn::Count {
                                            resp.on_hover_text("Amount of hitspheres of various sizes for each part/hzv combination");
                                        }
                                    },
                                );
                            });
                        }
                    })
                    .body(|mut body| {
                        let last_hit = self.hit_log.front();
                        for part in &monster.parts {
                            body.row(18.0, |mut row| {
                                for column in enabled_columns {
                                    row.col(|ui| {
                                        let highlight_id = HighlightID {
                                            monster_struct_idx: monster.struct_idx,
                                            part_idx: part.part_idx,
                                            hzv_idx: part.hzv_idx,
                                            column: column.kind,
                                        };
                                        if let Some(highlight) = self.highlights.get(&highlight_id) && !paint_highlight(ui, highlight.triggered) {
                                            self.highlights.remove(&highlight_id);
                                        }
                                        match &column.kind {
                                            HzvColumn::Part => {
                                                ui.with_layout(
                                                    Layout::top_down(Align::Center),
                                                    |ui| {
                                                        let label = &mut labels
                                                            .part_mut(part.part_idx as usize)
                                                            .label;
                                                        let resp =
                                                            egui::TextEdit::singleline(label)
                                                                .text_color(column.color)
                                                                .horizontal_align(Align::Center)
                                                                .clip_text(false)
                                                                .desired_width(30.0)
                                                                .margin(Margin::symmetric(8, 2))
                                                                .show(ui)
                                                                .response.on_hover_text(part.part_idx.to_string());
                                                        widest_part = (resp.rect.max.x
                                                            - resp.rect.min.x)
                                                            .max(widest_part);
                                                        if resp.lost_focus() && label.is_empty() {
                                                            *label = part.part_idx.to_string();
                                                        }
                                                    },
                                                );
                                            }
                                            HzvColumn::Hzv => {
                                                ui.with_layout(
                                                    Layout::top_down(Align::Center),
                                                    |ui| {
                                                        let label = labels
                                                            .part_mut(part.part_idx as usize)
                                                            .get_or_insert_hzv(
                                                                part.hzv_idx as usize,
                                                            );
                                                        let resp =
                                                            egui::TextEdit::singleline(label)
                                                                .text_color(column.color)
                                                                .horizontal_align(Align::Center)
                                                                .clip_text(false)
                                                                .desired_width(30.0)
                                                                .margin(Margin::symmetric(8, 2))
                                                                .show(ui)
                                                                .response.on_hover_text(part.hzv_idx.to_string());
                                                        widest_hzv = (resp.rect.max.x
                                                            - resp.rect.min.x)
                                                            .max(widest_hzv);
                                                        if resp.lost_focus() && label.is_empty() {
                                                            *label = part.hzv_idx.to_string();
                                                        }
                                                    },
                                                );
                                            }
                                            _ => {
                                                ui.with_layout(
                                                    Layout::centered_and_justified(
                                                        Direction::TopDown,
                                                    ),
                                                    |ui| {
                                                        let label = part.table_display(column.kind);
                                                        ui.colored_label(column.color, label);
                                                    },
                                                );
                                            }
                                        };
                                    });
                                }
                                if self.settings.highlight_changes
                                    && let Some(last) = last_hit
                                    && last.monster_id == monster.monster_id
                                    && last.struct_idx == monster.struct_idx
                                    && last.hitzone.hzv_idx == part.hzv_idx
                                    && last.hitzone.part_idx == part.part_idx
                                {
                                    damaged_rect = Some(row.response().interact_rect);
                                }
                            });
                        }
                    });
            });
        }

        if let Some(rect) = damaged_rect {
            let painter = ui.painter();
            let stroke = Stroke::new(0.5, Color32::WHITE);
            painter.hline(rect.x_range(), rect.top(), stroke);
            painter.hline(rect.x_range(), rect.bottom(), stroke);
            painter.vline(rect.left(), rect.y_range(), stroke);
            painter.vline(rect.right(), rect.y_range(), stroke);
        }
        self.columns[0].width = widest_part;
        self.columns[1].width = widest_hzv;
    }
}

#[derive(Default, Clone, Copy)]
struct WindowResize {
    left: ResizeSense,
    top: ResizeSense,
    right: ResizeSense,
    bottom: ResizeSense,
}

impl WindowResize {
    fn resize_direction(&self) -> Option<ResizeDirection> {
        match (
            self.left.drag,
            self.top.drag,
            self.right.drag,
            self.bottom.drag,
        ) {
            (true, false, false, false) => Some(ResizeDirection::West),
            (true, true, false, false) => Some(ResizeDirection::NorthWest),
            (false, true, false, false) => Some(ResizeDirection::North),
            (false, true, true, false) => Some(ResizeDirection::NorthEast),
            (false, false, true, false) => Some(ResizeDirection::East),
            (false, false, true, true) => Some(ResizeDirection::SouthEast),
            (false, false, false, true) => Some(ResizeDirection::South),
            (true, false, false, true) => Some(ResizeDirection::SouthWest),
            _ => None,
        }
    }

    fn cursor_icon(&self) -> Option<CursorIcon> {
        let left = self.left.any();
        let top = self.top.any();
        let right = self.right.any();
        let bottom = self.bottom.any();

        if (left && top) || (right && bottom) {
            Some(CursorIcon::ResizeNwSe)
        } else if (right && top) || (left && bottom) {
            Some(CursorIcon::ResizeNeSw)
        } else if left || right {
            Some(CursorIcon::ResizeHorizontal)
        } else if top || bottom {
            Some(CursorIcon::ResizeVertical)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Default)]
struct ResizeSense {
    hover: bool,
    drag: bool,
}

impl ResizeSense {
    fn any(&self) -> bool {
        self.hover || self.drag
    }

    fn with(&mut self, rhs: Self) {
        self.hover = self.hover || rhs.hover;
        self.drag = self.drag || rhs.drag;
    }
}

fn handle_window_resize(ui: &mut egui::Ui, ctx: &egui::Context, window_rect: Rect) {
    let style = ui.style();
    let side_grab_radius = style.interaction.resize_grab_radius_side;
    let corner_grab_radius = style.interaction.resize_grab_radius_corner;

    let vertical_rect = |a: Pos2, b: Pos2| {
        Rect::from_min_max(a, b).expand2(vec2(side_grab_radius, -corner_grab_radius))
    };
    let horizontal_rect =
        |a, b| Rect::from_min_max(a, b).expand2(vec2(-corner_grab_radius, side_grab_radius));
    let corner_rect =
        |center: Pos2| Rect::from_center_size(center, Vec2::splat(2.0 * corner_grab_radius));

    let resize_sense = |rect, id| {
        let resp = ui.interact(rect, id, Sense::DRAG);
        ResizeSense {
            hover: resp.hovered(),
            drag: resp.drag_started_by(egui::PointerButton::Primary),
        }
    };

    let mut resize = WindowResize::default();

    // Sides
    let left = resize_sense(
        vertical_rect(window_rect.left_top(), window_rect.left_bottom()),
        "left resize".into(),
    );
    resize.left.with(left);

    let top = resize_sense(
        horizontal_rect(window_rect.left_top(), window_rect.right_top()),
        "top resize".into(),
    );
    resize.top.with(top);

    let right = resize_sense(
        vertical_rect(window_rect.right_top(), window_rect.right_bottom()),
        "right resize".into(),
    );
    resize.right.with(right);

    let bottom = resize_sense(
        horizontal_rect(window_rect.left_bottom(), window_rect.right_bottom()),
        "bottom resize".into(),
    );
    resize.bottom.with(bottom);

    // Corners
    let top_left = resize_sense(
        corner_rect(window_rect.left_top()),
        "top left resize".into(),
    );
    resize.left.with(top_left);
    resize.top.with(top_left);

    let top_right = resize_sense(
        corner_rect(window_rect.right_top()),
        "top right resize".into(),
    );
    resize.right.with(top_right);
    resize.top.with(top_right);

    let bottom_right = resize_sense(
        corner_rect(window_rect.right_bottom()),
        "bottom right resize".into(),
    );
    resize.right.with(bottom_right);
    resize.bottom.with(bottom_right);

    let bottom_left = resize_sense(
        corner_rect(window_rect.left_bottom()),
        "bottom left resize".into(),
    );
    resize.left.with(bottom_left);
    resize.bottom.with(bottom_left);

    if let Some(direction) = resize.resize_direction() {
        ctx.send_viewport_cmd(ViewportCommand::BeginResize(direction));
    }
    if let Some(icon) = resize.cursor_icon() {
        ctx.set_cursor_icon(icon);
    }
}

fn handle_zoom(ctx: &egui::Context) {
    let delta = ctx.input(|i| i.zoom_delta());
    if delta != 1. {
        let change = 1. + (1. - delta) * 0.5;
        let current = ctx.zoom_factor();
        let new = (current * change).clamp(0.5, 2.);
        ctx.set_zoom_factor(new);
    }
}

impl Display for HzvColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HzvColumn::Part => "Part",
            HzvColumn::Hzv => "HZV",
            HzvColumn::Count => "Count",
            HzvColumn::Health => "HP",
            HzvColumn::Cut => "Cut",
            HzvColumn::Impact => "Blunt",
            HzvColumn::Shot => "Shot",
            HzvColumn::Fire => "Fire",
            HzvColumn::Water => "Wat",
            HzvColumn::Ice => "Ice",
            HzvColumn::Thunder => "Thu",
            HzvColumn::Dragon => "Dra",
            HzvColumn::Stun => "Stun",
        };
        write!(f, "{s}")
    }
}

impl PartialEq for Highlight {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::hash::Hash for Highlight {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Borrow<HighlightID> for Highlight {
    fn borrow(&self) -> &HighlightID {
        &self.id
    }
}

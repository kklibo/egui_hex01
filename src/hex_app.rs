use crate::diff::{self, HexCell};
use egui::{Color32, RichText, Ui};
use egui_extras::{Column, TableBody, TableBuilder, TableRow};
use rand::Rng;

#[derive(Debug, PartialEq)]
enum WhichFile {
    File0,
    File1,
}
fn drop_select_text(selected: bool) -> &'static str {
    if selected {
        "⬇ Loading dropped files here ⬇"
    } else {
        "⬇ Load dropped files here ⬇"
    }
}

pub struct HexApp {
    source_name0: Option<String>,
    source_name1: Option<String>,
    pattern0: Option<Vec<u8>>,
    pattern1: Option<Vec<u8>>,
    diffs0: Vec<HexCell>,
    diffs1: Vec<HexCell>,
    file_drop_target: WhichFile,
}

fn random_pattern() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..1000).map(|_| rng.gen_range(0..=255)).collect()
}

impl HexApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut result = Self {
            source_name0: Some("zeroes0".to_string()),
            source_name1: Some("zeroes1".to_string()),
            pattern0: Some(vec![0; 1000]),
            pattern1: Some(vec![0; 1000]),
            diffs0: vec![],
            diffs1: vec![],
            file_drop_target: WhichFile::File0,
        };

        result.update_diffs();
        result
    }

    fn update_diffs(&mut self) {
        let (diffs1, diffs2) =
            if let (Some(pattern0), Some(pattern1)) = (&self.pattern0, &self.pattern1) {
                diff::get_diffs(pattern0, pattern1, 0..100 * 16)
            } else {
                (vec![], vec![])
            };
        self.diffs0 = diffs1;
        self.diffs1 = diffs2;
    }

    fn add_header_row(&mut self, mut header: TableRow<'_, '_>) {
        let no_pattern = "[none]".to_string();

        header.col(|ui| {
            ui.heading("address");
        });
        header.col(|ui| {
            ui.heading(self.source_name0.as_ref().unwrap_or(&no_pattern));
            let text = drop_select_text(self.file_drop_target == WhichFile::File0);
            ui.selectable_value(&mut self.file_drop_target, WhichFile::File0, text)
                .highlight();
            if ui.button("randomize").clicked() {
                self.pattern0 = Some(random_pattern());
                self.source_name0 = Some("random".to_string());
                self.update_diffs();
            }
        });
        header.col(|_| {});
        header.col(|ui| {
            ui.heading(self.source_name1.as_ref().unwrap_or(&no_pattern));
            let text = drop_select_text(self.file_drop_target == WhichFile::File1);
            ui.selectable_value(&mut self.file_drop_target, WhichFile::File1, text)
                .highlight();
            if ui.button("randomize").clicked() {
                self.pattern1 = Some(random_pattern());
                self.source_name1 = Some("random".to_string());
                self.update_diffs();
            }
        });
    }

    fn add_body_contents(&self, body: TableBody<'_>) {
        let hex_grid_width = 16;

        let row_height = 18.0;
        let num_rows = 1 + std::cmp::max(self.diffs0.len(), self.diffs1.len()) / hex_grid_width;

        body.rows(row_height, num_rows, |mut row| {
            let row_index = row.index();

            let add_hex_row = |ui: &mut Ui, diffs: &Vec<HexCell>| {
                (0..hex_grid_width).for_each(|i| {
                    let cell = diffs.get(i + row_index * hex_grid_width);

                    match cell {
                        Some(&HexCell::Same {
                            value,
                            source_id: _,
                        }) => ui.label(RichText::new(format!("{value:02X}")).monospace()),
                        Some(&HexCell::Diff {
                            value,
                            source_id: _,
                        }) => ui.label(
                            RichText::new(format!("{value:02X}"))
                                .color(Color32::from_rgb(192, 64, 64))
                                .monospace(),
                        ),

                        Some(&HexCell::Blank) => ui.monospace("__"),
                        None => ui.monospace("xx"),
                    };
                });
            };

            let add_ascii_row = |ui: &mut Ui, diffs: &Vec<HexCell>| {
                (0..hex_grid_width).for_each(|i| {
                    let cell = diffs.get(i + row_index * hex_grid_width);

                    match cell {
                        Some(&HexCell::Same {
                            value,
                            source_id: _,
                        }) => ui.label(RichText::new(format!("{}", value as char)).monospace()),
                        Some(&HexCell::Diff {
                            value,
                            source_id: _,
                        }) => ui.label(
                            RichText::new(format!("{}", value as char))
                                .color(Color32::from_rgb(192, 64, 64))
                                .monospace(),
                        ),
                        Some(&HexCell::Blank) => ui.monospace("_"),
                        None => ui.monospace("x"),
                    };
                });
            };

            row.col(|ui| {
                ui.label(RichText::new(format!("{:08X}", row_index * hex_grid_width)).monospace());
            });
            row.col(|ui| add_hex_row(ui, &self.diffs0));
            row.col(|ui| add_ascii_row(ui, &self.diffs0));
            row.col(|ui| add_hex_row(ui, &self.diffs1));
            row.col(|ui| add_ascii_row(ui, &self.diffs1));
        });
    }
}

impl eframe::App for HexApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.input(|i| {
            if let Some(dropped_file) = i.raw.dropped_files.first() {
                if let Some(bytes) = &dropped_file.bytes {
                    match self.file_drop_target {
                        WhichFile::File0 => {
                            self.pattern0 = Some(bytes.to_vec());
                            self.source_name0 = Some(dropped_file.name.clone());
                        }
                        WhichFile::File1 => {
                            self.pattern1 = Some(bytes.to_vec());
                            self.source_name1 = Some(dropped_file.name.clone());
                        }
                    }
                    self.update_diffs();
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("hex diff test (egui UI)");

            TableBuilder::new(ui)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .striped(true)
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .column(Column::remainder())
                .header(20.0, |header| self.add_header_row(header))
                .body(|body| self.add_body_contents(body));
        });
    }
}

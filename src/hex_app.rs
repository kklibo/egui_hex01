use crate::diff::{self, HexCell};
use egui::Ui;
use egui_extras::{Column, TableBody, TableBuilder, TableRow};
use rand::Rng;

pub struct HexApp {
    source_name0: Option<String>,
    source_name1: Option<String>,
    pattern0: Option<Vec<u8>>,
    pattern1: Option<Vec<u8>>,
    diffs0: Vec<HexCell>,
    diffs1: Vec<HexCell>,
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
            if ui.button("randomize").clicked() {
                self.pattern0 = Some(random_pattern());
                self.source_name0 = Some("random".to_string());
                self.update_diffs();
            }
        });
        header.col(|ui| {
            ui.heading(self.source_name1.as_ref().unwrap_or(&no_pattern));
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

            let add_body_hex_row = |ui: &mut Ui, diffs: &Vec<HexCell>| {
                (0..hex_grid_width).for_each(|i| {
                    let cell = diffs.get(i + row_index * hex_grid_width);

                    match cell {
                        Some(&HexCell::Same { value, source_id }) => {
                            ui.label(format!("{value:02X}"))
                        }
                        Some(&HexCell::Diff { value, source_id }) => {
                            ui.label(format!("{value:02X}"))
                        }
                        Some(&HexCell::Blank) => ui.label("__"),
                        None => ui.label("xx"),
                    };
                });
            };

            row.col(|ui| {
                ui.label(format!("{:08X}", row_index * hex_grid_width));
            });
            row.col(|ui| add_body_hex_row(ui, &self.diffs0));
            row.col(|ui| add_body_hex_row(ui, &self.diffs1));
        });
    }
}

impl eframe::App for HexApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("hex diff test (egui UI)");

            TableBuilder::new(ui)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .striped(true)
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .column(Column::remainder())
                .header(20.0, |header| self.add_header_row(header))
                .body(|body| self.add_body_contents(body));
        });
    }
}

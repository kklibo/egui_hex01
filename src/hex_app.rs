use egui::Ui;
use egui_extras::{Column, TableBody, TableBuilder, TableRow};
use rand::Rng;

pub struct HexApp {
    source_name0: Option<String>,
    source_name1: Option<String>,
    pattern0: Option<Vec<u8>>,
    pattern1: Option<Vec<u8>>,
}

fn random_pattern() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..1000).map(|_| rng.gen_range(0..=255)).collect()
}

impl HexApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            source_name0: Some("zeroes0".to_string()),
            source_name1: Some("zeroes1".to_string()),
            pattern0: Some(vec![0; 1000]),
            pattern1: Some(vec![0; 1000]),
        }
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
            }
        });
        header.col(|ui| {
            ui.heading(self.source_name1.as_ref().unwrap_or(&no_pattern));
            if ui.button("randomize").clicked() {
                self.pattern1 = Some(random_pattern());
                self.source_name1 = Some("random".to_string());
            }
        });
    }

    fn add_body_contents(&self, body: TableBody<'_>) {
        let hex_grid_width = 16;

        let row_height = 18.0;
        let num_rows = 1 + std::cmp::max(
            self.pattern0.as_ref().map(Vec::len).unwrap_or_default(),
            self.pattern1.as_ref().map(Vec::len).unwrap_or_default(),
        ) / hex_grid_width;

        body.rows(row_height, num_rows, |mut row| {
            let row_index = row.index();

            let add_body_hex_row = |ui: &mut Ui, pattern: &Option<Vec<u8>>| {
                (0..hex_grid_width).for_each(|i| {
                    let s = pattern
                        .as_ref()
                        .and_then(|bytes| bytes.get(i + row_index * hex_grid_width))
                        .map(|&b| format!("{b:02X}"))
                        .unwrap_or_else(|| "__".to_string());

                    ui.label(s);
                });
            };

            row.col(|ui| {
                ui.label(format!("{:08X}", row_index * hex_grid_width));
            });
            row.col(|ui| add_body_hex_row(ui, &self.pattern0));
            row.col(|ui| add_body_hex_row(ui, &self.pattern1));
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

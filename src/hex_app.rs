use egui::Ui;
use egui_extras::{Column, TableBody, TableBuilder, TableRow};

struct Pattern {
    name: String,
    data: Vec<u8>,
}
pub struct HexApp {
    pattern0: Option<Pattern>,
    pattern1: Option<Pattern>,
}

impl HexApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            pattern0: Some(Pattern {
                name: "zeroes0".to_string(),
                data: vec![0; 1000],
            }),
            pattern1: Some(Pattern {
                name: "zeroes1".to_string(),
                data: vec![0; 1000],
            }),
        }
    }
}

fn add_header_row(mut header: TableRow<'_, '_>) {
    header.col(|ui| {
        ui.heading("addresses0");
    });
    header.col(|ui| {
        ui.heading("hex0");
    });
    header.col(|ui| {
        ui.heading("hex1");
    });
}

impl HexApp {
    fn add_body_contents(&self, body: TableBody<'_>) {
        let hex_grid_width = 16;

        let row_height = 18.0;
        let num_rows = 10_000;

        body.rows(row_height, num_rows, |mut row| {
            let row_index = row.index();

            let add_body_hex_row = |ui: &mut Ui, pattern: &Option<Pattern>| {
                (0..hex_grid_width).for_each(|i| {
                    let s = if let Some(pattern) = pattern {
                        if let Some(&b) = pattern.data.get(i + row_index * hex_grid_width) {
                            format!("{b:02X}")
                        } else {
                            "__".to_string()
                        }
                    } else {
                        "xx".to_string()
                    };

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
                .header(20.0, add_header_row)
                .body(|body| self.add_body_contents(body));
        });
    }
}

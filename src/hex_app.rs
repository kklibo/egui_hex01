use egui::style::ScrollStyle;
use egui::{Ui, Vec2};
use egui_extras::{Column, TableBody, TableBuilder};

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

fn display_address_column(egui_id: &str, ui: &mut Ui, rows: usize) {
    let hex_width = 16;
    egui::Grid::new(egui_id)
        .spacing(Vec2::new(-20f32, 0f32))
        .striped(true)
        .show(ui, |ui| {
            (0..rows).for_each(|c| {
                ui.label(format!("{:08X}", c * hex_width));
                ui.end_row();
            });
        });
}

fn display_hex_field(egui_id: &str, ui: &mut Ui, bytes: &[u8]) {
    let width = 16;
    egui::Grid::new(egui_id)
        .spacing(Vec2::new(-20f32, 0f32))
        .striped(true)
        .show(ui, |ui| {
            bytes.chunks(width).for_each(|row| {
                row.iter().for_each(|&b| {
                    ui.label(format!("{b:02X}"));
                });
                ui.end_row();
            });
        });
}

impl HexApp {
    fn add_body_contents(&self, mut body: TableBody<'_>) {
        let hex_grid_width = 16;

        let mut addresses0 = (0..).map(|c| format!("{:08X}", c * hex_grid_width));

        let mut pattern0 = if let Some(pattern) = &self.pattern0 {
            pattern.data.iter()
        } else {
            [].iter()
        }
        .map(|&b| format!("{b:02X}"));

        let row_height = 18.0;
        let num_rows = 10_000;
        body.rows(row_height, num_rows, |mut row| {
            let row_index = row.index();
            row.col(|ui| {
                ui.label(addresses0.next().expect("iterator should be infinite"));
            });
            row.col(|ui| {
                egui::Grid::new(format!("pattern0row{row_index}"))
                    .spacing(Vec2::new(-20f32, 0f32))
                    .striped(true)
                    .show(ui, |ui| {
                        (0..hex_grid_width).for_each(|_| {
                            let s = pattern0.next().unwrap_or_else(|| "__".to_string());
                            ui.label(s);
                        });
                        ui.end_row();
                    });
            });
        });
    }
}

impl eframe::App for HexApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("hex diff test (egui UI)");

            TableBuilder::new(ui)
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .column(Column::remainder())
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("addresses0");
                    });
                    header.col(|ui| {
                        ui.heading("hex0");
                    });
                    header.col(|ui| {
                        ui.heading("hex1");
                    });
                })
                .body(|body| self.add_body_contents(body));
        });
    }
}

use egui::style::ScrollStyle;
use egui::{Ui, Vec2};

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

impl eframe::App for HexApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("hex diff test (egui UI)");

            ui.spacing_mut().scroll = ScrollStyle::solid();
            egui::ScrollArea::vertical()
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
                .show(ui, |ui| {
                    ui.columns(2, |columns| {
                        columns[0].label("First column");

                        columns[0].columns(2, |columns| {
                            display_address_column(
                                "pattern0 addresses",
                                &mut columns[0],
                                1 + &self.pattern0.as_ref().unwrap().data.len() / 16,
                            );
                            display_hex_field(
                                "pattern0 hex",
                                &mut columns[1],
                                &self.pattern0.as_ref().unwrap().data,
                            );
                        });

                        columns[1].label("Second column");
                        display_hex_field(
                            "pattern1 hex",
                            &mut columns[1],
                            &self.pattern1.as_ref().unwrap().data,
                        );
                    });
                });
        });
    }
}

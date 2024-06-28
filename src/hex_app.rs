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
                //ui.label(addresses0.next().expect("iterator should be infinite"));
                ui.label(format!("{row_index}"));
            });
            row.col(|ui| {
                (0..hex_grid_width).for_each(|_| {
                    let s = pattern0.next().unwrap_or_else(|| "__".to_string());
                    ui.label(s);
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

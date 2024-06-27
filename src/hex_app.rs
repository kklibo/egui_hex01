use egui::style::ScrollStyle;

pub struct HexApp {}

impl Default for HexApp {
    fn default() -> Self {
        Self {}
    }
}

impl HexApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}
impl eframe::App for HexApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("hex diff test (egui UI)");

            ui.spacing_mut().scroll = ScrollStyle::solid();
            egui::ScrollArea::vertical()
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
                .show(ui, |ui| {
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                    ui.label("123132");
                });
        });
    }
}

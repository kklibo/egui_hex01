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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {}
}

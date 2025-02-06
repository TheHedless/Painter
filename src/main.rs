mod painter_db;
mod painter_io;
mod drawn_shape;

use eframe::egui;
use crate::drawn_shape::drawn_shape_mod::DrawingShapes;

fn main()-> Result<(), eframe::Error> {
    painter_db::painter_db::db_ping().expect("Ping failed");
    eframe::run_native(
        "Painter",
        eframe::NativeOptions::default(),
        Box::new(|_cc| {
            Ok(Box::<DrawingShapes>::default())
        }),
    )
}
impl eframe::App for DrawingShapes {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui_controls(ui);
            self.ui_canvas(ui);
            self.ui_io(ui);
        });
    }
}
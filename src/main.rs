mod painter_db;
mod painter_io;
mod drawn_shape;
mod gallery_view;

use eframe::egui;
use egui::Id;
use crate::drawn_shape::drawn_shape_mod::ShapeDrawWindow;

fn main() -> Result<(), eframe::Error> {
    //painter_db::painter_db::db_ping().expect("Ping failed");
    eframe::run_native(
        "Painter",
        eframe::NativeOptions::default(),
        Box::new(|_cc| {
            Ok(Box::<WindowState>::default())
        }),
    )
}

impl eframe::App for ShapeDrawWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui_controls(ui);
            self.ui_canvas(ui);
            self.ui_io(ui);
        });
    }
}

enum WindowState {
    DrawingWindow(ShapeDrawWindow),
    GalleryWindow(GalleryWindow),
}

impl Default for WindowState {
    fn default() -> Self {
        WindowState::GalleryWindow(GalleryWindow::default())
    }
}

impl eframe::App for WindowState {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top(Id::new("state")).show(ctx, |ui| {
            if ui.button("Switch").clicked() {
                *self = match self {
                    WindowState::DrawingWindow(_) => WindowState::GalleryWindow(GalleryWindow::default()),
                    WindowState::GalleryWindow(_) => WindowState::DrawingWindow(ShapeDrawWindow::default()),
                }
            }
        });
        match self {
            WindowState::DrawingWindow(w) => w.update(ctx, frame),
            WindowState::GalleryWindow(w) => w.update(ctx, frame),
        }
    }
}

#[derive(Default)]
struct GalleryWindow {

}

impl eframe::App for GalleryWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("CSÁÁÁ");
        });
    }
}
mod painter_db;
mod painter_io;
mod drawn_shape;
mod gallery_view;

use eframe::egui;
use egui::Id;
use crate::drawn_shape::drawn_shape_mod::DrawingShapes;
use crate::gallery_view::gallery::Gallery;

fn main() -> Result<(), eframe::Error> {
    painter_db::painter_db::db_ping().expect("Ping failed");
    eframe::run_native(
        "Painter",
        eframe::NativeOptions::default(),
        Box::new(|_cc| {
            Ok(Box::<WindowState>::default())
        }),
    )
}

enum WindowState {
    DrawingWindow(DrawingShapes),
    GalleryWindow(Gallery),
}

impl Default for WindowState {
    fn default() -> Self {
        Self::DrawingWindow(DrawingShapes::default())
    }
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

impl eframe::App for Gallery {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            Gallery::show(self, ui);
        });
    }
}

impl eframe::App for WindowState {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top(Id::new("state")).show(ctx, |ui| {
            if ui.button("Switch between Gallery and Drawing").clicked() {
                *self = match self {
                    WindowState::DrawingWindow(_) => WindowState::GalleryWindow(Gallery::default()),
                    WindowState::GalleryWindow(_) => WindowState::DrawingWindow(DrawingShapes::default()),
                }
            }
        });
        match self {
            WindowState::DrawingWindow(w) => w.update(ctx, frame),
            WindowState::GalleryWindow(w) => w.update(ctx, frame),
        }
    }
}
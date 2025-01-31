mod painter_db;

use chrono::Local;
use eframe::emath::{Pos2, Vec2};
use eframe::epaint::{Rect, Shape, Stroke};
use eframe::{egui, emath};
use egui::epaint::PathShape;
use egui::{Color32, Grid, Sense};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use pollster::FutureExt as _;

fn main() -> Result<(), eframe::Error> {
    painter_db::painter_db::db_ping().block_on().expect("Ping failed");
    eframe::run_native(
        "Painter",
        eframe::NativeOptions::default(),
        Box::new(|_cc| {
            Ok(Box::<DrawnShape>::default())
        }),
    )
}
#[derive(Debug, Serialize, Deserialize)]
struct DrawnShape {
    stroke: Stroke,
    node: Vec<Pos2>,
    fill: Color32,
    #[serde(skip)]
    filename: String,
    #[serde(skip)]
    io_status: String,
    author: String,
    creation_time: chrono::DateTime<Local>,
}
impl Default for DrawnShape {
    fn default() -> Self {
        Self {
            stroke: Stroke::new(1.0, Color32::from_rgb(100, 100, 100)),
            node: Vec::from([
                Pos2::new(100.0, 100.0),
                Pos2::new(100.0, 200.0),
                Pos2::new(50.0, 150.0)]),
            fill: Color32::from_rgb(50, 50, 50),
            filename: "".to_string(),
            io_status: "".to_string(),
            author: "".to_string(),
            creation_time: Local::now(),
        }
    }
}


impl DrawnShape {
    fn ui_controls(&mut self, ui: &mut egui::Ui) {
        //control options like line colors
        Grid::new("colors").spacing([5.0, 10.0]).show(ui, |ui| {
            ui.label("Fill colors");
            ui.color_edit_button_srgba(&mut self.fill);
            ui.end_row();
            ui.label("Line properties");
            ui.add(&mut self.stroke);
            ui.end_row();
            let add_element = ui.button("Add Node");
            let remove_element = ui.button("Remove Node");
            if add_element.clicked() {
                self.node.push(Pos2::new(50.0, 100.0));
            }
            if remove_element.clicked() {
                self.node.pop();
            }
        });
    }
    fn ui_canvas(&mut self, ui: &mut egui::Ui) {
        // define our canvas
        let (response, painter) =
            ui.allocate_painter(Vec2::new(300.0, 300.0), Sense::hover());
        // normalise coords to canvas instead of the window
        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.size()),
            response.rect,
        );
        let node_centers: Vec<Pos2> = self
            .node
            .iter()
            .map(|point| {
                to_screen.transform_pos(to_screen.from().clamp(*point))
            })
            .collect();
        // dragable circles
        // using node_centers would save us point_in_screen having to be calculated twice
        // however drag logic breaks if it is used
        for (i, node) in self.node.iter_mut().enumerate() {
            let size = Vec2::splat(8.0);
            //gets the points as uniques
            let point_in_screen = to_screen.transform_pos(*node);
            let point_rect = Rect::from_center_size(point_in_screen, size);
            let point_id = response.id.with(i);
            //drag logic
            let point_response = ui.interact(point_rect, point_id, Sense::drag());
            *node = to_screen.from().clamp(*node + point_response.drag_delta());
        }
        let node_circles: Vec<Shape> = self
            .node
            .iter()
            .map(|point| {
                Shape::circle_stroke(
                    to_screen.transform_pos(*point),
                    4.0,
                    Stroke::new(1.0, Color32::from_rgb(255, 255, 255)))
            })
            .collect();
        painter.add(PathShape::convex_polygon(node_centers, self.fill, self.stroke));
        // convex_polygon tries to fill from the origin node [0]
        // concave shapes are not fully supported because of this
        painter.extend(node_circles);
    }
    fn ui_io(&mut self, ui: &mut egui::Ui) {
        //IO buttons
        Grid::new("save").spacing([5.0, 10.0]).show(ui, |ui| {
            ui.label("Shape name:");
            ui.text_edit_singleline(&mut self.filename);
            ui.end_row();

            let save_button = ui.button("Save");
            let load_button = ui.button("Load");

            let named;
            if self.filename.is_empty() {
                named = false
            } else {
                named = true
            }

            if !named { self.io_status = "".to_string(); }
            if save_button.clicked() && named {
                let file = File::create(self.filename.clone() + ".json").unwrap();
                let mut writer = BufWriter::new(file);
                serde_json::to_writer(&mut writer, &self).expect("write to file failed");
                writer.flush().expect("flush failed");
                self.io_status = "Saved successfully".to_string();
            }
            if load_button.clicked() && named {
                if File::open(self.filename.clone() + ".json").is_err() {
                    self.io_status = "Could not open file".to_string();
                } else {
                    let file = File::open(self.filename.clone() + ".json").unwrap();
                    let reader = BufReader::new(file);
                    *self = serde_json::from_reader(reader).unwrap();
                    self.io_status = "Load successfully".to_string();
                }
            }
            ui.end_row();
            ui.label(&self.io_status);
            ui.end_row();
            ui.label("Author:");
            ui.text_edit_singleline(&mut self.author);
        });
    }
}

impl eframe::App for DrawnShape {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui_controls(ui);
            self.ui_canvas(ui);
            self.ui_io(ui);
        });
    }
}
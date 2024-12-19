use std::fs::File;
use std::io::{BufWriter, Write};
use eframe::{egui, emath};
use eframe::emath::{Pos2, Vec2};
use eframe::epaint::{Rect, Shape, Stroke};
use egui::{ Color32, Grid, Sense};
use egui::epaint::PathShape;
use serde:: {Serialize, Deserialize};

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Painter",
        eframe::NativeOptions::default(),
        Box::new(|_cc| {
            Ok(Box::<MyApp>::default())
        }),
    )
}
#[derive(Debug, Serialize, Deserialize)]
struct MyApp {
    stroke: Stroke,
    node: Vec<Pos2>,
    fill: Color32,
    point_count: usize,
    filename: String,
    io_status: String
}
impl Default for MyApp {
    fn default() -> Self {
        Self {
            stroke: Stroke::new(1.0, Color32::from_rgb(100, 100, 100)),
            node: Vec::from([Pos2::new(100.0, 100.0), Pos2::new(100.0, 200.0), Pos2::new(50.0, 150.0)]),
            fill: Color32::from_rgb(50, 50, 50),
            point_count: 3,
            filename: "".to_string(),
            io_status: "".to_string(),
        }
    }
}


impl MyApp {
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
                self.point_count += 1
            }
            if remove_element.clicked() {
                self.node.pop();
                self.point_count -= 1
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
            .to_vec()
            .iter_mut()
            .enumerate()
            .take(self.point_count)
            .map(|(i, point)| {
                let mut point_in_screen = to_screen.transform_pos(*point);
                *point = to_screen.from().clamp(*point);
                point_in_screen = to_screen.transform_pos(*point);
                point_in_screen
            })
            .collect();
        // dragable circles
        // using node_centers would save us point_in_screen having to be calculated twice
        // however drag logic breaks if it is used
        let node_circles: Vec<Shape> = self
            .node
            .to_vec()
            .iter_mut()
            .enumerate()
            .take(self.point_count)
            .map(|(i, point)| {
                let size = Vec2::splat(8.0);
                //gets the points as uniques
                let mut point_in_screen = to_screen.transform_pos(*point);
                let point_rect = Rect::from_center_size(point_in_screen, size);
                let point_id = response.id.with(i);
                //drag logic
                let point_response = ui.interact(point_rect, point_id, Sense::drag());
                *point += point_response.drag_delta();
                *point = to_screen.from().clamp(*point);
                point_in_screen = to_screen.transform_pos(*point);
                self.node[i] = *point;

                let stroke = ui.style().interact(&point_response).fg_stroke;
                Shape::circle_stroke(point_in_screen, 4.0, stroke)
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
            let mut named = false;
            if self.filename == "" {
                named = false
            } else {
                named = true
            }
            if save_button.clicked() && named {
                //add save feature
                //save node Vec, fill and line color
                let file = File::create(self.filename.clone()).unwrap();
                let mut writer= BufWriter::new(file);
                serde_json::to_writer(&writer, &self).expect("write to file failed");
                writer.flush().expect("flush failed");
                self.io_status="Saved successfully".to_string();

            }
            if load_button.clicked() && named {
                //add load feature
                self.io_status="Load successfully".to_string();
            }
            ui.end_row();
            ui.label(&self.io_status)
        });
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui_controls(ui);
            self.ui_canvas(ui);
            self.ui_io(ui);
        });
    }
}
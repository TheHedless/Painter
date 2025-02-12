pub(crate) mod drawn_shape_mod {
    use std::fmt::Debug;
    use std::fs::File;
    use std::io::{BufReader, BufWriter, Write};
    use chrono::Local;
    use eframe::emath;
    use serde::{Deserialize, Serialize};
    use eframe::emath::{Pos2, Vec2};
    use eframe::epaint::{Rect, Shape, Stroke, PathShape};
    use egui::{Color32, Grid, Sense};
    use crate::gallery_view::gallery::Gallery;
    use crate::painter_io::file_io::IO;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct DrawingShapes {
        pub stroke: Stroke,
        pub node: Vec<Pos2>,
        pub fill: Color32,
        pub _id: String, //is the filename, but is chosen to work in the database as the id
        #[serde(skip)]
        pub io_status: String,
        pub author: String,
        pub creation_time: String,
        #[serde(skip)]
        pub window: Vec<Gallery>,
    }
    impl Default for DrawingShapes {
        fn default() -> Self {
            Self {
                stroke: Stroke::new(1.0, Color32::from_rgb(100, 100, 100)),
                node: Vec::from([
                    Pos2::new(100.0, 100.0),
                    Pos2::new(100.0, 200.0),
                    Pos2::new(50.0, 150.0)]),
                fill: Color32::from_rgb(50, 50, 50),
                _id: "".to_string(),
                io_status: "".to_string(),
                author: "".to_string(),
                creation_time: Local::now().format("%Y.%m.%d").to_string(),
                window: vec![],
            }
        }
    }
    impl Clone for DrawingShapes {
        fn clone(&self) -> Self {
            DrawingShapes {
                stroke: self.stroke,
                node: self.node.clone(),
                fill: self.fill,
                _id: self._id.clone(),
                io_status: self.io_status.clone(),
                author: self.author.clone(),
                creation_time: self.creation_time.clone(),
                window: self.window.clone(),
            }
        }
    }
    impl DrawingShapes {
        pub(crate) fn ui_controls(&mut self, ui: &mut egui::Ui) {
            //control options like line colors
            Grid::new("colors").spacing([5.0, 10.0]).show(ui, |ui| {
                ui.label("Fill colors");
                ui.color_edit_button_srgba(&mut self.fill);
                ui.end_row();
                ui.label("Line properties");
                ui.add(&mut self.stroke);
            });
            ui.horizontal(|ui| {
                if ui.button("Add Node").clicked() { //add_element.clicked() {
                    self.node.push(Pos2::new(50.0, 100.0));
                }
                if ui.button("Remove Node").clicked() {
                    self.node.pop();
                }
                if ui.button("Open/Close Gallery").clicked() {
                    if self.window.is_empty() {
                        self.window.push(Gallery::default())
                    } else {
                        self.window.pop();
                    }
                }
                for window in self.window.iter() {
                    window.show(ui)
                }
            });
        }
        pub(crate) fn ui_canvas(&mut self, ui: &mut egui::Ui) {
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
            // draggable circles
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
        pub(crate) fn ui_io(&mut self, ui: &mut egui::Ui) {
            //IO buttons

            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self._id)
                        .hint_text("Shape name")
                        .desired_width(100.)
                );
                ui.add(
                    egui::TextEdit::singleline(&mut self.author)
                        .hint_text("Author name")
                        .desired_width(100.)
                );
            });
            ui.horizontal(|ui| {
                let named;
                if self._id.is_empty() {
                    named = false
                } else {
                    named = true
                }
                if !named { self.io_status = "".to_string(); }

                if ui.button("Save").clicked() && named {
                    IO::save(self);
                }
                if ui.button("Load").clicked() && named {
                    IO::load(self);
                }
            });
            ui.label(&self.io_status);
        }
    }
    impl IO for DrawingShapes {
        fn save(&mut self) {
            if self.author.is_empty() { self.author = "N/A".to_string() }
            let file = File::create(self._id.clone() + ".json").unwrap();
            let mut writer = BufWriter::new(file);
            serde_json::to_writer(&mut writer, &self).expect("write to file failed");
            writer.flush().expect("flush failed");
            self.io_status = "Saved successfully".to_string();
        }
        fn load(&mut self) {
            if let Ok(file) = File::open(self._id.clone() + ".json") {
                let reader = BufReader::new(file);
                *self = serde_json::from_reader(reader).unwrap();
                self.io_status = "Load successfully".to_string();
            }
        }
    }
}
use eframe::emath::{Pos2, Vec2};
use eframe::epaint::{Rect, Shape, Stroke};
use eframe::{egui, emath};
use egui::epaint::PathShape;
use egui::{Color32, Grid, Sense};
use std::fs::File;
use std::io::{BufWriter, Read, Write};

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Painter",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Debug)]
struct MyApp {
    stroke: Stroke,
    node: Vec<Pos2>,
    fill: Color32,
    point_count: usize,
    filename: String,
    io_status: String,
}

/*
    Saving and loading: Convert the struct into a binary format and write it to a file.
    The binary:
    8 bytes for the point count
    4 bytes for the fill color
    4 bytes for the stroke color
    4 bytes for the stroke width
    8 bytes for each node
    - 4 bytes for the x coordinate
    - 4 bytes for the y coordinate
*/
impl MyApp {
    pub fn to_binary(&self) -> Vec<u8> {
        let mut binary: Vec<u8> = Vec::new();

        binary.extend(self.point_count.to_be_bytes());

        let mut fill = self.fill.to_hex().to_string();
        fill.remove(0);
        binary.extend(u32::from_str_radix(&fill, 16).unwrap().to_be_bytes());

        let mut stroke_color = self.stroke.color.to_hex().to_string();
        stroke_color.remove(0);
        binary.extend(
            u32::from_str_radix(&stroke_color, 16)
                .unwrap()
                .to_be_bytes(),
        );

        binary.extend(self.stroke.width.to_be_bytes());

        for node in &self.node {
            binary.extend(node.x.to_be_bytes());
            binary.extend(node.y.to_be_bytes());
        }

        binary
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            stroke: Stroke::new(1.0, Color32::from_rgb(100, 100, 100)),
            node: Vec::from([
                Pos2::new(100.0, 100.0),
                Pos2::new(100.0, 200.0),
                Pos2::new(50.0, 150.0),
            ]),
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
        let (response, painter) = ui.allocate_painter(Vec2::new(300.0, 300.0), Sense::hover());
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

        painter.add(PathShape::convex_polygon(
            node_centers,
            self.fill,
            self.stroke,
        ));
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
                let binary = self.to_binary();
                let file = File::create(self.filename.clone() + ".pshp").unwrap();
                let mut writer = BufWriter::new(file);

                // write the binary to the file
                let _ = writer.write_all(&binary);
                if let Err(_e) = writer.flush() {
                    self.io_status = "Save failed".to_string();
                } else {
                    self.io_status = "Saved successfully".to_string();
                }
            }
            if load_button.clicked() && named {
                // read the file
                let file = File::open(self.filename.clone() + ".pshp").unwrap();

                // Check if the file exists
                if !file.metadata().is_ok() {
                    self.io_status = "File does not exist".to_string();
                    return;
                }

                let mut reader = std::io::BufReader::new(file);
                let mut binary: Vec<u8> = Vec::new();
                let _ = reader.read_to_end(&mut binary);

                let point_count;
                let fill;
                let stroke;
                let mut node = Vec::new();
                let mut i = 0;

                let point_count_bytes = &binary[0..8];
                let fill_bytes = &binary[8..12];
                let stroke_bytes = &binary[12..16];
                let stroke_width_bytes = &binary[16..20];
                let node_bytes = &binary[20..];

                point_count = usize::from_be_bytes([
                    point_count_bytes[0],
                    point_count_bytes[1],
                    point_count_bytes[2],
                    point_count_bytes[3],
                    point_count_bytes[4],
                    point_count_bytes[5],
                    point_count_bytes[6],
                    point_count_bytes[7],
                ]);
                fill = Color32::from_rgba_premultiplied(
                    fill_bytes[0],
                    fill_bytes[1],
                    fill_bytes[2],
                    fill_bytes[3],
                );
                stroke = Stroke::new(
                    f32::from_be_bytes([
                        stroke_width_bytes[0],
                        stroke_width_bytes[1],
                        stroke_width_bytes[2],
                        stroke_width_bytes[3],
                    ]),
                    Color32::from_rgba_premultiplied(
                        stroke_bytes[0],
                        stroke_bytes[1],
                        stroke_bytes[2],
                        stroke_bytes[3],
                    ),
                );

                while i < node_bytes.len() {
                    let x = f32::from_be_bytes([
                        node_bytes[i],
                        node_bytes[i + 1],
                        node_bytes[i + 2],
                        node_bytes[i + 3],
                    ]);
                    let y = f32::from_be_bytes([
                        node_bytes[i + 4],
                        node_bytes[i + 5],
                        node_bytes[i + 6],
                        node_bytes[i + 7],
                    ]);
                    node.push(Pos2::new(x, y));
                    i += 8;
                }

                self.point_count = point_count;
                self.fill = fill;
                self.stroke = stroke;
                self.node = node;

                self.io_status = "Loaded successfully".to_string();
            }
            ui.end_row();
            ui.label(&self.io_status);
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

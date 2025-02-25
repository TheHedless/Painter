pub mod gallery {
    use egui::Grid;
    use crate::drawn_shape::drawn_shape_mod::DrawingShapes;
    use crate::painter_db::painter_db::db_query;

    #[derive(Debug)]
    pub struct Gallery {
        pub(crate) drawing_vec: Vec<DrawingShapes>,
        pub(crate) query_settings: Vec<String>,
        painter_dim: f32,
    }

    impl Default for Gallery {
        fn default() -> Self {
            Self {
                drawing_vec: Vec::new(),
                query_settings: vec!["".to_string(), "".to_string(), "".to_string()],
                painter_dim: 150.0,
            }
        }
    }
    impl Clone for Gallery {
        fn clone(&self) -> Self {
            Self {
                drawing_vec: self.drawing_vec.clone(),
                query_settings: self.query_settings.clone(),
                painter_dim: self.painter_dim.clone(),
            }
        }
    }
    // reminder for query_settings
    //      query_settings[1] _id: String
    //      query_settings[2] author: String
    //      query_settings[3] creation_time: String

    impl Gallery {
        pub fn show(&mut self, ui: &mut egui::Ui) {
            Grid::new("gallery_scenes").spacing([5.0, 10.0]).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.query_settings[0])
                            .hint_text("Filename")
                            .desired_width(100.)
                    );
                    ui.add(
                        egui::TextEdit::singleline(&mut self.query_settings[1])
                            .hint_text("Author name")
                            .desired_width(100.)
                    );
                    ui.add(
                        egui::TextEdit::singleline(&mut self.query_settings[2])
                            .hint_text("Creation date YYYY.mm.dd format")
                            .desired_width(200.)
                    );
                    if ui.button("Query").clicked() {
                        self.drawing_vec =
                            db_query(self.query_settings[0].clone(),
                                     self.query_settings[1].clone(),
                                     self.query_settings[2].clone());
                        /*.iter_mut()
                        .map(|item|
                                 item.node=item
                                    .node
                                    .iter_mut()
                                    .map(|coord| *coord * self.painter_dim)
                                    .collect::<Vec<_>>())
                        .collect::<Vec<_>>();*/ //this is left here for later analysis of why it's not working this way
                        for item in self.drawing_vec.iter_mut() {
                            item.node = item
                                .node
                                .iter_mut()
                                .map(|coord| *coord * self.painter_dim / 300.0)
                                .collect::<Vec<_>>();
                        }
                    }
                })
            });
            ui.horizontal_wrapped(|ui| {
                for item in self.drawing_vec.iter_mut() {
                    DrawingShapes::ui_canvas(&mut item.clone(), ui, self.painter_dim)
                }
            });
        }
    }
}
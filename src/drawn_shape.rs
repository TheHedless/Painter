pub(crate) mod DrawnShape {
    use chrono::Local;
    use eframe::emath::Pos2;
    use eframe::epaint::Stroke;
    use eframe::egui;
    use egui::Color32;
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DrawnShape {
        pub stroke: Stroke,
        pub node: Vec<Pos2>,
        pub fill: Color32,
        pub filename: String,
        #[serde(skip)]
        pub io_status: String,
        pub author: String,
        pub creation_time: chrono::DateTime<Local>,
        pub _id: String,
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
                _id: "".to_string(),
            }
        }
    }
}
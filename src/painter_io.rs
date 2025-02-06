pub mod file_io {
    use crate::drawn_shape;
    use std::fs::File;
    use std::io::{BufReader, BufWriter, Write};
    pub fn save(shape: &mut drawn_shape::drawn_shape_mod::DrawingShapes) -> String {
        let file = File::create(shape._id.clone() + ".json").unwrap();
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &shape).expect("write to file failed");
        writer.flush().expect("flush failed");
        "Saved successfully".to_string()
    }
    pub fn load(mut shape: drawn_shape::drawn_shape_mod::DrawingShapes) -> drawn_shape::drawn_shape_mod::DrawingShapes {
        if File::open(shape._id.clone() + ".json").is_err() {
            shape
        } else {
            let file = File::open(shape._id.clone() + ".json").unwrap();
            let reader = BufReader::new(file);
            shape = serde_json::from_reader(reader).unwrap();
            shape.io_status = "Load successfully".to_string();
            shape
        }
    }
}
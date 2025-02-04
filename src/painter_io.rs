pub mod file_io {
    use crate::drawn_shape;
    use std::fs::File;
    use std::io::{BufReader, BufWriter, Write};


    pub fn save(shape: &mut drawn_shape::DrawnShape::DrawnShape) -> String {
        shape._id = format!("{}_{}", shape.author.clone(), shape.creation_time);
        let file = File::create(shape.filename.clone() + ".json").unwrap();
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &shape).expect("write to file failed");
        writer.flush().expect("flush failed");
        "Saved successfully".to_string()
    }
    pub fn load(mut shape: drawn_shape::DrawnShape::DrawnShape) -> drawn_shape::DrawnShape::DrawnShape {
        if File::open(shape.filename.clone() + ".json").is_err() {
            shape
        } else {
            let file = File::open(shape.filename.clone() + ".json").unwrap();
            let reader = BufReader::new(file);
            shape = serde_json::from_reader(reader).unwrap();
            shape.io_status = "Load successfully".to_string();
            shape
        }
    }
}
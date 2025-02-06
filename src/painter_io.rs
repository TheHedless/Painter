pub mod file_io {
    pub trait IO {
        fn save(&mut self);
        fn load(&mut self);
    }
}
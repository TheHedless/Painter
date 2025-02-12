pub mod gallery {
    use egui::Window;

    #[derive(Debug)]
    pub struct Gallery {
        pub(crate) is_open: bool,
    }

    impl Gallery {
        pub fn show(&mut self, ui: &mut egui::Ui) {
            Window::new("Gallery").open(&mut self.is_open).show(ui.ctx(), |ui| {
                ui.label("Hello World!");
            });
        }
    }

    impl Default for Gallery {
        fn default() -> Self {
            Self {
                is_open: true
            }
        }
    }
    impl Clone for Gallery {
        fn clone(&self) -> Self {
            Self {
                is_open: self.is_open
            }
        }
    }
}
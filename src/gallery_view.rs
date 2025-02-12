pub mod gallery {
    use egui::Window;

    #[derive(Debug)]
    pub struct Gallery {
        pub(crate) is_open: bool,
    }

    impl Gallery {
        pub fn show(&self, ui: &mut egui::Ui) {
            Window::new("Gallery").show(ui.ctx(), |ui| {
                ui.label("Hello World!");
            });
        }
    }

    impl Default for Gallery {
        fn default() -> Self {
            Self {
                is_open: false
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
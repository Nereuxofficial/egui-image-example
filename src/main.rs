use eframe::egui;
use egui_extras::RetainedImage;

fn main() {
    // We can use the options to set things like resolution
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Cool Image Viewer",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    );
}

#[derive(Default)]
struct MyApp {
    retained_image: Option<RetainedImage>,
}

impl eframe::App for MyApp {
    /// The update function is called on every frame.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Every UI Element has to be part of a Panel so egui knows where to draw it.
        egui::CentralPanel::default().show(ctx, |ui| {
            // We can edit `ui` to add UI Elements to the Panel by calling functions on it.
            match &self.retained_image {
                Some(image) => {
                    // We add an image to the panel with max size of 500*500 Pixels
                    image.show_max_size(ui, egui::vec2(500.0, 500.0));
                }
                None => {
                    ui.label("No image loaded");
                }
            }
            // Here we add a button that Loads an image if clicked into self.retained_image
            if ui.button("Load Image").clicked() {
                // Photo by Ilia Bronskiy on Unsplash
                self.retained_image = Some(
                    RetainedImage::from_image_bytes(
                        "Bild",
                        include_bytes!("ilia-bronskiy-on-unsplash.jpg"),
                    )
                    .unwrap(),
                );
            }
        });
    }
}

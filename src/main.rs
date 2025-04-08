use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "VisualTho",
        eframe::NativeOptions::default(),
        Box::new(
            |_cc| std::result::Result::Ok(
                Box::new(MyApp::default())
            )
        ),
    )
}

#[derive(Default)]
struct MyApp {
    enabled: bool,
    code: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Teste de texto");

            ui.checkbox(
                &mut self.enabled,
                "Enable"
            );

            if self.enabled {
                ui.text_edit_multiline(&mut self.code);

                ui.button("Submit").clicked().then(|| {
                    println!("Clicked");
                    self.code = String::from("");
                });
            }
        });
    }
}

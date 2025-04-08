use eframe::egui;

#[derive(Default)]
pub struct Window {
    enabled: bool,
    code: String,

    title: String,
}

impl Window {
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_title(&mut self, title : &str) {
        self.title = String::from(title);
    }
}

impl eframe::App for Window {
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

pub fn run_window(window: Window) -> Result<(), eframe::Error> {
    eframe::run_native(
        &window.title,
        eframe::NativeOptions::default(),
        Box::new(
            |_cc| std::result::Result::Ok(
                Box::new(Window::default())
            )
        ),
    )
}
use interpreter::{interpreter::Interpreter, languages::visualg::VisuAlg};

mod app;
mod interpreter;

fn main() {
    if atty::is(atty::Stream::Stdin) {
        let lang = VisuAlg {};

        let mut int = Interpreter::from_file(String::from("tests/scripts/visualg/script-4.alg"));

        int.interpret(Box::new(lang));

        return;
    }

    let mut window = app::window::Window::default();
    window.set_title("VisualTho");

    let _ = app::window::run_window(window);
}

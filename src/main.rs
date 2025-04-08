use interpreter::{langs::visualg::visual_g, language::Language, rules::{TokenRule, TokenRuleItem}, token::Token};

mod app;
mod interpreter;

fn main() {
    if atty::is(atty::Stream::Stdin) {
        println!("terminal");

        let mut inter = interpreter::interpreter::Interpreter
            ::from_file("tests/scripts/visualg/script-1.alg");

        inter.set_language(visual_g());

        inter.tokenize();

        return;
    }

    let mut window = app::window::Window::default();
    window.set_title("VisualTho");

    let _ = app::window::run_window(window);
}

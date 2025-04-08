use interpreter::token::{Token, TokenRule, TokenTypes};

mod app;
mod interpreter;

fn main() {
    if atty::is(atty::Stream::Stdin) {
        println!("terminal");

        let mut inter = interpreter::interpreter::Interpreter::from_file("tests/scripts/visualg/script-1.alg");

        inter.set_rules(vec![
            TokenRule::new(vec![
                Token::Keyword(String::from("inicio"))
            ])
        ]);

        inter.tokenize();

        println!("{}", inter.get_code());

        return;
    }

    let mut window = app::window::Window::default();
    window.set_title("VisualTho");

    let _ = app::window::run_window(window);
}

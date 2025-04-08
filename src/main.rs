mod app;
mod interpreter;

fn main() {
    if atty::is(atty::Stream::Stdin) {
        println!("terminal");

        return;
    }

    let mut window = app::window::Window::default();
    window.set_title("VisualTho");

    app::window::run_window(window);
}

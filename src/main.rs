use rudit::App;

fn main() {
    ratatui::run(|terminal| App::default().run(terminal))
}

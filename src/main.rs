mod application;
mod window;

use application::app::Application;

fn main() {
    Application::new("Safe Windows App").run();
}

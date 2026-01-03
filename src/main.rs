mod application;
mod platform;
mod window;

use application::app::Application;

fn main() {
    Application::new("Echo").run();
}

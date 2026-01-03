use crate::platform::set_menu_item;
use crate::window::Window;

pub struct Application<'a> {
    name: &'a str,
    window: Window<'a>,
}

impl<'a> Application<'a> {
    pub fn new(name: &'a str) -> Self {
        Application {
            name,
            window: Window::new(name),
        }
    }

    pub fn run(&self) {
        set_menu_item();
        self.window.show();
        println!("testing");
        // Platform Setup
    }
}

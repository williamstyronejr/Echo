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
        self.window.show();
    }
}

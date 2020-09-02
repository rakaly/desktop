#[cfg(feature = "gui")]
mod gui;

#[cfg(not(feature = "gui"))]
pub fn run() {
    panic!("gui has not been compiled in");
}

#[cfg(feature = "gui")]
pub fn run() {
    gui::run();
}

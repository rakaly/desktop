#[cfg(target_os = "windows")]
mod windows;

#[cfg(not(target_os = "windows"))]
pub fn run_service() {
    panic!("unable to run as a service");
}

#[cfg(target_os = "windows")]
pub fn run_service() {
    windows::run()
}

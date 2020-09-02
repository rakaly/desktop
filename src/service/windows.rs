use std::ffi::OsString;
use windows_service::service_dispatcher;

windows_service::define_windows_service!(ffi_service_main, my_service_main);

fn my_service_main(_arguments: Vec<OsString>) {
    // The entry point where execution will start on a background thread after a call to
    // `service_dispatcher::start` from `main`.
}

pub fn run() {
    service_dispatcher::start("myservice", ffi_service_main).unwrap();
}

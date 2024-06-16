use web_sys::window;

pub use web_sys::console;

pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

pub fn search_params() -> Option<String> {
    window()
        .and_then(|w| w.location().search().ok())
        .map(|s| s.trim_start_matches('?').to_owned())
}

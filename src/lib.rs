#[macro_use]
extern crate sciter;

mod load_handler;

struct EventHandler;

impl EventHandler {

}

impl sciter::EventHandler for EventHandler {

    dispatch_script_call! {
    }

}

pub fn show() -> bool {
    let resources = include_bytes!("resources.rc");
    let handler = load_handler::LoadHandler::new(resources);

    let mut settings_window = sciter::window::Builder::main_window()
        .fixed()
        .with_title()
        .create();

    settings_window.sciter_handler(handler);
    settings_window.event_handler(EventHandler);
    settings_window.load_file("this://app/html/index.htm");
    settings_window.run_app();
    true
}

#[cfg(test)]
mod tests {

    use show;

    #[test]
    fn show_settings_window_test() {
        assert!(show());
    }

}

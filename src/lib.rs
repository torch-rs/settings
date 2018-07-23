#[macro_use]
extern crate sciter;

mod load_handler;

use sciter::Value;

struct EventHandler;

impl EventHandler {

    fn get_os(&self) -> String {
        if cfg!(target_os = "windows") {
            String::from("Windows")
        } else if cfg!(target_os = "macos") {
            String::from("Mac OS")
        } else if cfg!(target_os = "linux") {
            String::from("Linux")
        } else {
            String::from("Other")
        }
    }

    fn save_keybindings_config(&self, config: sciter::Value) -> bool {
        println!("{}", config.get_item("show-window"));
        println!("{}", config.get_item("hide-window"));
        println!("{}", config.get_item("previous-option"));
        println!("{}", config.get_item("next-option"));
        println!("{}", config.get_item("execute-primary-action"));
        println!("{}", config.get_item("execute-secondary-action"));
        true
    }

}

impl sciter::EventHandler for EventHandler {


    dispatch_script_call! {
        fn get_os();
        fn save_keybindings_config(Value);
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

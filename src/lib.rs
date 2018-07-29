extern crate config;
#[macro_use]
extern crate sciter;

mod load_handler;

use config::Config;
use config::keybindings_config;
use sciter::Value;
use std::collections::HashMap;

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

    fn get_keybindings_config(&self) -> Value {
        let mut config_to_send: Value = Value::map();
        let config = keybindings_config::KeybindingsConfig::new("keybindings.yaml");
        config_to_send.set_item("close-window", config.get("close-window").unwrap());
        config_to_send.set_item("previous-option", config.get("previous-option").unwrap());
        config_to_send.set_item("next-option", config.get("next-option").unwrap());
        config_to_send.set_item("execute-primary-action", config.get("execute-primary-action").unwrap());
        config_to_send.set_item("execute-secondary-action",
                                config.get("execute-secondary-action").unwrap());
        config_to_send
    }

    fn save_keybindings_config(&self, new_config: sciter::Value) -> bool {
        let mut config_data = HashMap::new();
        config_data.insert(String::from("close-window"), new_config.get_item("close-window").as_string().unwrap());
        config_data.insert(String::from("previous-option"), new_config.get_item("previous-option").as_string().unwrap());
        config_data.insert(String::from("next-option"), new_config.get_item("next-option").as_string().unwrap());
        config_data.insert(String::from("execute-primary-action"),
                           new_config.get_item("execute-primary-action").as_string().unwrap());
        config_data.insert(String::from("execute-secondary-action"),
                           new_config.get_item("execute-secondary-action").as_string().unwrap());

        let mut config = keybindings_config::KeybindingsConfig::new("keybindings.yaml");
        config.set(config_data);
        config.save().is_ok()
    }

}

impl sciter::EventHandler for EventHandler {


    dispatch_script_call! {
        fn get_os();
        fn get_keybindings_config();
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

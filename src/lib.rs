use gio::{prelude::SettingsExtManual, traits::SettingsExt, Settings};

pub struct InputManager {
    handler: Settings,
}

impl InputManager {
    pub fn new() -> InputManager {
        let handler = Settings::new("org.gnome.desktop.peripherals.touchpad");
        InputManager { handler }
    }

    pub fn list(&self) {
        let schema = self.handler.settings_schema().unwrap();
        println!("{:?}", schema.list_keys());
    }

    pub fn get_vals(&self) {
        let val: f64 = self.handler.get("speed");
        println!("speed: {val}");
    }

    pub fn check_change(&self) {
        self.handler.connect_changed(None, |_, key| {
            println!("{key} changed");
        });
    }
}

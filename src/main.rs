use gio::prelude::*;
use gio::{Application, ApplicationFlags};
use gsettings_demo_rs::InputManager;

fn main() {
    let app = Application::new(None, ApplicationFlags::IS_SERVICE);
    println!("Starting app");
    let manager = InputManager::new();
    manager.list();
    manager.get_vals();
    manager.check_change();
    app.run();
}

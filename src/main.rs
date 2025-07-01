mod ui;
mod file_operations;
mod shell_operations;

use gtk4::Application;
use gtk4::prelude::*;
use crate::ui::build_ui;

fn main() {
    let app = Application::new(Some("org.lucajoshuaweiss.ShellFlux"), Default::default());

    app.connect_activate(|app| {
        build_ui(app);
    });

    app.run();
}

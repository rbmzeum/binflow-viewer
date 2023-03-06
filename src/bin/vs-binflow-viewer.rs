use gtk4::{
    Application,
    gio,
    prelude::*,
};

use binflow::app::binflowviewer::window::BViewerWindow;

fn main() {
    // Register and include resources
    gio::resources_register_include!("resources.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(binflow::app::binflowviewer::APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = BViewerWindow::new(app);

    window.setup_menubar();
    window.setup_accels_for_actions();

    // Present window
    window.present();
}
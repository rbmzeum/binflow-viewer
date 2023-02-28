use gtk4::{
    Application,
    prelude::*,
};

use binflow::app::binflowviewer::window::BViewerWindow;

const APP_ID: &str = "net.binflow.viewer";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = BViewerWindow::new(app);

    // // Present window
    window.present();
}
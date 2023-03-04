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

    // let menubuilder =
    //     gtk4::Builder::from_resource("/vs/binflow/viewer/data/resources/ui/menu.ui");
    // let menubar: gio::MenuModel = menubuilder
    //     .object("main-menu")
    //     .expect("Could not get object 'main-menu' from builder.");
    // app.set_menubar(Some(&menubar));

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = BViewerWindow::new(app);

    // Present window
    window.present();
}
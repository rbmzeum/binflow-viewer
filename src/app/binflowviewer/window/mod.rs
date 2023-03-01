pub mod imp;

use gtk4::{
    gio,
    glib,
    Window,
    Widget,
    Application,
    ApplicationWindow,
    Accessible,
    Buildable,
    ConstraintTarget,
    Native,
    Root,
    ShortcutManager,
    subclass::prelude::*,
    prelude::*,
};

use gio::{
    ActionMap,
    ActionGroup, Settings,
    // SimpleAction,
};

// use glib::{
//     clone,
//     ObjectExt,
//     closure_local
// };

// use gio::MenuModel;

glib::wrapper! {
    pub struct BViewerWindow(ObjectSubclass<imp::BViewerWindow>)
        @extends ApplicationWindow, Window, Widget,
        @implements ActionGroup, ActionMap, Accessible, Buildable,
                    ConstraintTarget, Native, Root, ShortcutManager;
}

impl BViewerWindow {

    pub fn new<P: glib::IsA<Application>>(app: &P) -> Self {
        let win = glib::Object::new::<BViewerWindow>(&[("application", app)]).expect("Failed to create `ExApplicationWindow`");
        // ...
        win
    }

    // pub fn init_label(&self) {
    //     let _imp = self.imp();
    //     // ...
    // }

    fn setup_settings(&self) {
        let settings = Settings::new(crate::app::binflowviewer::APP_ID);
        self.imp()
            .settings
            .set(settings)
            .expect("`settings` should not be set before calling `setup_settings`.");
    }

    fn settings(&self) -> &Settings {
        self.imp()
            .settings
            .get()
            .expect("`settings` should be set in `setup_settings`.")
    }

    pub fn save_window_size(&self) -> Result<(), glib::BoolError> {
        // Get the size of the window
        let size = self.default_size();

        // Set the window state in `settings`
        self.settings().set_int("default-width", size.0)?;
        self.settings().set_int("default-height", size.1)?;
        self.settings()
            .set_boolean("is-maximized", self.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        // Get the window state from `settings`
        let width = self.settings().int("default-width");
        let height = self.settings().int("default-height");
        let is_maximized = self.settings().boolean("is-maximized");

        // Set the size of the window
        self.set_default_size(width, height);

        // If the window was maximized when it was closed, maximize it again
        if is_maximized {
            self.maximize();
        }
    }

}
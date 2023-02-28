pub mod imp;

use gtk4::{
    gio,
    glib,
    Window,
    Widget,
    Application,
    ApplicationWindow,
    // subclass::widget::TemplateChild,
    subclass::prelude::*,
    // prelude::*,
};

use gio::{
    ActionMap,
    ActionGroup,
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
        @extends Widget, Window, ApplicationWindow,
        @implements ActionGroup, ActionMap;
}

impl BViewerWindow {

    pub fn new<P: glib::IsA<Application>>(app: &P) -> Self {
        let win = glib::Object::new::<BViewerWindow>(&[("application", app)]).expect("Failed to create `ExApplicationWindow`");
        // ...
        win
    }

    pub fn init_label(&self) {
        let _imp = self.imp();
        // ...
    }

}
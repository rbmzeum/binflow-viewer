pub mod imp;
pub mod components;

use std::{path::Path, fs::OpenOptions, io::SeekFrom};
use std::io::{prelude::*};

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
    FileChooserDialog,
    FileChooserAction,
    ResponseType,
    subclass::prelude::*,
    prelude::*,
};

use gio::{
    ActionMap,
    ActionGroup, Settings, SimpleAction,
    // SimpleAction,
    // MenuModel,
};

use glib::{
    clone,
    // ObjectExt,
    // closure_local
};

glib::wrapper! {
    pub struct BViewerWindow(ObjectSubclass<imp::BViewerWindow>)
        @extends ApplicationWindow, Window, Widget,
        @implements ActionGroup, ActionMap, Accessible, Buildable,
                    ConstraintTarget, Native, Root, ShortcutManager;
}

impl BViewerWindow {

    pub fn new<P: glib::IsA<Application>>(app: &P) -> Self {
        let win = glib::Object::new::<BViewerWindow>(&[("application", app)]).expect("Failed to create `ExApplicationWindow`");
        win
    }

    // pub fn init_label(&self) {
    //     let _imp = self.imp();
    //     // ...
    // }

    pub fn setup_menubar(&self) {
        let app = self.application().expect("self does not have an application set");
        let menubuilder =
            gtk4::Builder::from_resource("/vs/binflow/viewer/data/resources/ui/menu.ui");
        let menubar: gio::MenuModel = menubuilder
            .object("main-menu")
            .expect("Could not get object 'main-menu' from builder.");
        app.set_menubar(Some(&menubar));
        self.set_show_menubar(true);
    }

    fn setup_actions(&self) {
        let window = self;

        let action_open = SimpleAction::new("open", None);
        action_open.connect_activate(clone!(@weak window => move |_action, _parameter| {
            let chooser = FileChooserDialog::builder()
                .modal(true)
                .action(FileChooserAction::Open)
                .title("Open binary data file")
                .build();
            chooser.set_transient_for(Some(&window));

            if let Err(_err) = chooser.set_current_folder(Some(&gio::File::for_path(window.settings().string("default-directory")))) {
                // tracing::warn!("Failed to set current folder: {:?}", err);
            }

            chooser.add_button("_Cancel", ResponseType::Cancel);
            chooser.add_button("_Select", ResponseType::Accept);
            chooser.set_default_response(ResponseType::Accept);

            chooser.present();

            chooser.connect_response(clone!(@weak window => move |chooser, response| {
                if response != ResponseType::Accept {
                    chooser.close();
                    return;
                }

                let filename = if let Some(filename) = chooser.file().and_then(|file| file.path()) {
                    filename
                } else {
                    // TODO: output message: "Please choose a file and try again."
                    return;
                };

                // Save current directory to settings
                let directory = filename
                    .parent()
                    .expect("Failed to get the directory from the path.")
                    .to_str()
                    .expect("Failed convert `Path` to `&str`");
                window.settings().set_string("default-directory", directory).expect("Failed to save `default-directory`");

                let f = filename.to_str().expect("Failed convert `Path` to `&str`");
                // println!("Selected filename: {:#?}", &filename);

                let mut file = OpenOptions::new()
                    .read(true)
                    .open(f)
                    .unwrap();

                // file.seek(SeekFrom::Start(offset)).unwrap();

                let length = match std::fs::metadata(f) {
                    Ok(m) => m.len(),
                    Err(_e) => 0,
                };

                let mut buf = vec![0; length as usize];
                let _r = file.read_exact(&mut buf);

                // convert Vec<u8> to Vec<f64>
                let values = buf.chunks(8).map(|v| {
                    f64::from_be_bytes([v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]])
                }).collect::<Vec<f64>>();

                window.imp().chart_component.set_values(values);
                window.imp().chart_component.get().queue_draw();

                chooser.close();
            }));
        }));
        window.add_action(&action_open);

        let action_quit = SimpleAction::new("quit", None);
        action_quit.connect_activate(clone!(@weak window => move |_action, _parameter| {
            window.close();
        }));
        window.add_action(&action_quit);
    }

    pub fn setup_accels_for_actions(&self) {
        let app = self.application().expect("self does not have an application set");

        // Set keyboard accelerator to trigger "win.quit".
        app.set_accels_for_action("win.quit", &["<Ctrl>Q"]);
    }

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
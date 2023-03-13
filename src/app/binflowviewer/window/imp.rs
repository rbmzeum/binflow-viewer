use gtk4::{
    // gio,
    glib,
    ApplicationWindow,
    // Label,
    // HeaderBar,
    Inhibit,
    CompositeTemplate,
    template_callbacks,
    prelude::*,
    subclass::prelude::*,
};

use glib::once_cell::sync::OnceCell;
use gio::Settings;

use super::components::chart::BChartComponent;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/vs/binflow/viewer/data/resources/ui/window.ui")]
pub struct BViewerWindow {
    // #[template_child(id = "label")]
    // pub main_menu_bar: TemplateChild<gtk4::Label>,

    // @see https://github.com/plotters-rs/plotters-gtk-demo/blob/master/src/main.rs
    #[template_child(id = "chart")]
    pub chart_component: TemplateChild<BChartComponent>,

    pub settings: OnceCell<Settings>,
}

#[glib::object_subclass]
impl ObjectSubclass for BViewerWindow {
    const NAME: &'static str = "BViewerWindow";
    type Type = super::BViewerWindow;
    type ParentType = ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        BViewerWindowCallbacks::bind_template_callbacks(klass);
    }

    // You must call `Widget`'s `init_template()` within `instance_init()`.
    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

struct BViewerWindowCallbacks {}

#[template_callbacks(functions)]
impl BViewerWindowCallbacks {
    // #[template_callback]
    // fn to_string(value: &glib::Value) -> String {
    //     if let Ok(value) = value.get::<u64>() {
    //         value.to_string()
    //     } else if let Ok(value) = value.get::<&str>() {
    //         value.to_owned()
    //     } else {
    //         "".into()
    //     }
    // }
    // #[template_callback]
    // fn strlen(s: &str) -> u64 {
    //     s.len() as u64
    // }
    // #[template_callback(name = "concat_strs")]
    // fn concat(#[rest] values: &[glib::Value]) -> String {
    //     let mut res = String::new();
    //     for (index, value) in values.iter().enumerate() {
    //         res.push_str(value.get::<&str>().unwrap_or_else(|e| {
    //             panic!("Expected string value for argument {}: {}", index, e);
    //         }));
    //     }
    //     res
    // }
    // #[template_callback(function = false, name = "reset_entry")]
    // fn reset(entry: &gtk4::Entry) {
    //     entry.set_text("Nothing");
    // }
}

impl ObjectImpl for BViewerWindow {

    fn constructed(&self, obj: &Self::Type) {
        // obj.init_label();
        self.parent_constructed(obj);

        // Load latest window state
        obj.setup_settings();
        obj.load_window_size();

        // Add actions
        obj.setup_actions();

        obj.setup_key_events();
    }

}

// impl BoxImpl for BViewerWindow {}

impl WidgetImpl for BViewerWindow {}

impl WindowImpl for BViewerWindow {
    // Save window state right before the window will be closed
    fn close_request(&self, obj: &Self::Type) -> Inhibit {
        // Save window size
        obj
            .save_window_size()
            .expect("Failed to save window state");

        // Don't inhibit the default handler
        Inhibit(false)
    }
}

impl ApplicationWindowImpl for BViewerWindow {}
use gtk4::{
    // gio,
    glib,
    ApplicationWindow,
    // Label,
    // HeaderBar,
    CompositeTemplate,
    // template_callbacks,
    prelude::*,
    subclass::prelude::*,
};

#[derive(Default, CompositeTemplate)]
#[template(file = "window.ui")]
pub struct BViewerWindow {
    #[template_child(id = "label")]
    pub main_menu_bar: TemplateChild<gtk4::Label>,
}

#[glib::object_subclass]
impl ObjectSubclass for BViewerWindow {
    const NAME: &'static str = "BViewerWindow";
    type Type = super::BViewerWindow;
    type ParentType = ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        WindowCallbacks::bind_template_callbacks(klass);
    }

    // You must call `Widget`'s `init_template()` within `instance_init()`.
    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

struct WindowCallbacks {}

#[gtk4::template_callbacks(functions)]
impl WindowCallbacks {
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
        obj.init_label();
        self.parent_constructed(obj);

        // Add actions
        // self.obj().setup_actions();
    }

}
// impl BoxImpl for BViewerWindow {}
impl WidgetImpl for BViewerWindow {}
impl WindowImpl for BViewerWindow {}
impl ApplicationWindowImpl for BViewerWindow {}
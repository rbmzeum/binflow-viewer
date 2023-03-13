use std::{cell::{RefCell, Cell}, rc::Rc};

// use gdk4::glib::WeakRef;
use gtk4::{
    glib,
    // glib::subclass::prelude::*,
    glib::subclass::Signal,
    CompositeTemplate,
    DrawingArea,
    prelude::*,
    subclass::prelude::*, gdk::Event, EventControllerKey,
};

use glib::{once_cell::sync::OnceCell, WeakRef};

#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/vs/binflow/viewer/data/resources/ui/chart.ui")]
pub struct BChartComponent {
    // pub state: OnceCell<BChartComponentState>,
    pub is_spacebar_pressed: Cell<bool>,
    pub values: RefCell<Vec<f64>>,
}

#[glib::object_subclass]
impl ObjectSubclass for BChartComponent {
    const NAME: &'static str = "BChartComponent";
    type Type = super::BChartComponent;
    type ParentType = DrawingArea;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[gtk4::template_callbacks]
impl BChartComponent {
    #[template_callback(name = "on_chart_resize")]
    fn on_chart_resize(&self, width: i32, height: i32) {
        let len = self.values.borrow().len();
        println!("ON CHART RESIZE: {:#?} {:#?} {:#?}", width, height, len);
    }

    // #[template_callback(name = "on_chart_key_pressed")]
    // fn on_chart_key_pressed(&self) {
    //     // let len = match self.values.get() {
    //     //     Some(x) => x.len(),
    //     //     None => 0,
    //     // };
    //     // println!("ON CHART RESIZE: {:#?} {:#?} {:#?}", width, height, len);
    //     println!("ON KEY PRESSED");
    // }
}

impl ObjectImpl for BChartComponent {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);

        obj.setup_drawing_area();
        // obj.setup_key_events();
    }

    // fn signals() -> &'static [Signal] {
    //     static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
    //         vec![Signal::builder(
    //             // Signal name
    //             "change-selected-symbol",
    //             // Types of the values which will be sent to the signal handler
    //             &[u32::static_type().into()],
    //             // Type of the value the signal handler sends back
    //             <()>::static_type().into(),
    //         )
    //         .build()]
    //     });
    //     SIGNALS.as_ref()
    // }
}

impl WidgetImpl for BChartComponent {
}

impl DrawingAreaImpl for BChartComponent {
}
use gtk4::{
    glib,
    // glib::subclass::prelude::*,
    glib::subclass::Signal,
    CompositeTemplate,
    DrawingArea,
    prelude::*,
    subclass::prelude::*,
};

#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/vs/binflow/viewer/data/resources/ui/chart.ui")]
pub struct BChartComponent {
    // callback: Box<dyn FnMut() -> Rc<Cell<u32>>>,
    // pub selected_symbol: Option<Arc<Cell<*mut u32>>>,
    // pub selected_symbol: Rc<Cell<u32>>,
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
        // ...
        // let mut d = self.chart_price.get();
        // let mut s = self.toolbar.get();
        // println!("CP & SS: {:#?} {:#?}", d.imp().selected_symbol.get(), s.imp().selected_symbol.get());
        // d.set_content_height(123);
        println!("ON CHART RESIZE: {:#?} {:#?}", width, height);
    }
}

impl ObjectImpl for BChartComponent {
    // Needed for direct subclasses of GtkWidget;
    // Here you need to unparent all direct children
    // of your template.
    // fn dispose(&self, obj: &Self::Type) {
    //     while let Some(child) = obj.first_child() {
    //         child.unparent();
    //     }
    // }
}

impl WidgetImpl for BChartComponent {
    // fn size_allocate(&self, widget: &Self::Type, width: i32, height: i32, baseline: i32) {
    //     self.parent_size_allocate(widget, width, height, baseline);
    //     self.popover.present();
    // }
}

impl DrawingAreaImpl for BChartComponent {
    // fn size_allocate(&self, widget: &Self::Type, width: i32, height: i32, baseline: i32) {
    //     self.parent_size_allocate(widget, width, height, baseline);
    //     self.popover.present();
    // }
}
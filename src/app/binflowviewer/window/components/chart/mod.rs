pub mod imp;

use gtk4::{
    glib,
    glib::closure_local,
    glib::ObjectExt,
    subclass::prelude::*,
    StringList,
    Widget,
    DrawingArea,
    prelude::DrawingAreaExtManual,
    cairo::{Context, Error},
};

use glib::{
    clone,
    // ObjectExt,
    // closure_local
};

glib::wrapper! {
    pub struct BChartComponent(ObjectSubclass<imp::BChartComponent>)
        @extends Widget, DrawingArea;
}

impl BChartComponent {

    pub fn set_values(&self, values: Vec<f64>) {
        self.imp().values.set(values);
    }

    pub fn setup_drawing_area(&self) {
        let chart = self;
        self.set_draw_func(clone!(@weak chart => move |drawing_area, ctx, width, height| {
            chart.draw_grid(drawing_area, ctx, width, height);
            // ...
        }));
    }

    pub fn draw_grid(&self, _drawing_area: &DrawingArea, ctx: &Context, width: i32, height: i32) {
        const PADDING_LEFT: f64 = 20.0;
        const PADDING_RIGHT: f64 = 80.0;
        const PADDING_BOTTOM: f64 = 20.0;
        const PADDING_TOP: f64 = 20.0;
        // const PADDING_CHART: f64 = 20.0; // отступ графика от краёв системы координат

        // Фон
        ctx.set_source_rgb(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0); // Background color
        ctx.paint().expect("Error in: components.chart.draw_grid > ctx.paint()");

        // Оси
        ctx.set_source_rgb(0.0 / 255.0, 0.0 / 255.0, 0.0 / 255.0);
        ctx.set_line_width(1.0);
        ctx.move_to(PADDING_LEFT, height as f64 - PADDING_BOTTOM);
        ctx.line_to(width as f64 - PADDING_RIGHT, height as f64 - PADDING_BOTTOM); // Ось абсцисс
        ctx.line_to(width as f64 - PADDING_RIGHT, PADDING_TOP); // Ось ординат
        ctx.stroke().expect("Error in: components.chart.draw_grid > ctx.stroke()");
    }

}
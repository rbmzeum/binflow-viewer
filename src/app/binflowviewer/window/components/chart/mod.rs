pub mod imp;

use std::cell::{Cell, RefCell};

use chrono::{Utc, Duration};
use chrono::prelude::*;

use gdk4::glib::WeakRef;
use glib::once_cell::sync::OnceCell;
use gtk4::traits::{GestureSingleExt, GestureDragExt};
use gtk4::{
    glib,
    glib::closure_local,
    glib::ObjectExt,
    subclass::prelude::*,
    EventControllerKey,
    Inhibit,
    StringList,
    Widget,
    DrawingArea,
    prelude::DrawingAreaExtManual,
    cairo::{Context, Error},
    traits::{EventControllerExt, WidgetExt},
    gdk::EventType::{
        ButtonPress,
        MotionNotify,
        ButtonRelease,
    },
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
        self.imp().values.replace(values);
    }

    pub fn setup_drawing_area(&self) {
        let chart = self;
        self.set_draw_func(clone!(@weak chart => move |drawing_area, ctx, width, height| {
            chart.draw_grid(drawing_area, ctx, width, height);
            chart.draw_chart(drawing_area, ctx, width, height);
            // ...
        }));
    }

    pub fn setup_drag(&self) {
        self.imp().offset.replace(0);
        self.imp().start_offset.replace(0);

        let gesture = gtk4::GestureDrag::new();
        self.add_controller(&gesture);
        gesture.set_exclusive(true);
        gesture.connect_drag_begin(clone!(@weak self as this => move |_, x, y| {
            this.imp().start_offset.replace(this.imp().offset.borrow().clone());
        }));
        gesture.connect_drag_update(clone!(@weak self as this => move |gesture, offset_x, offset_y| {
            if let Some((start_x, start_y)) = gesture.start_point() {
                let offset = this.imp().start_offset.borrow().clone();
                let mut new_offset = offset as i32 + offset_x.floor() as i32;
                if new_offset < 0 {
                    new_offset = 0;
                }
                if new_offset as usize > this.imp().values.borrow().len() {
                    new_offset = this.imp().values.borrow().len() as i32;
                }
                this.imp().offset.replace(new_offset as usize);
                this.queue_draw();
            }
        }));
        gesture.connect_drag_end(clone!(@weak self as this => move |gesture, offset_x, offset_y| {
            // if let Some((start_x, start_y)) = gesture.start_point() {
            // }
            // TODO: установить изображение курсора мыши в виде обычного курсора мыши (или в виде разжатой руки готовой для перетаскивания)
        }));
    }

    pub fn draw_grid(&self, _drawing_area: &DrawingArea, ctx: &Context, width: i32, height: i32) {
        const PADDING_LEFT: f64 = 20.0;
        const PADDING_RIGHT: f64 = 80.0;
        const PADDING_BOTTOM: f64 = 20.0;
        const PADDING_TOP: f64 = 20.0;
        const PADDING_CHART: f64 = 20.0; // отступ графика от краёв системы координат

        // Фон
        ctx.set_source_rgb(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0); // Background color
        ctx.paint().expect("Error in: components.chart.draw_grid > ctx.paint()");

        // Отображение шкалы и сетки на основе загруженных данных

        // Оси
        ctx.set_source_rgb(0.0 / 255.0, 0.0 / 255.0, 0.0 / 255.0);
        ctx.set_line_width(1.0);
        ctx.move_to(PADDING_LEFT, height as f64 - PADDING_BOTTOM);
        ctx.line_to(width as f64 - PADDING_RIGHT, height as f64 - PADDING_BOTTOM); // Ось абсцисс
        ctx.line_to(width as f64 - PADDING_RIGHT, PADDING_TOP); // Ось ординат
        ctx.stroke().expect("Error in: components.chart.draw_grid > ctx.stroke()");

        // Шкала оси абсцисс и вертикальная разметка сетки
        let dt = Local::now();
        let dh = dt.hour();
        let dm = dt.minute();
        // let mdt = dt - Duration::seconds(dt.second() as i64);

        let timeframe = 1; // minutes
        let m = 60 * timeframe;

        // Вычисление требуемого для сетки кол-ва вертикальных линий
        let mut cx = 1 + (width - PADDING_LEFT as i32 - PADDING_RIGHT as i32) / m;
        if (width - PADDING_LEFT as i32 - PADDING_RIGHT as i32) % m > dm as i32 {
            cx = cx + 1;
        }

        // Отображение вертикальных линий сетки
        for n in 1 .. cx {
            let x = (width as f64 - PADDING_RIGHT) - dm as f64 - ((n - 1) * m) as f64;
            ctx.set_source_rgb(0.0 / 255.0, 0.0 / 255.0, 0.0 / 255.0);
            ctx.move_to(x, height as f64 - PADDING_BOTTOM);
            ctx.line_to(x, height as f64 - (PADDING_BOTTOM + 4.0));
            ctx.stroke();

            // Вертикальная разметка сетки
            ctx.set_source_rgb(230.0 / 255.0, 230.0 / 255.0, 230.0 / 255.0);
            ctx.move_to(x, height as f64 - (PADDING_BOTTOM + 5.0));
            ctx.line_to(x, PADDING_TOP);
            ctx.stroke();

            // Подпись (время)
            let mut t = dh as i32 + 1 - n;
            while t < 0 {
                t = t + 24;
            }
            let sh = t.to_string();
            ctx.set_source_rgb(17.0 / 255.0, 34.0 / 255.0, 45.0 / 255.0);
            ctx.set_font_size(14.0);
            let b = ctx.text_extents(sh.as_str()).unwrap();
            ctx.move_to(x - (b.width / 2.0 + b.x_bearing), height as f64 - 4.0);
            ctx.show_text(sh.as_str());
        }

        let values = self.imp().values.borrow();
        let offset = self.imp().offset.borrow().clone();
        // TODO: показывать графики с длиной меньше width
        if values.len() > width as usize {
            let v = &values[(values.len() - width as usize + PADDING_LEFT as usize + PADDING_RIGHT as usize) - offset..values.len() - offset];
            let max = v.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            let min = v.iter().copied().fold(f64::INFINITY, f64::min);

            // Отображение горизонтальных линий сетки и шкалы
            let n = 20;
            let hmm = (height as f64 - (PADDING_BOTTOM + 2.0 * PADDING_CHART)) / (max-min) as f64;
            for i in 0..=n {
                let p = min + i as f64 * (max-min) / n as f64;
                let y = height as f64 - PADDING_BOTTOM - PADDING_CHART - (p - min) * hmm;

                let sp = format!("{:.2}", p);
                ctx.set_source_rgb(17.0 / 255.0, 34.0 / 255.0, 45.0 / 255.0);
                ctx.set_font_size(10.0);
                let b = ctx.text_extents(sp.as_str()).unwrap();
                ctx.move_to(width as f64 - (PADDING_RIGHT - 4.0), y - (b.height / 2.0 + b.y_bearing));
                ctx.show_text(sp.as_str());

                ctx.set_source_rgb(230.0 / 255.0, 230.0 / 255.0, 230.0 / 255.0);
                ctx.move_to(PADDING_LEFT, y);
                ctx.line_to(width as f64 - (PADDING_RIGHT + 5.0), y); // Горизонтальные линии сетки
                ctx.stroke();

                ctx.set_source_rgb(0.0 / 255.0, 0.0 / 255.0, 0.0 / 255.0);
                ctx.move_to(width as f64 - (PADDING_RIGHT + 6.0), y);
                ctx.line_to(width as f64 - PADDING_RIGHT, y); // Деление шкалы на оси ординат
                ctx.stroke();
            }
        }
    }

    pub fn draw_chart(&self, _drawing_area: &DrawingArea, ctx: &Context, width: i32, height: i32) {
        const PADDING_LEFT: f64 = 20.0;
        const PADDING_RIGHT: f64 = 80.0;
        const PADDING_BOTTOM: f64 = 20.0;
        const PADDING_TOP: f64 = 20.0;
        const PADDING_CHART: f64 = 20.0; // отступ графика от краёв системы координат

        // let x = self.imp().values.borrow_mut();
        let values = self.imp().values.borrow();
        let offset = self.imp().offset.borrow().clone();
        // TODO: показывать графики с длиной меньше width
        if values.len() > width as usize {
            let v = &values[(values.len() - width as usize + PADDING_LEFT as usize + PADDING_RIGHT as usize) - offset..values.len() - offset];
            let max = v.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            let min = v.iter().copied().fold(f64::INFINITY, f64::min);

            let hmm = (height as f64 - (PADDING_BOTTOM + 2.0 * PADDING_CHART)) / (max-min) as f64;

            // Отображение графика
            ctx.set_source_rgb(67.0 / 255.0, 70.0 / 255.0, 255.0 / 255.0); // Set the chart lines color
            ctx.set_line_width(1.0);
            for (ix, p) in v.windows(2).rev().enumerate() {
                let x = width as f64 - PADDING_RIGHT - ix as f64;
                let y1 = height as f64 - PADDING_BOTTOM - PADDING_CHART - (p[0] - min) * hmm;
                let y2 = height as f64 - PADDING_BOTTOM - PADDING_CHART - (p[1] - min) * hmm;
                if (y1 == y2) {
                    ctx.move_to(x + 1.0, y1);
                    ctx.line_to(x, y2);
                } else {
                    ctx.move_to(x, y1);
                    ctx.line_to(x, y2);
                }
                ctx.stroke();
            }

            // Отображение текущей цены
            let p = v[v.len() - 1];
            let y = height as f64 - PADDING_BOTTOM - PADDING_CHART - (p - min) * hmm;

            // фон для текущей цены
            ctx.set_source_rgb(67.0 / 255.0, 70.0 / 255.0, 255.0 / 255.0);
            ctx.set_line_width(15.0);
            ctx.move_to(width as f64 - (PADDING_RIGHT - 1.0), y);
            ctx.line_to(width as f64, y);
            ctx.stroke();

            // текущая цена
            let sp = format!("{:.2}", p);
            ctx.set_source_rgb(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0);
            ctx.set_font_size(10.0);
            let b = ctx.text_extents(sp.as_str()).unwrap();
            ctx.move_to(width as f64 - (PADDING_RIGHT - 4.0), y - (b.height / 2.0 + b.y_bearing));
            ctx.show_text(sp.as_str());
        }
    }

}
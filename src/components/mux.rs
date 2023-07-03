use crate::{
    common::{Component, Input, Output, OutputType, Ports, Simulator},
    gui_vizia::{tooltip::new_component_tooltip, GuiData},
};
use serde::{Deserialize, Serialize};
use vizia::{
    prelude::*,
    vg::{Paint, Path},
};

#[derive(Serialize, Deserialize)]
pub struct Mux {
    pub id: String,
    pub pos: (f32, f32),
    pub select: Input,
    pub m_in: Vec<Input>,
}

#[typetag::serde]
impl Component for Mux {
    fn to_(&self) {
        println!("mux");
    }

    fn get_id_ports(&self) -> (String, Ports) {
        let mut inputs = vec![self.select.clone()];
        let mut m = self.m_in.clone();
        inputs.append(&mut m);

        (
            self.id.clone(),
            Ports {
                inputs,
                out_type: OutputType::Combinatorial,
                outputs: vec![Output::Function],
            },
        )
    }

    // propagate selected input value to output
    fn evaluate(&self, simulator: &mut Simulator) {
        // get input value
        let select = simulator.get_input_val(&self.select) as usize;
        println!("select {}", select);
        let value = simulator.get_input_val(&self.m_in[select]);

        // set output
        simulator.set_id_index(&self.id, 0, value);
    }

    // create view
    fn view(&self, cx: &mut Context) {
        println!("---- Create Add View");

        View::build(
            MuxView {
                select: self.select.clone(),
            },
            cx,
            |_cx| {},
        )
        .position_type(PositionType::SelfDirected)
        .left(Pixels(self.pos.0 - 20.0))
        .top(Pixels(self.pos.1 - 10.0 * self.m_in.len() as f32 - 10.0))
        .width(Pixels(40.0))
        .height(Pixels(20.0 * self.m_in.len() as f32 + 20.0))
        .tooltip(|cx| new_component_tooltip(cx, self));
    }
}

pub struct MuxView {
    select: Input,
}

impl View for MuxView {
    fn element(&self) -> Option<&'static str> {
        Some("Mux")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        let scale = cx.scale_factor();
        // println!("Mux draw {:?}", bounds);

        let mut path = Path::new();
        let mut paint = Paint::color(vizia::vg::Color::rgbf(0.0, 0.0, 0.0));
        paint.set_line_width(cx.logical_to_physical(1.0));

        let height = bounds.height();
        let width = bounds.width();
        let top = bounds.top();
        let left = bounds.left();
        let right = bounds.right();
        let bottom = bounds.bottom();

        // top left
        path.move_to(left + 0.5, top + 0.5);

        // top and right corner
        path.line_to(left + width * 0.5 + 0.5, top + 0.5);
        path.line_to(right + 0.5, top + height * 0.25 + 0.5);

        // bottom and right corner
        path.line_to(bounds.right() + 0.5, bottom - height * 0.25 + 0.5);
        path.line_to(left + width * 0.5 + 0.5, bottom + 0.5);
        path.line_to(left + 0.5, bottom + 0.5);

        // left side
        path.line_to(left + 0.5, top + 0.5);

        canvas.stroke_path(&path, &paint);

        // selector
        let simulator = GuiData::simulator.get(cx);

        let select = simulator.get_input_val(&self.select);

        // println!("----- select = {}", select);
        paint = Paint::color(vizia::vg::Color::rgbf(1.0, 0.0, 0.0));
        let mut path = Path::new();

        path.move_to(
            left + 0.5,
            top + 0.5 + (20.0 + select as f32 * 20.0) * scale,
        );
        path.line_to(right + 0.5, top + height * 0.5 + 0.5);
        canvas.stroke_path(&path, &paint);
    }
}

use crate::common::{Component, Input, OutputType, Ports, Simulator};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use vizia::prelude::*;
use vizia::vg::{Paint, Path};

#[derive(Serialize, Deserialize)]
pub struct Probe {
    pub id: String,
    pub pos: (f32, f32),
    pub input: Input,
}

#[typetag::serde]
impl Component for Probe {
    fn to_(&self) {
        println!("Probe");
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                // Probes take one input
                inputs: vec![self.input.clone()],
                out_type: OutputType::Combinatorial,
                // No output value
                outputs: vec![],
            },
        )
    }

    // egui
    fn render(
        &self,
        sim_state: &mut crate::common::SimState,
        ui: &mut egui::Ui,
        simulator: Rc<Simulator>,
        offset: egui::Vec2,
        scale: f32,
    ) {
        let mut offset = offset.clone();
        offset.x += self.pos.0 * scale;
        offset.y += self.pos.1 * scale;
        let input = self.input.clone();
        let w = egui::Window::new(format!("test{}", self.id))
            .movable(false)
            .frame(egui::Frame {
                inner_margin: egui::Margin::same(1f32),
                outer_margin: egui::Margin::same(1f32),
                rounding: egui::Rounding::none(),
                shadow: epaint::Shadow::NONE,
                fill: egui::Color32::LIGHT_BLUE,
                stroke: egui::Stroke::NONE,
            })
            .fixed_pos(egui::Pos2 {
                x: offset.x,
                y: offset.y,
            })
            .title_bar(false)
            .resizable(false)
            .pivot(egui::Align2::CENTER_CENTER);
        w.show(ui.ctx(), |ui| {
            ui.label(format!(
                " {:?}",
                simulator.clone().get_input_val(sim_state, &input)
            ));
        });
    }

    // create view vizia
    fn view(&self, cx: &mut Context, simulator: Rc<Simulator>) {
        println!("---- Create Probe View");
        View::build(ProbeView {}, cx, |cx| {
            let input = self.input.clone();

            Label::new(
                cx,
                crate::gui::Gui::state.map(move |sim_state| {
                    format!(" {:?}", simulator.clone().get_input_val(sim_state, &input))
                }),
            );
        })
        .position_type(PositionType::SelfDirected)
        // .min_size(Pixels(20.0))
        .left(Pixels(self.pos.0 - 10.0))
        .top(Pixels(self.pos.1 - 10.0))
        .width(Pixels(20.0))
        .height(Pixels(20.0));
    }
}

pub struct ProbeView {}

impl View for ProbeView {
    fn element(&self) -> Option<&'static str> {
        Some("Probe")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        // println!("Probe draw {:?}", bounds);

        let mut path = Path::new();
        let mut paint = Paint::color(vizia::vg::Color::rgbf(0.0, 1.0, 1.0));
        paint.set_line_width(cx.logical_to_physical(1.0));

        path.move_to(bounds.left() + 0.5, bounds.top() + 0.5);
        path.line_to(bounds.right() + 0.5, bounds.top() + 0.5);
        path.line_to(bounds.right() + 0.5, bounds.bottom() + 0.5);
        path.line_to(bounds.left() + 0.5, bounds.bottom() + 0.5);
        path.line_to(bounds.left() + 0.5, bounds.top() + 0.5);

        canvas.fill_path(&path, &paint);
    }
}

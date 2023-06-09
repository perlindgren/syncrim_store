use std::{path::PathBuf, rc::Rc};
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
};

fn main() {
    let cs = ComponentStore {
        store: vec![
            Rc::new(Constant {
                id: "c".to_string(),
                pos: (150.0, 100.0),
                value: 3,
            }),
            Rc::new(Register {
                id: "reg".to_string(),
                pos: (200.0, 100.0),
                r_in: Input::new("c", 0),
            }),
            Rc::new(Wire {
                id: "w1".to_string(),
                pos: (160.0, 100.0),
                delta: (30.0, 0.0),
                input: Input::new("c", 0),
            }),
            Rc::new(Wire {
                id: "w2".to_string(),
                pos: (210.0, 100.0),
                delta: (30.0, 0.0),
                input: Input::new("reg", 0),
            }),
            Rc::new(Probe {
                id: "p_reg".to_string(),
                pos: (250.0, 100.0),
                input: Input::new("reg", 0),
            }),
        ],
    };

    let path = PathBuf::from("reg.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(&cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(&cs, &path);
}

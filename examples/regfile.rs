use std::cell::Cell;
use std::rc::Rc;
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
    gui_vizia::gui,
};

fn main() {
    let cs = ComponentStore {
        path: "regfile.json".to_string(),
        store: vec![
            Rc::new(Constant {
                id: "c_read_reg_1".to_string(),
                pos: (100.0, 100.0),
                value: 3,
            }),
            Rc::new(Constant {
                id: "c_read_reg_2".to_string(),
                pos: (100.0, 200.0),
                value: 4,
            }),
            Rc::new(Constant {
                id: "c_write_data".to_string(),
                pos: (100.0, 140.0),
                value: 42,
            }),
            Rc::new(Constant {
                id: "c_write_addr".to_string(),
                pos: (100.0, 160.0),
                value: 4,
            }),
            Rc::new(Constant {
                id: "c_write_enable".to_string(),
                pos: (100.0, 180.0),
                value: true as u32,
            }),
            // regfile
            Rc::new(RegFile {
                id: "reg_file".to_string(),
                pos: (200.0, 150.0),
                width: 100.0,
                height: 150.0,

                // ports
                read_addr: vec![
                    Input {
                        id: "c_read_reg_1".to_string(),
                        index: 0,
                    },
                    Input {
                        id: "c_read_reg_2".to_string(),
                        index: 0,
                    },
                ],
                write_data: Input {
                    id: "c_write_data".to_string(),
                    index: 0,
                },
                write_addr: Input {
                    id: "c_write_addr".to_string(),
                    index: 0,
                },
                write_enable: Input {
                    id: "c_write_enable".to_string(),
                    index: 0,
                },

                // data
                registers: vec![Cell::new(0); 32],
            }),
        ],
    };

    cs.save_file();

    gui(&cs);
}
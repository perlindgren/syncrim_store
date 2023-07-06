pub(crate) struct Menu {}

impl Menu {
    pub(crate) fn new(ui: &mut egui::Ui, gui: &mut crate::gui_egui::gui::Gui) {
        fn btn(ui: &mut egui::Ui, name: &str, keys: egui::KeyboardShortcut) -> egui::Response {
            ui.add(egui::Button::new(name).shortcut_text(ui.ctx().format_shortcut(&keys)))
        }

        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if btn(ui, "File", gui.shortcuts.file_new).clicked() {
                    // New here
                }
                if btn(ui, "Open", gui.shortcuts.file_open).clicked() {
                    // Open here
                }
                ui.menu_button("Open Recent", |ui| {
                    // Recent here
                    //if ui.button("file1").clicked() {
                    //    // Open file
                    //}
                });
                ui.separator();
                if btn(ui, "Save", gui.shortcuts.file_save).clicked() {
                    // Save here
                }
                if btn(ui, "Save As", gui.shortcuts.file_save_as).clicked() {
                    // Save As here
                }
                ui.separator();
                if btn(ui, "Preferences", gui.shortcuts.file_preferences).clicked() {
                    // Preferences here
                }
                if btn(ui, "Quit", gui.shortcuts.file_quit).clicked() {
                    // Quit here
                }
            });

            ui.menu_button("Edit", |ui| {
                if btn(ui, "Cut", gui.shortcuts.edit_cut).clicked() {
                    // Cut here
                }
                if btn(ui, "Copy", gui.shortcuts.edit_copy).clicked() {
                    // Copy here
                }
                if btn(ui, "Paste", gui.shortcuts.edit_paste).clicked() {
                    // Paste here
                }
            });

            ui.menu_button("View", |ui| {
                if btn(ui, "Zoom In", gui.shortcuts.view_zoom_in).clicked() {
                    crate::gui_egui::keymap::view_zoom_in_fn(gui);
                }
                if btn(ui, "Zoom Out", gui.shortcuts.view_zoom_out).clicked() {
                    crate::gui_egui::keymap::view_zoom_out_fn(gui);
                }
                ui.menu_button("Zoom Level", |ui| {
                    if ui.button("10%").clicked() {
                        // 10% zoom here here
                        gui.scale = 0.1f32;
                    }
                    if ui.button("25%").clicked() {
                        // 25% zoom here here
                        gui.scale = 0.25f32;
                    }
                    if ui.button("50%").clicked() {
                        // 50% zoom here here
                        gui.scale = 0.5f32;
                    }
                    if ui.button("100%").clicked() {
                        // 100% zoom here here
                        gui.scale = 1f32;
                    }
                    if ui.button("150%").clicked() {
                        // 150% zoom here here
                        gui.scale = 1.5f32;
                    }
                    if ui.button("200%").clicked() {
                        // 200% zoom here here
                        gui.scale = 2f32;
                    }
                });
            });

            ui.menu_button("Help", |ui| {
                if ui.button("Show license").clicked() {
                    // Show license here
                }
                if ui.button("About").clicked() {
                    // About here
                }
            });
        });
        ui.horizontal(|ui| {
            if ui.button("⟲").clicked() {
                gui.simulator.reset(&mut gui.clock);
                gui.pause = true;
            }
            if ui.button("⏮").clicked() {
                gui.simulator.un_clock(&mut gui.clock);
            }
            if ui.button("⏭").clicked() {
                gui.simulator.clock(&mut gui.clock);
            }
            if ui.button("▶").clicked() {
                gui.pause = false;
            }
            if ui.button("⏸").clicked() {
                gui.pause = true;
            }
            ui.label(format!("Clock #{}", gui.clock));
        });
    }
}
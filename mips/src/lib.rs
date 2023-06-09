// Library specific to the MIPS architecture
pub mod components;

#[cfg(feature = "gui-vizia")]
pub mod gui_vizia;

#[cfg(feature = "gui-egui")]
pub mod gui_egui;

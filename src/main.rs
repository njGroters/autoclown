use std::default;

use eframe::{
    egui::{CentralPanel, Ui},
    epi::App,
    run_native, NativeOptions,
};

struct AutoClown;

impl App for AutoClown {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.label("hello world!");
        });
    }

    fn name(&self) -> &str {
        "AutoClown"
    }
}

fn main() {
    let app: AutoClown = AutoClown;
    let win_option = NativeOptions::default();

    //run_native(Box::new(app), win_option);
}

use std::default;
use xcap::Window;

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

fn normalization(filename: &str) -> String {
    filename
        .replace("|", "")
        .replace("\\", "")
        .replace(":", "")
        .replace("/", "")
}

fn main() {
    let windows = Window::all().unwrap();

    let mut i = 0;

    for window in windows {
        if window.is_minimized() {
            continue;
        }

        println!(
            "Window: {:?} {:?} {:?}",
            window.title(),
            (window.x(), window.y(), window.width(), window.height()),
            (window.is_minimized(), window.is_maximized())
        );

        let image = window.capture_image().unwrap();

        image
            .save(format!(
                "target/window--{}--{}.png",
                i,
                normalization(window.title())
            ))
            .unwrap();

        i += 1;
    }

    let app: AutoClown = AutoClown;
    let win_option = NativeOptions::default();

    //run_native(Box::new(app), win_option);
}

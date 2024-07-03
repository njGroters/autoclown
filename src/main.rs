#![allow(unused_imports, dead_code, unused_variables)]
use std::default;
use std::time::Instant;
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
    filename.replace(['|', '\\', ':', '/'], "")
}

fn main() {
    let start = Instant::now();

    let windows = Window::all().unwrap();

    let mut i = 0;

    for window in windows {
        if window.title() != "Freddy Fazbear's Pizzeria Simulator" {
            continue;
        }

        /*
        println!(
            "Window: {:?} {:?} {:?}",
            window.title(),
            (window.x(), window.y(), window.width(), window.height()),
            (window.is_minimized(), window.is_maximized())
        );
        */

        let image = window.capture_image().unwrap();

        /*
        image
            .save(format!(
                "target/window--{}--{}.png",
                i,
                normalization(window.title())
            ))
            .unwrap();
        */

        i += 1;
    }

    println!("Took: {:?}", start.elapsed());

    let app: AutoClown = AutoClown;
    let win_option = NativeOptions::default();

    //run_native(Box::new(app), win_option);
}

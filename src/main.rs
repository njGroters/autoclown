#![allow(unused_imports, dead_code, unused_variables)]
use std::time::Instant;
use std::{default, u32};
use xcap::{
    image::{DynamicImage, GenericImage},
    Window,
};

use console::Term;

struct Subwindow {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

fn main() {
    let start = Instant::now();

    let stdout = Term::buffered_stdout();

    let deadzone = 100;
    let mut active = true;

    let left_side = Subwindow {
        x: 0,
        y: 825 + 8,
        width: 514, // should use deadzone var here
        height: 95,
    };

    let right_side = Subwindow {
        x: 1177 + deadzone + 8,
        ..left_side
    };

    let subwindows = [left_side, right_side];

    let windows = Window::all().unwrap();

    for window in windows {
        if window.title() != "Freddy Fazbear's Pizzeria Simulator" {
            continue;
        }

        /* Printing out window info to console
        println!(
            "Window: {:?} {:?} {:?}",
            window.title(),
            (window.x(), window.y(), window.width(), window.height()),
            (window.is_minimized(), window.is_maximized())
        );
        */

        let image: DynamicImage =
            xcap::image::DynamicImage::ImageRgba8(window.capture_image().unwrap());

        let mut image = image.into_luma8();

        image.save("full-test-window.png").unwrap();
        //let mut part = image.sub_image(180 + 8, 825 + 8, (1560 / 2) - deadzone, 95);

        let mut clear_count = 0;

        for (i, sub) in subwindows.iter().enumerate() {
            let part = image.sub_image(sub.x, sub.y, sub.width, sub.height);

            if !part.to_image().contains(&44) {
                //part.change_bounds(1177 + deadzone + 8, 0 + 8, 383 - deadzone - 8, 95);
                //part.change_bounds(x, y, width, height)
                println!("got a clear subimage");

                //part.to_image().save("right-side.png").unwrap();
                clear_count += 1;
            } else {
                println!("got blocked subimage");
                part.to_image()
                    .save(format!("blocked_sub_{}.png", i))
                    .unwrap();
            }
        }

        if clear_count >= 2 {
            println!("Cleared!");
        } else {
            println!("Nope");
        }

        // let left_side = image.view(0, 0, 383 - deadzone, 95);
        // let right_side = image.view(1177 + deadzone, 0, 383 - deadzone, 95);
    }

    println!("Took: {:?}", start.elapsed());
}

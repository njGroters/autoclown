#![allow(unused_imports, dead_code, unused_variables)]
#[macro_use]
extern crate crossterm;

use std::{
    char,
    io::{self, stdout},
    thread,
    time::{Duration, Instant},
};
use std::{default, os::windows::process};
use xcap::{
    image::{DynamicImage, GenericImage},
    Window,
};

use enigo::{
    Direction::{Press, Release},
    Enigo, Key, Keyboard, Settings,
};

use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{cursor, event::poll, style::Print};

struct Subwindow {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

fn main() {
    let deadzone = 100;
    let mut active = false;
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

    let mut term = stdout();

    let mut swap_time = Instant::now();

    loop {
        // Nonblocking check for a keypress
        if keypress_event_found().unwrap() {
            if check_keypress('p') {
                if Instant::now() >= swap_time + Duration::from_millis(200) {
                    swap_time = Instant::now();
                    active = !active
                }
            } else if check_keypress('q') {
                std::process::exit(0);
            }
        }

        if active {
            execute!(
                term,
                Clear(ClearType::All),
                cursor::MoveTo(0, 0),
                Print("Checking for sides to be clear...")
            )
            .unwrap();
            let windows = Window::all().unwrap();

            for window in windows {
                if window.title() != "Freddy Fazbear's Pizzeria Simulator" {
                    continue;
                }

                let image: DynamicImage =
                    xcap::image::DynamicImage::ImageRgba8(window.capture_image().unwrap());

                let mut image = image.into_luma8();

                //image.save("full-test-window.png").unwrap();
                //let mut part = image.sub_image(180 + 8, 825 + 8, (1560 / 2) - deadzone, 95);

                let mut clear_count = 0;

                for (i, sub) in subwindows.iter().enumerate() {
                    let part = image.sub_image(sub.x, sub.y, sub.width, sub.height);

                    if !part.to_image().contains(&44) {
                        //part.change_bounds(1177 + deadzone + 8, 0 + 8, 383 - deadzone - 8, 95);
                        //part.change_bounds(x, y, width, height)
                        //println!("got a clear subimage");

                        execute!(term, cursor::MoveTo(0, 1), Print("Found a clear subimage"))
                            .unwrap();
                        //part.to_image().save("right-side.png").unwrap();
                        clear_count += 1;
                    } else {
                        //println!("got blocked subimage");
                        //part.to_image()
                        //    .save(format!("blocked_sub_{}.png", i))
                        //    .unwrap();
                    }
                }

                if clear_count >= 2 {
                    //println!("Cleared!");
                    execute!(term, Print("\nAll clear!")).unwrap();
                    send_space_keypress();
                } else {
                    //println!("Nope");
                }

                // let left_side = image.view(0, 0, 383 - deadzone, 95);
                // let right_side = image.view(1177 + deadzone, 0, 383 - deadzone, 95);
            }
        } else {
            //println!("Inactive, press \'q\' to resume");

            execute!(
                term,
                Clear(ClearType::All),
                cursor::MoveTo(0, 0),
                Print("Inactive, press \'p\' to resume or \'q\' to quit")
            )
            .unwrap();
        }
    }
}

fn keypress_event_found() -> io::Result<bool> {
    poll(Duration::from_millis(30))
}

fn send_space_keypress() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    enigo.key(Key::Space, Press).unwrap();
    thread::sleep(Duration::from_millis(160));
    enigo.key(Key::Space, Release).unwrap();
}

fn check_keypress(letter: char) -> bool {
    if let Event::Key(KeyEvent { code, .. }) = read().unwrap() {
        code == KeyCode::Char(letter)
    } else {
        false
    }
}

extern crate hsl;
extern crate image;
extern crate num;
extern crate sdl2;
extern crate termion;

use hsl::HSL;
use num::complex::Complex;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::time::Instant;
use termion::clear;
use termion::cursor;
pub fn main() {
    let mut ws = WindowSpec {
        max_iter: 64,
        center: Complex::new(0.0, 0.0),
        win_size: 2.0,
        img_size: (960, 540),
        print_size: (1920*4, 1080*4),
        bands: 1,
    };

    let num_threads = 4;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", ws.img_size.0, ws.img_size.1)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut first = true;
    let mut pow = 0.5;
    'running: loop {
        let mut trigger = false;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyUp {
                    keycode: Some(Keycode::KpPlus),
                    ..
                } => {
                    ws.win_size /= 2.0;

                    println!("Registered +");
                    println!(
                        "Center: ({}, {}), Window Width: {}, M_I: {}",
                        ws.center.re, ws.center.im, ws.win_size, ws.max_iter
                    );
                    trigger = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::KpMinus),
                    ..
                } => {
                    ws.win_size *= 2.0;

                    println!("Registered -");
                    println!(
                        "Center: ({}, {}), Window Width: {}, M_I: {}",
                        ws.center.re, ws.center.im, ws.win_size, ws.max_iter
                    );
                    trigger = true;
                }
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Left,
                    x: new_x,
                    y: new_y,
                    ..
                } => {
                    let wd = ws.get_win_dim();
                    ws.center.re += new_x as f64 / ws.img_size.0 as f64 * wd.0 - wd.0 / 2.0;
                    ws.center.im += new_y as f64 / ws.img_size.1 as f64 * wd.1 - wd.1 / 2.0;
                    println!("Registered click");
                    println!(
                        "Center: ({}, {}), Window Width: {}, M_I: {}",
                        ws.center.re, ws.center.im, ws.win_size, ws.max_iter
                    );
                    trigger = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::N),
                    ..
                } => {
                    ws.max_iter /= 2;
                    if ws.max_iter == 0 {
                        ws.max_iter = 1;
                    }
                    println!("Registered N");
                    println!(
                        "Center: ({}, {}), Window Width: {}, M_I: {}",
                        ws.center.re, ws.center.im, ws.win_size, ws.max_iter
                    );
                    trigger = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::M),
                    ..
                } => {
                    ws.max_iter *= 2;
                    println!("Registered M");
                    println!(
                        "Center: ({}, {}), Window Width: {}, M_I: {}",
                        ws.center.re, ws.center.im, ws.win_size, ws.max_iter
                    );
                    trigger = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::P),
                    ..
                } => {
                    println!("Registered P");
                    println!(
                        "Center: ({}, {}), Window Width: {}, M_I: {}, Print Size {}, {}",
                        ws.center.re,
                        ws.center.im,
                        ws.win_size,
                        ws.max_iter,
                        ws.print_size.0,
                        ws.print_size.1
                    );
                    dump_to_file(ws);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::I),
                    ..
                } => {
                    ws.print_size.0 /= 2;
                    ws.print_size.1 /= 2;
                    if ws.print_size.0 < 240 {
                        ws.print_size.0 = 240;
                        ws.print_size.1 = 135;
                    }
                    println!("Registered I");
                    println!(
                        "Center: ({}, {}), Window Width: {}, M_I: {}, Print Size {}, {}",
                        ws.center.re,
                        ws.center.im,
                        ws.win_size,
                        ws.max_iter,
                        ws.print_size.0,
                        ws.print_size.1
                    );
                }
                Event::KeyUp {
                    keycode: Some(Keycode::O),
                    ..
                } => {
                    ws.print_size.0 *= 2;
                    ws.print_size.1 *= 2;
                    println!("Registered O");
                    println!(
                        "Center: ({}, {}), Window Width: {}, M_I: {}, Print Size {}, {}",
                        ws.center.re,
                        ws.center.im,
                        ws.win_size,
                        ws.max_iter,
                        ws.print_size.0,
                        ws.print_size.1
                    );
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Q),
                    ..
                } => {
                    /*ws.bands -= 1;
                    if ws.bands < 1 {
                        ws.bands = 1;
                    }
                    println!("Registered Q");
                    println!(
                        "Center: ({}, {}), Window Width: {}, M_I: {}, Print Size {}, {}",
                        ws.center.re,
                        ws.center.im,
                        ws.win_size,
                        ws.max_iter,
                        ws.print_size.0,
                        ws.print_size.1
                    );*/

                    pow -= 0.05;
                    println!("{}", pow);

                    trigger = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    /*
                    ws.bands += 1;
                    if ws.bands < 1 {
                        ws.bands = 1;
                    }
                    println!("Registered W");
                    println!(
                        "Center: ({}, {}), Window Width: {}, M_I: {}, Print Size {}, {}",
                        ws.center.re,
                        ws.center.im,
                        ws.win_size,
                        ws.max_iter,
                        ws.print_size.0,
                        ws.print_size.1
                    );*/
                    pow += 0.05;
                    println!("{}", pow);

                    trigger = true;
                }
                _ => {}
            }
        }

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        /*
          New parameter that stores the desired number of threads
          On trigger, spawn that number of threads and each gets that fraction
          of the pixels to share, returning a Vec of coord color tuples
        */

        if trigger || first {
            first = false;
            let mut rows = calc(ws, num_threads, false);
            //insert normalization process here
            //find the smallest value (0 if there is a None value)
            let mut min = 360.0;
            let mut max = 0.0;
            for r in rows.iter() {
                for v in r.pixels.iter() {
                    if let Some(x) = v {
                        if x < &min {
                            min = *x;
                        }
                        if x > &max {
                            max = *x;
                        }
                    }
                }
            }
            println!("min: {} max: {}", min, max);

            for row in rows.into_iter() {
                for (x, i) in row.pixels.into_iter().enumerate() {
                    let mut color = Color::RGB(0, 0, 0);
                    if let Some(v) = i {
                        let hsl = HSL {
                            h: normalize(v, min, max) * 720.0 % 360.0,
                            s: 0.8,
                            l: 0.3,
                        };
                        let col = hsl.to_rgb();

                        color = Color::RGB(col.0, col.1, col.2)
                    }

                    canvas.set_draw_color(color);
                    canvas.draw_point(sdl2::rect::Point::new(x as i32, row.y as i32));
                }
                canvas.present();
            }
            canvas.present();
        }
    }
}

//maps a value in a range to be from 0 to 1
fn normalize(val: f64, min: f64, max: f64) -> f64 {
    (val - min) / (max - min)
}

fn to_col(i: u32, max: u32, bands: u32, z: Complex<f64>) -> Option<f64> {
    let sn = ((i as f64 -z.norm().log2().log2() + 4.0) / max as f64);
    //println!("{}", sn);
    if sn.is_nan() || sn > 1.0 || i == 0 {
        None
    } else {
        Some(sn)
    }
}

#[derive(Copy, Clone)]
struct WindowSpec {
    max_iter: u32,
    center: Complex<f64>,
    win_size: f64,
    img_size: (u32, u32),
    print_size: (u32, u32),
    bands: u32,
}

impl WindowSpec {
    fn get_win_dim(&self) -> (f64, f64) {
        let ar = self.img_size.1 as f64 / self.img_size.0 as f64;
        (self.win_size, self.win_size * ar)
    }
}

struct Row {
    y: u32,
    pixels: Vec<Option<f64>>,
}

impl Row {
    fn new(y: u32, width: u32) -> Self {
        Row {
            y: y,
            pixels: vec![Some(0.0); width as usize],
        }
    }
}

fn function(z: Complex<f64>, spice: Complex<f64>) -> Complex<f64> {
    //z*z*z*z*z*z*z + (Complex::new(3.0, 0.0) - spice)*z*z*z + (spice + Complex::new(1.0, 1.0))*z + spice
    z*z + spice
}

fn calc(ws: WindowSpec, num_threads: u32, print: bool) -> Vec<Row> {
    let mut sx = ws.img_size.0;
    let mut sy = ws.img_size.1;
    if print {
        sx = ws.print_size.0;
        sy = ws.print_size.1;
    }

    let progress = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    let mut rows = Vec::new();
    for i in 0..num_threads {
        let progress1 = progress.clone();
        let handle = std::thread::spawn(move || {
            let ar = sy as f64 / sx as f64;
            let window_dim = (ws.win_size, ws.win_size * ar);
            let scalex = window_dim.0 / sx as f64;
            let scaley = window_dim.1 / sy as f64;
            let mut rows = Vec::new();
            for y in 0..sy {
                if y % num_threads == i {
                    let mut row = Row::new(y, sx);
                    for x in 0..sx {
                        let cx = x as f64 * scalex - window_dim.0 / 2.0 + ws.center.re;
                        let cy = y as f64 * scaley - window_dim.1 / 2.0 + ws.center.im;

                        let c = Complex::new(cx, cy);
                        let mut z = Complex::new(0.0, 0.0);

                        let mut i = 0;

                        for t in 0..ws.max_iter {
                            if z.norm() > 128.0 {
                                break;
                            }
                            z = function(z, c);
                            i = t;
                        }
                        row.pixels[x as usize] = to_col(i, ws.max_iter, ws.bands, z);
                    }
                    rows.push(row);
                    let mut prog = progress1.lock().unwrap();
                    *prog += 1;
                    //println!("thread {} incremented prog : {}", i, *prog);
                }
            }
            rows
        });
        handles.push(handle);
    }
    let progress1 = progress.clone();
    let watcher = std::thread::spawn(move || {
        let now = Instant::now();
        let mut go = true;
        while go {
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
            let prog = progress1.lock().unwrap();
            if *prog >= sy {
                go = false;
            }
            println!(
                "{}{}/{} done calculating, {}s",
                clear::CurrentLine,
                *prog,
                sy,
                format_time(now.elapsed())
            );
            println!("{}", cursor::Up(2));
        }
        println!(
            "{} done calculating, took {}",
            clear::CurrentLine,
            format_time(now.elapsed())
        );
    });

    for handle in handles.into_iter() {
        rows.append(&mut handle.join().unwrap());
    }
    watcher.join();
    rows
}

fn format_time(d: Duration) -> String {
    let secs = d.as_secs();
    format!(
        "{:02}:{:02}:{:02}",
        secs / 3600,
        (secs / 60) % 60,
        secs % 60
    )
}

fn dump_to_file(ws: WindowSpec) {
    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(ws.print_size.0, ws.print_size.1);

    // Iterate over the coordinates and pixels of the image
    let rows = calc(ws, 3, true);

    //find the smallest value (0 if there is a None value)
    let mut min = 360.0;
    let mut max = 0.0;
    for r in rows.iter() {
        for v in r.pixels.iter() {
            if let Some(x) = v {
                if x < &min {
                    min = *x;
                }
                if x > &max {
                    max = *x;
                }
            }
        }
    }
    println!("min: {} max: {}", min, max);

    for row in rows.into_iter() {
        for (x, i) in row.pixels.into_iter().enumerate() {
            let mut color = Color::RGB(0, 0, 0);
            if let Some(v) = i {
                let hsl = HSL {
                    h: normalize(v, min, max)* 720.0 % 360.0,
                    s: 0.8,
                    l: 0.3,
                };
                let col = hsl.to_rgb();

                color = Color::RGB(col.0, col.1, col.2)
            }
            imgbuf.put_pixel(x as u32, row.y, image::Rgb([color.r, color.g, color.b]));
        }
    }

    // Save the image as “fractal.png”
    let ref mut fout = File::create(&Path::new("fractal.png")).unwrap();

    // We must indicate the image’s color type and what format to save as
    let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);
    println!("Done printing!");
}

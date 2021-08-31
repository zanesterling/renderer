extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};

mod data;
mod draw;
mod parser;
mod transform;

use crate::transform::Transform;

const SCR_W: u32 = 800;
const SCR_H: u32 = 600;
const NANOS_PER_FRAME: u32 = 1_000_000_000u32 / 60;
const TICK_DURATION: Duration = Duration::from_nanos(NANOS_PER_FRAME as u64);

const SCENE_PATH: &str = "./scenes/animate_test.scn";

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", SCR_W, SCR_H)
        .position(0, 800)
        .borderless()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator.create_texture_static(None, SCR_W, SCR_H)
        .unwrap();
    let mut draw_data: Vec<u8> = vec![0; (SCR_W * SCR_H * 4) as usize];
    let mut screen = draw::Screen::new(SCR_W as usize, SCR_H as usize);

    let mut scene = parser::load_scene(SCENE_PATH).unwrap();

    let mut loop_start = Instant::now();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut t: u64 = 0;
    let mut frames_this_second = 0;
    'running: loop {
        let tick_start = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseButtonDown {..} => {
                    scene = parser::load_scene(SCENE_PATH).unwrap();
                    loop_start = Instant::now();
                    t = 0;
                },
                _ => {}
            }
        }

        {
            draw_scene(&mut screen, &scene, t as f32 / 1_000_000_000f32).unwrap();

            // Blit!
            copy_screen_data(&screen, &mut draw_data);
            texture.update(None, &draw_data, SCR_W as usize * 4).unwrap();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
        }

        // FIXME This will overflow at some point and cause a crash.
        let tick_length = tick_start.elapsed();
        let t2 = loop_start.elapsed().as_nanos() as u64;
        frames_this_second += 1;
        if t2 / 1_000_000_000u64 > t / 1_000_000_000u64 {
            println!("second {}, fps {}", t / 1_000_000_000u64, frames_this_second);
            frames_this_second = 0;
        }
        t = t2;
        if tick_length < TICK_DURATION {
            ::std::thread::sleep(TICK_DURATION - tick_length);
        }
    }
}

fn copy_screen_data(screen: &draw::Screen, out: &mut Vec<u8>) {
    for i in 0..screen.w * screen.h {
        out[i*4    ] = screen.data[i].b;
        out[i*4 + 1] = screen.data[i].g;
        out[i*4 + 2] = screen.data[i].r;
    }
}

fn draw_scene(
    screen: &mut draw::Screen,
    scene: &parser::Scene,
    t: f32
) -> Result<(), String> {
    fn ps(p: data::Point3) -> data::PointScreen {
        data::PointScreen { x: p.x as isize, y: p.y as isize }
    }
    use crate::parser::Command;

    screen.clear();

    let mut color = data::Color::WHITE;
    let mut tr = Transform::IDENTITY;

    for cmd in &scene.commands {
        match cmd {
            Command::Point { x, y, z, rad } => {
                let rad = scene.eval_at(t, rad)?;
                let p = data::Point3{
                    x: scene.eval_at(t, x)?,
                    y: scene.eval_at(t, y)?,
                    z: scene.eval_at(t, z)?
                };
                draw::draw_point(screen, ps(tr*p), rad as usize, color)
            },
            Command::Line(p1, p2) =>
                draw::draw_line(screen, ps(tr*(*p1)), ps(tr*(*p2)), color),
            Command::Triangle(p1, p2, p3) =>
                draw::draw_triangle(
                    screen,
                    ps(tr*(*p1)), ps(tr*(*p2)), ps(tr*(*p3)),
                    color),

            Command::Scale(x, y, z) =>
                tr = Transform::scale(*x, *y, *z) * tr,
            Command::Translate(x, y, z) =>
                tr = Transform::translate(*x, *y, *z) * tr,
            Command::Identity => tr = Transform::IDENTITY,

            Command::Color(c) => color = *c,

            _ => return Err(format!("command not implemented: {:?}", cmd))
        }
    }

    Ok(())
}
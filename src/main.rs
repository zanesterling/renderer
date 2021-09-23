extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Instant};

#[allow(dead_code)]
mod data;
mod draw;
mod parser;
mod transform;
mod util;

use crate::transform::Transform;

const SCR_W: u32 = 800;
const SCR_H: u32 = 600;

const SCENE_PATH: &str = "./scenes/mesh_test.scn";

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
    let mut screen = draw::Screen::new(SCR_W as usize, SCR_H as usize);

    let mut scene = parser::load_scene(SCENE_PATH).unwrap();

    let mut loop_start = Instant::now();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut t: u64 = 0;
    let mut frames_this_second = 0;
    'running: loop {
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

        // FIXME This will overflow at some point and cause a crash.
        let t2 = loop_start.elapsed().as_nanos() as u64;
        frames_this_second += 1;
        if t2 / 1_000_000_000u64 > t / 1_000_000_000u64 {
            println!("second {}, fps {}", t / 1_000_000_000u64, frames_this_second);
            frames_this_second = 0;
        }
        t = t2;
        // No need to manually manage framerate -- SDL2 canvas locks present()
        // to 60fps.

        {
            draw_scene(&mut screen, &scene, t as f32 / 1_000_000_000f32).unwrap();

            // Blit!
            texture.update(None, &screen.data, SCR_W as usize * 4).unwrap();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
        }
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
    use crate::parser::Eval;

    screen.clear();

    let mut color = data::Color::WHITE;
    let mut tr = Transform::IDENTITY;

    for cmd in &scene.commands {
        match cmd {
            Command::Point { p, rad } => {
                let rad = rad.eval_at(t, scene)?;
                let p = p.eval_at(t, scene)?;
                draw::draw_point(screen, ps(tr*p), rad as usize, color)
            },
            Command::Line(p1, p2) => {
                let p1 = p1.eval_at(t, scene)?;
                let p2 = p2.eval_at(t, scene)?;
                draw::draw_line(screen, ps(tr*p1), ps(tr*p2), color)
            },
            Command::Triangle(p1, p2, p3) => {
                let p1 = p1.eval_at(t, scene)?;
                let p2 = p2.eval_at(t, scene)?;
                let p3 = p3.eval_at(t, scene)?;
                draw::draw_triangle(
                    screen,
                    ps(tr*p1), ps(tr*p2), ps(tr*p3),
                    color);
            },
            Command::Mesh{ points, triangles } => {
                let mut pts: Vec<data::PointScreen> = Vec::with_capacity(points.len());
                for pt in points {
                    pts.push(ps(tr*(*pt)));
                }
                for i in 0..triangles.len() / 3 {
                    draw::draw_triangle(
                        screen,
                        pts[triangles[i*3 + 0]],
                        pts[triangles[i*3 + 1]],
                        pts[triangles[i*3 + 2]],
                        color);
                }
            },

            Command::Scale(x, y, z) => {
                let x = x.eval_at(t, scene)?;
                let y = y.eval_at(t, scene)?;
                let z = z.eval_at(t, scene)?;
                tr = Transform::scale(x, y, z) * tr;
            },
            Command::Translate(x, y, z) => {
                let x = x.eval_at(t, scene)?;
                let y = y.eval_at(t, scene)?;
                let z = z.eval_at(t, scene)?;
                tr = Transform::translate(x, y, z) * tr;
            },
            Command::Rotate { theta, v } => {
                let theta = theta.eval_at(t, scene)?;
                let v = v.eval_at(t, scene)?;
                tr = Transform::rotate(theta, v) * tr;
            },
            Command::Identity => tr = Transform::IDENTITY,

            Command::Color(c) => color = *c,

            #[allow(unreachable_patterns)]
            _ => return Err(format!("command not implemented: {:?}", cmd))
        }
    }

    Ok(())
}
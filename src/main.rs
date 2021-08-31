extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod data;
mod draw;
mod parser;

const SCR_W: u32 = 800;
const SCR_H: u32 = 600;

const SCENE_PATH: &str = "./scenes/geom_test.scn";

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

    let scene = parser::load_scene(SCENE_PATH).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        {
            draw_scene(&mut screen, &scene);

            // Blit!
            copy_screen_data(&screen, &mut draw_data);
            texture.update(None, &draw_data, SCR_W as usize * 4).unwrap();
            canvas.copy(&texture, None, None).unwrap();
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn copy_screen_data(screen: &draw::Screen, out: &mut Vec<u8>) {
    for i in 0..screen.w * screen.h {
        out[i*4    ] = screen.data[i].b;
        out[i*4 + 1] = screen.data[i].g;
        out[i*4 + 2] = screen.data[i].r;
    }
}

fn draw_scene(screen: &mut draw::Screen, scene: &parser::Scene) {
    let color = data::Color::WHITE;
    fn ps(p: &data::Point3) -> data::PointScreen {
        data::PointScreen { x: p.x as usize, y: p.y as usize}
    }
    use crate::parser::Command;

    for cmd in &scene.commands {
        match cmd {
            Command::Line(p1, p2) => draw::draw_line(screen, ps(p1), ps(p2), color),

            _ => println!("command not implemented: {:?}", cmd)
        }
    }
}

fn draw_stuff(screen: &mut draw::Screen) {
    draw::draw_point(
        screen,
        data::PointScreen { x: 50, y: 50 },
        10,
        data::Color::RED
    );

    draw::draw_line(
        screen,
        data::PointScreen { x: 400, y: 300},
        data::PointScreen { x: 400-50, y: 300-100},
        data::Color::GREEN
    );
    draw::draw_line(
        screen,
        data::PointScreen { x: 400, y: 300},
        data::PointScreen { x: 400-100, y: 300-50},
        data::Color::GREEN
    );

    draw::draw_line(
        screen,
        data::PointScreen { x: 400, y: 300},
        data::PointScreen { x: 400-50, y: 300+100},
        data::Color::GREEN
    );
    draw::draw_line(
        screen,
        data::PointScreen { x: 400, y: 300},
        data::PointScreen { x: 400-100, y: 300+50},
        data::Color::GREEN
    );

    draw::draw_line(
        screen,
        data::PointScreen { x: 400, y: 300},
        data::PointScreen { x: 400+50, y: 300-100},
        data::Color::GREEN
    );
    draw::draw_line(
        screen,
        data::PointScreen { x: 400, y: 300},
        data::PointScreen { x: 400+100, y: 300-50},
        data::Color::GREEN
    );

    draw::draw_line(
        screen,
        data::PointScreen { x: 400, y: 300},
        data::PointScreen { x: 400+50, y: 300+100},
        data::Color::GREEN
    );
    draw::draw_line(
        screen,
        data::PointScreen { x: 400, y: 300},
        data::PointScreen { x: 400+100, y: 300+50},
        data::Color::GREEN
    );

    draw::draw_triangle(
        screen,
        data::PointScreen {x: 100, y: 200},
        data::PointScreen {x:  50, y: 250},
        data::PointScreen {x: 150, y: 300},
        data::Color::WHITE
    );
    draw::draw_triangle(
        screen,
        data::PointScreen {x: 100, y: 400},
        data::PointScreen {x: 150, y: 450},
        data::PointScreen {x:  50, y: 500},
        data::Color::WHITE
    );
}
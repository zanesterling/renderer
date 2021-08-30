extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod data;
mod draw;


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position(0, 0)
        .borderless()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator.create_texture_static(None, 800, 600)
        .unwrap();
    let mut draw_data: Vec<u8> = vec![0; 800 * 600 * 4];
    let mut screen = draw::Screen::new(800, 600);

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
            draw::draw_point(
                &mut screen,
                data::PointScreen { x: 50, y: 50 },
                10,
                data::Color::WHITE
            );

            // Blit!
            copy_screen_data(&screen, &mut draw_data);
            texture.update(None, &draw_data, 800*4);
            canvas.copy(&texture, None, None);
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn copy_screen_data(screen: &draw::Screen, out: &mut Vec<u8>) {
    for i in 0..screen.w * screen.h {
        out[i*4    ] = screen.data[i].r;
        out[i*4 + 1] = screen.data[i].g;
        out[i*4 + 2] = screen.data[i].b;
    }
}
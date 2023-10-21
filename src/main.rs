use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};

pub const DELTA_TIME: f64 = 1.0 / 60.0;

pub const ASTEROID_SPRITE: &[u8] = include_bytes!("assets/Asteroids#01.png");

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("FG Space Shooter", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let texture_creator = canvas.texture_creator();

    let asteroid_data =
        image::load_from_memory_with_format(&ASTEROID_SPRITE, image::ImageFormat::Png).unwrap();

    let asteroid_data = asteroid_data.into_rgba8();

    let mut asteroid_sprite = texture_creator
        .create_texture_static(
            PixelFormatEnum::RGBA32,
            asteroid_data.width(),
            asteroid_data.height(),
        )
        .unwrap();

    let width = asteroid_data.width() as usize;

    asteroid_sprite
        .update(
            None,
            asteroid_data.as_raw(),
            width * 4,
        )
        .unwrap();

    let mut should_exit = false;

    let mut time: f64 = 0.0;
    let mut time_scale: f64 = 1.0;
    let mut last_time = std::time::Instant::now();
    let mut accumulator: f64 = 0.0;

    loop {
        if should_exit {
            std::process::exit(0);
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => should_exit = true,
                _ => {}
            }
        }

        {
            // Calculate frame times
            let current_time = std::time::Instant::now();
            let frame_time = current_time.duration_since(last_time).as_secs_f64();
            last_time = current_time;
            accumulator += frame_time * time_scale;
        }

        while accumulator >= DELTA_TIME {
            // Fixed update
            {}
            accumulator -= DELTA_TIME;
            time += DELTA_TIME;
        }

        // Render
        {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            canvas.copy(&asteroid_sprite, None, None).unwrap();
            canvas.present();
        }
    }
}

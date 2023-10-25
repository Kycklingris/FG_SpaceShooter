use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

mod sprite;
mod world;

pub const DELTA_TIME: f64 = 1.0 / 60.0;

pub const ASTEROID_TEXTURE: &[u8] = include_bytes!("assets/Asteroids#01.png");
pub const SPACESHIP_TEXTURE: &[u8] = include_bytes!("assets/Spaceship#01(24x24).png");

fn main() {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem
		.window("FG Space Shooter", 800, 600)
		.position_centered()
		.resizable()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas().build().unwrap();
	let mut event_pump = sdl_context.event_pump().unwrap();

	let texture_creator = canvas.texture_creator();

	let asteroid_sprite = sprite::Sprite::new(
		&texture_creator,
		ASTEROID_TEXTURE,
		Some(sdl2::rect::Rect::new(32, 0, 48, 48)),
	);
	let spaceship_sprite = sprite::Sprite::new(&texture_creator, SPACESHIP_TEXTURE, None);

	let mut world = world::World::new(1000, 1000);

	// Game Variables

	let mut should_exit = false;

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
			// canvas
			// 	.copy_ex(
			// 		&asteroid_sprite.texture,
			// 		asteroid_sprite.rect,
			// 		None,
			// 		asteroid_sprite.angle,
			// 		None,
			// 		false,
			// 		false,
			// 	)
			// 	.unwrap();
			// canvas.present();
		}

		world.render(&mut canvas);
	}
}

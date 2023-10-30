use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

mod player;
mod sprite;
// mod world;

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
	canvas.set_logical_size(1000, 1000).unwrap();

	let mut event_pump = sdl_context.event_pump().unwrap();

	let texture_creator = canvas.texture_creator();

	let asteroid_texture = sprite::Sprite::load_texture(&texture_creator, ASTEROID_TEXTURE);
	let spaceship_texture = sprite::Sprite::load_texture(&texture_creator, SPACESHIP_TEXTURE);

	let asteroid_sprite =
		sprite::Sprite::new(&asteroid_texture, Some(Rect::new(32, 0, 48, 48)), 48, 48);
	let mut spaceship_sprite = sprite::Sprite::new(&spaceship_texture, None, 50, 50);

	let mut player = player::Player::new(&mut spaceship_sprite);

	player.set_position(500.0, 500.0);

	// Game Variables

	let mut should_exit = false;

	loop {
		if should_exit {
			std::process::exit(0);
		}

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. } => should_exit = true,
				Event::Window { win_event, .. } => match win_event {
					_ => {}
				},
				Event::MouseMotion { x, y, .. } => player.set_mouse_position(x, y),
				Event::KeyDown {
					keycode: Some(Keycode::W),
					..
				} => player.up = true,
				Event::KeyDown {
					keycode: Some(Keycode::A),
					..
				} => player.left = true,
				Event::KeyDown {
					keycode: Some(Keycode::S),
					..
				} => player.down = true,
				Event::KeyDown {
					keycode: Some(Keycode::D),
					..
				} => player.right = true,

				Event::KeyUp {
					keycode: Some(Keycode::W),
					..
				} => player.up = false,
				Event::KeyUp {
					keycode: Some(Keycode::A),
					..
				} => player.left = false,
				Event::KeyUp {
					keycode: Some(Keycode::S),
					..
				} => player.down = false,
				Event::KeyUp {
					keycode: Some(Keycode::D),
					..
				} => player.right = false,
				_ => {}
			}
		}

		{
			player.update(0.1);

			//Clear the screen
			canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
			canvas.clear();

			player.render(&mut canvas);
			canvas.present();
		}
	}
}

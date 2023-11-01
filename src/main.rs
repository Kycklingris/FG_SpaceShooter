use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;

use rand::prelude::*;

mod asteroid;
mod player;
mod sprite;

pub const ASTEROID_TEXTURE: &[u8] = include_bytes!("assets/Asteroids#01.png");
pub const SPACESHIP_TEXTURE: &[u8] = include_bytes!("assets/Spaceship#01(24x24).png");
pub const BULLET_TEXTURE: &[u8] = include_bytes!("assets/Bullet.png");

pub const LOGICAL_WIDTH: u32 = 1000;
pub const LOGICAL_HEIGHT: u32 = 1000;

pub const F64_LOGICAL_WIDTH: f64 = LOGICAL_WIDTH as f64;
pub const F64_LOGICAL_HEIGHT: f64 = LOGICAL_HEIGHT as f64;

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
	canvas
		.set_logical_size(LOGICAL_WIDTH, LOGICAL_HEIGHT)
		.unwrap();

	let mut event_pump = sdl_context.event_pump().unwrap();

	let texture_creator = canvas.texture_creator();

	let asteroid_texture = sprite::Sprite::load_texture(&texture_creator, ASTEROID_TEXTURE);
	let spaceship_texture = sprite::Sprite::load_texture(&texture_creator, SPACESHIP_TEXTURE);
	let bullet_texture = sprite::Sprite::load_texture(&texture_creator, BULLET_TEXTURE);

	let mut spaceship_sprite = sprite::Sprite::new(&spaceship_texture, None, 50, 50);

	// Game Variables

	let mut rng = rand::thread_rng();

	let mut player = player::Player::new(&mut spaceship_sprite, &bullet_texture);
	player.set_position(F64_LOGICAL_WIDTH / 2.0, F64_LOGICAL_HEIGHT / 2.0);

	let mut asteroids_rate = std::time::Duration::from_secs_f64(0.5);
	let mut last_asteroid = std::time::Instant::now();
	let mut asteroids: Vec<asteroid::Asteroid<'_>> = Vec::new();

	let mut last_update = std::time::Instant::now();

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
				Event::MouseButtonDown {
					mouse_btn: MouseButton::Left,
					x,
					y,
					..
				} => player.fire(x, y),
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
			// Updates
			let now = std::time::Instant::now();
			let time_step = now.duration_since(last_update).as_secs_f64();
			last_update = now;

			player.update(time_step);

			let mut asteroids_to_remove = Vec::new();
			for (i, asteroid) in asteroids.iter_mut().enumerate() {
				if asteroid.update(time_step) {
					asteroids_to_remove.push(i);
				}
			}

			for i in asteroids_to_remove.iter().rev() {
				asteroids.remove(*i);
				println!("removed asteroid");
			}

			// Spawn asteroid?
			if now.duration_since(last_asteroid) >= asteroids_rate {
				last_asteroid = now;

				let asteroid = asteroid::Asteroid::new(&asteroid_texture, &mut rng);
				asteroids.push(asteroid);

				println!("spawned asteroid");
			}

			//Clear the screen
			canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
			canvas.clear();

			player.render(&mut canvas);
			for asteroid in asteroids.iter() {
				asteroid.render(&mut canvas);
			}

			canvas.present();
		}
	}
}

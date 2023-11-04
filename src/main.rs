use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::render::BlendMode;

use std::io::prelude::*;

mod asteroid;
mod player;
mod sprite;
mod wave;

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

	let mut asteroid_texture = sprite::load_texture(&texture_creator, ASTEROID_TEXTURE);
	asteroid_texture.set_blend_mode(BlendMode::Blend);

	let mut spaceship_texture = sprite::load_texture(&texture_creator, SPACESHIP_TEXTURE);
	spaceship_texture.set_blend_mode(BlendMode::Blend);

	let mut bullet_texture = sprite::load_texture(&texture_creator, BULLET_TEXTURE);
	bullet_texture.set_blend_mode(BlendMode::Blend);

	// Game Variables

	let mut waves = [
		wave::Wave::new(20, 10, 5.0, 5.0),
		wave::Wave::new(30, 15, 5.0, 5.0),
		wave::Wave::new(50, 25, 7.0, 5.0),
		wave::Wave::new(75, 37, 10.0, 5.0),
		wave::Wave::new(100, 50, 10.0, 5.0),
	];
	let mut current_wave_index: usize = 0;

	let mut rng = rand::thread_rng();

	let mut player = player::Player::new(&spaceship_texture, 50, 50);
	player.set_position(F64_LOGICAL_WIDTH / 2.0, F64_LOGICAL_HEIGHT / 2.0);

	let mut asteroids = asteroid::Asteroids::new(&asteroid_texture);
	let mut bullets = player::bullet::Bullets::new(&bullet_texture);

	waves[current_wave_index].start(&mut rng, &mut asteroids);

	let mut last_update = std::time::Instant::now();

	let mut should_exit = false;

	let mut frames: u64 = 0;

	loop {
		if should_exit {
			let mut file = std::fs::File::create("frames.txt").unwrap();
			write!(&mut file, "{:?}", frames).unwrap();
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
				} => player.fire(x, y, &mut bullets),
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

			if cfg!(feature = "benchmark") {
				player.fire(LOGICAL_WIDTH as i32 / 2, 0, &mut bullets);
			}

			player.update(time_step);
			asteroids.update(time_step);
			bullets.update(time_step);

			// Check collisions.
			{
				let mut bullets_to_remove = Vec::new();
				let mut asteroids_to_remove = Vec::new();

				'asteroid: for (asteroid_index, asteroid_collider) in
					asteroids.colliders.iter().enumerate()
				{
					for (bullet_index, bullet_collider) in bullets.colliders.iter().enumerate() {
						if !bullets_to_remove.contains(&bullet_index)
							&& asteroid_collider.check_circle_overlap(&bullet_collider)
						{
							bullets_to_remove.push(bullet_index);
							asteroids_to_remove.push(asteroid_index);
							continue 'asteroid;
						}
					}
					
					if asteroid_collider.check_circle_overlap(&player.collider) {
						asteroids_to_remove.push(asteroid_index);
						player.update_health(-1);
					}
				}

				for i in bullets_to_remove.iter() {
					bullets.remove_at(*i);
				}

				for i in asteroids_to_remove.iter() {
					asteroids.remove_at(*i);
				}
			}

			// Handle wave.
			{
				if waves[current_wave_index].update(&mut rng, &mut asteroids) {
					if current_wave_index < waves.len() - 1 {
						current_wave_index += 1;
					} else if cfg!(feature = "benchmark") {
						should_exit = true;
					}

					waves[current_wave_index].start(&mut rng, &mut asteroids);
				}
			}

			//Clear the screen
			canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
			canvas.clear();

			bullets.render(&mut canvas);
			asteroids.render(&mut canvas);
			player.render(&mut canvas);

			canvas.present();

			frames += 1;
		}
	}
}

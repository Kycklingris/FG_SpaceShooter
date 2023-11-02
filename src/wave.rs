use rand::rngs::ThreadRng;
use sdl2::render::Texture;

use crate::asteroid::Asteroid;

pub struct Wave {
	asteroid_amount: u32,
	asteroid_initial_amount: u32,
	asteroids_spawned: u32,
	time_between_asteroid: f64,
	time_til_next: std::time::Duration,
	last_asteroid: std::time::Instant,
}

impl Wave {
	pub fn new(
		asteroid_amount: u32,
		asteroid_initial_amount: u32,
		spawn_duration: f64,
		time_til_next: f64,
	) -> Self {
		Self {
			asteroid_amount,
			asteroid_initial_amount,
			asteroids_spawned: 0,
			time_between_asteroid: spawn_duration
				/ (asteroid_amount as f64 - asteroid_initial_amount as f64),
			time_til_next: std::time::Duration::from_secs_f64(time_til_next),
			last_asteroid: std::time::Instant::now(),
		}
	}

	pub fn start<'a>(
		&mut self,
		asteroid_texture: &'a Texture,
		rng: &mut ThreadRng,
		asteroids: &mut Vec<Asteroid<'a>>,
	) {
		println!("Spawning Wave!");

		for _ in 0..self.asteroid_initial_amount {
			asteroids.push(Asteroid::new(asteroid_texture, rng));
		}

		self.last_asteroid = std::time::Instant::now();
		self.asteroids_spawned += self.asteroid_initial_amount;
	}

	pub fn update<'a>(
		&mut self,
		asteroid_texture: &'a Texture,
		rng: &mut ThreadRng,
		asteroids: &mut Vec<Asteroid<'a>>,
	) -> bool {
		let now = std::time::Instant::now();

		if self.asteroids_spawned >= self.asteroid_amount {
			if now.duration_since(self.last_asteroid) >= self.time_til_next {
				return true;
			}
			return false;
		}

		let duration_since = now.duration_since(self.last_asteroid).as_secs_f64();
		let spawn_amount = f64::floor(duration_since / self.time_between_asteroid) as u32;

		for _ in 0..spawn_amount {
			asteroids.push(Asteroid::new(asteroid_texture, rng));
			self.asteroids_spawned += 1;
			self.last_asteroid = now;
		}

		false
	}
}

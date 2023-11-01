use rand::prelude::*;
use sdl2::render::WindowCanvas;

// use crate::LOGICAL_HEIGHT;
// use crate::LOGICAL_WIDTH;
use crate::F64_LOGICAL_HEIGHT;
use crate::F64_LOGICAL_WIDTH;

pub struct Asteroid<'a> {
	pub sprite: crate::sprite::Sprite<'a>,
	speed: f64,
	direction: (f64, f64),
}

impl<'a> Asteroid<'a> {
	pub fn new(
		texture: &'a sdl2::render::Texture<'a>,
		rng: &mut ThreadRng,

	) -> Self {
		let on_width = rng.gen_bool(0.5);
		let position = if on_width {
			let y = if rng.gen_bool(0.5) {
				F64_LOGICAL_HEIGHT + 50.0
			} else {
				-50.0
			};

			(rng.gen_range(-50.0..=(F64_LOGICAL_WIDTH + 50.0)), y)
		} else {
			let x = if rng.gen_bool(0.5) {
				F64_LOGICAL_WIDTH + 50.0
			} else {
				-50.0
			};

			(x, rng.gen_range(-50.0..=(F64_LOGICAL_HEIGHT + 50.0)))
		};

		let target = (
			rng.gen_range((F64_LOGICAL_WIDTH * 0.10)..=(F64_LOGICAL_WIDTH * 0.90)),
			rng.gen_range((F64_LOGICAL_HEIGHT * 0.10)..=(F64_LOGICAL_HEIGHT * 0.90)),
		);

		let mut direction = (
			target.0 - position.0,
			target.1 - position.1,
		);

		let length = f64::sqrt((direction.0 * direction.0) + (direction.1 * direction.1));

		direction = (
			direction.0 / length,
			direction.1 / length,
		);

		let speed = rng.gen_range(50.0..=400.0);

		let mut sprite = crate::sprite::Sprite::new(texture, Some(sdl2::rect::Rect::new(32, 0, 48, 48)), 64, 64);		
		sprite.set_position(position.0, position.1);

		Self {
			sprite,
			speed,
			direction,
		}
	}

	#[inline]
	pub fn update(&mut self, time_step: f64) -> bool {
		let movement = (
			self.direction.0 * self.speed * time_step,
			self.direction.1 * self.speed * time_step,
		);

		self.sprite.update_position(movement.0, movement.1);

		let new_position = self.sprite.get_position();

		self.sprite.rotation += self.speed / 5.0 * time_step;

		if new_position.0 < -100.0
			|| new_position.0 > crate::F64_LOGICAL_WIDTH + 100.0
			|| new_position.1 < -100.0
			|| new_position.1 > crate::F64_LOGICAL_HEIGHT + 100.0
		{
			return true;
		}

		false
	}

	#[inline]
	pub fn render(&self, canvas: &mut WindowCanvas) {
		self.sprite.render(canvas);
	}
}

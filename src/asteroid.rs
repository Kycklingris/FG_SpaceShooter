use rand::prelude::*;
use sdl2::render::WindowCanvas;

use crate::F64_LOGICAL_HEIGHT;
use crate::F64_LOGICAL_WIDTH;
use crate::sprite::Collider;

pub struct Asteroids<'a> {
	texture: &'a sdl2::render::Texture<'a>,
	pub colliders: Vec<crate::sprite::Collider>,
	movements: Vec<AsteroidMovement>
}

impl<'a> Asteroids<'a> {
	pub fn new(texture: &'a sdl2::render::Texture<'a>) -> Self {
		Self {
			texture,
			colliders: Vec::new(),
			movements: Vec::new(),
		}
	}

	

	pub fn new_asteroid(&mut self, rng: &mut ThreadRng) -> usize {
		let (movement, position) = AsteroidMovement::new(rng);
		let mut collider = Collider::new(64, 64);
		collider.set_position(position.0, position.1);
		
		self.movements.push(movement);
		self.colliders.push(collider);

		self.movements.len() - 1
	}

	#[inline]
	pub fn update(&mut self, time_step: f64) {
		let mut indices_to_remove = Vec::new();

		for (i, (movement, collider)) in self.movements.iter().zip(self.colliders.iter_mut()).enumerate() {
			let movement_change = (
				movement.direction.0 * movement.speed * time_step,
				movement.direction.1 * movement.speed * time_step,
			);

			collider.update_position(movement_change.0, movement_change.1);

			let new_position = collider.get_position();

			collider.rotation += movement.speed / 5.0 * time_step;

			if new_position.0 < -100.0
				|| new_position.0 > crate::F64_LOGICAL_WIDTH + 100.0
				|| new_position.1 < -100.0
				|| new_position.1 > crate::F64_LOGICAL_HEIGHT + 100.0
			{
				indices_to_remove.push(i);
			}
		}
	}

	#[inline]
	pub fn remove_at(&mut self, index: usize) {
		self.colliders.remove(index);
		self.movements.remove(index);
	}

	#[inline]
	pub fn render(&mut self, canvas: &mut WindowCanvas) {
		for collider in self.colliders.iter() {
			collider.render(canvas, self.texture);
		}
	}
}

pub struct AsteroidMovement {
	speed: f64,
	direction: (f64, f64),
}

impl AsteroidMovement {
	pub fn new(rng: &mut ThreadRng) -> (Self, (f64, f64)) {
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

		let mut direction = (target.0 - position.0, target.1 - position.1);

		let length = f64::sqrt((direction.0 * direction.0) + (direction.1 * direction.1));

		direction = (direction.0 / length, direction.1 / length);

		let speed = rng.gen_range(50.0..=400.0);


		(Self {
			speed,
			direction,
		}, position)
	}
}

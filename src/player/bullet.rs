use sdl2::render::WindowCanvas;

use crate::sprite::Collider;

pub struct Bullets<'a> {
	texture: &'a sdl2::render::Texture<'a>,
	pub colliders: Vec<crate::sprite::Collider>,
	movements: Vec<BulletMovement>
}

impl<'a> Bullets<'a> {
	pub fn new(texture: &'a sdl2::render::Texture<'a>) -> Self {
		Self {
			texture,
			colliders: Vec::new(),
			movements: Vec::new(),
		}
	}

	pub fn new_bullet(&mut self, position: (f64, f64), direction: (f64, f64), speed: f64) -> usize {
		let mut collider = Collider::new(32, 32);
		collider.set_position(position.0, position.1);

		let movement = BulletMovement {
			speed,
			direction,
		};

		self.colliders.push(collider);
		self.movements.push(movement);

		return self.colliders.len() - 1;
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

pub struct BulletMovement {
	speed: f64,
	direction: (f64, f64),
}
use crate::sprite::Sprite;

pub struct Bullet<'a> {
	pub sprite: Sprite<'a>,
	speed: f64,
	direction: (f64, f64),
}

impl<'a> Bullet<'a> {
	pub fn new(
		texture: &'a sdl2::render::Texture<'a>,
		position: (f64, f64),
		direction: (f64, f64),
		speed: f64,
	) -> Self {
		let mut sprite = Sprite::new(texture, None, 32, 32);
		sprite.set_position(position.0, position.1);

		Self {
			sprite,
			speed,
			direction,
		}
	}

	#[inline(always)]
	pub fn update(&mut self, time_step: f64) -> bool {
		let movement = (
			self.direction.0 * self.speed * time_step,
			self.direction.1 * self.speed * time_step,
		);

		self.sprite.update_position(movement.0, movement.1);

		let new_position = self.sprite.get_position();

		if new_position.0 < -100.0
			|| new_position.0 > crate::F64_LOGICAL_WIDTH + 100.0
			|| new_position.1 < -100.0
			|| new_position.1 > crate::F64_LOGICAL_HEIGHT + 100.0
		{
			return true;
		}

		false
	}

	pub fn render(&self, canvas: &mut sdl2::render::WindowCanvas) {
		self.sprite.render(canvas);
	}
}

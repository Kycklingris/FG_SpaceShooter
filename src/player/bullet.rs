use crate::sprite::Sprite;

pub struct Bullet<'a> {
	sprite: Sprite<'a>,
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

	pub fn update(&mut self, time_step: f64) {
		let movement = (
			self.direction.0 * self.speed * time_step,
			self.direction.1 * self.speed * time_step,
		);

		self.sprite.update_position(movement.0, movement.1);
	}

	pub fn render(&self, canvas: &mut sdl2::render::WindowCanvas) {
		self.sprite.render(canvas);
	}
}

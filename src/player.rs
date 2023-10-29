use sdl2::render::WindowCanvas;

pub struct Player<'a> {
	sprite: &'a mut crate::sprite::Sprite<'a>,
	pub up: bool,
	pub left: bool,
	pub right: bool,
	pub down: bool,
	speed: f64,
	mouse_position: sdl2::rect::Point,
}

impl<'a> Player<'a> {
	pub fn new(sprite: &'a mut crate::sprite::Sprite<'a>) -> Self {
		Self {
			sprite,
			up: false,
			left: false,
			right: false,
			down: false,
			speed: 1.0,
			mouse_position: sdl2::rect::Point::new(0, 0),
		}
	}

	#[inline]
	pub fn update(&mut self, time_step: f64) {
		let mut direction = (0.0, 0.0);

		if self.up {
			direction.1 -= 1.0;
		}
		if self.left {
			direction.0 -= 1.0;
		}
		if self.right {
			direction.0 += 1.0;
		}
		if self.down {
			direction.1 += 1.0;
		}

		if direction.0 == 0.0 && direction.1 == 0.0 {
			return;
		}

		let length = f64::abs(f64::sqrt(
			(direction.0 * direction.0) + (direction.1 * direction.1),
		));

		let movement: (f64, f64) = (
			(direction.0 / length) * self.speed * time_step,
			(direction.1 / length) * self.speed * time_step,
		);

		self.sprite.update_position(movement.0, movement.1);
	}

	#[inline]
	pub fn render(&self, canvas: &mut WindowCanvas) {
		canvas
			.copy_ex(
				self.sprite.texture,
				self.sprite.src_rect,
				self.sprite.get_dst_rect(),
				self.sprite.rotation,
				None,
				false,
				false,
			)
			.unwrap();
	}
}

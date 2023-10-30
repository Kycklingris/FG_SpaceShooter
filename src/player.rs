use sdl2::render::WindowCanvas;

pub const PI: f64 = 3.1415926535897932384626433832795028841971693993751058209749445923078164062;

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

		if direction.0 != 0.0 || direction.1 != 0.0 {
			let length = f64::abs(f64::sqrt(
				(direction.0 * direction.0) + (direction.1 * direction.1),
			));

			let movement: (f64, f64) = (
				(direction.0 / length) * self.speed * time_step,
				(direction.1 / length) * self.speed * time_step,
			);

			self.sprite.update_position(movement.0, movement.1);
		}

		// Rotate towards the mouse.
		// https://stackoverflow.com/a/507879
		self.sprite.rotation = f64::atan2(
			self.mouse_position.x as f64 - self.sprite.get_position().0,
			(self.mouse_position.y as f64 - self.sprite.get_position().1) * -1.0,
		) * 180.0 / PI;
	}

	#[inline]
	pub fn set_position(&mut self, x: f64, y: f64) {
		self.sprite.set_position(x, y);
	}

	#[inline]
	pub fn set_mouse_position(&mut self, x: i32, y: i32) {
		self.mouse_position = sdl2::rect::Point::new(x, y);
	}

	#[inline]
	pub fn render(&self, canvas: &mut WindowCanvas) {
		self.sprite.render(canvas);
	}
}

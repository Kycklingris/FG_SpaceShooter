use sdl2::render::WindowCanvas;

pub const PI: f64 = 3.1415926535897932384626433832795028841971693993751058209749445923078164062;

pub mod bullet;

pub struct Player<'a> {
	pub sprite: &'a mut crate::sprite::Sprite<'a>,
	bullet_texture: &'a sdl2::render::Texture<'a>,
	pub up: bool,
	pub left: bool,
	pub right: bool,
	pub down: bool,
	speed: f64,
	mouse_position: sdl2::rect::Point,
	fire_rate: std::time::Duration,
	last_fire: std::time::Instant,
	pub bullets: Vec<bullet::Bullet<'a>>,
	health: i32,
}

impl<'a> Player<'a> {
	pub fn new(
		sprite: &'a mut crate::sprite::Sprite<'a>,
		bullet_texture: &'a sdl2::render::Texture<'a>,
	) -> Self {
		Self {
			sprite,
			bullet_texture,
			up: false,
			left: false,
			right: false,
			down: false,
			speed: 150.0,
			mouse_position: sdl2::rect::Point::new(0, 0),
			fire_rate: std::time::Duration::from_secs_f64(0.2),
			last_fire: std::time::Instant::now(),
			bullets: Vec::new(),
			health: 10,
		}
	}

	#[inline]
	pub fn update(&mut self, time_step: f64) {
		// Handle bullets
		let mut to_remove = Vec::new();
		for (i, bullet) in self.bullets.iter_mut().enumerate() {
			if bullet.update(time_step) {
				to_remove.push(i);
			}
		}

		for i in to_remove.iter().rev() {
			self.bullets.remove(*i);
		}

		// Get input direction
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

		// Move the player
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
		// https://stackoverflow.com/a/6247163
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
	pub fn update_health(&mut self, change: i32) {
		self.health += change;

		if self.health <= 0 {
			println!("Dead!");
			std::process::exit(0);
		}
	}

	#[inline]
	pub fn fire(&mut self, x: i32, y: i32) {
		let now = std::time::Instant::now();

		if now.duration_since(self.last_fire) < self.fire_rate {
			return;
		}

		self.last_fire = now;
		let mut direction = (
			x as f64 - self.sprite.get_position().0,
			y as f64 - self.sprite.get_position().1,
		);

		let length = f64::sqrt((direction.0 * direction.0) + (direction.1 * direction.1));

		direction = (direction.0 / length, direction.1 / length);

		let bullet = bullet::Bullet::new(
			self.bullet_texture,
			self.sprite.get_position(),
			direction,
			500.0,
		);

		self.bullets.push(bullet);
	}

	#[inline]
	pub fn render(&self, canvas: &mut WindowCanvas) {
		for bullet in self.bullets.iter() {
			bullet.render(canvas);
		}

		self.sprite.render(canvas);
	}
}

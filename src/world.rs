use sdl2::{rect::Rect, render::WindowCanvas};

pub struct World {
	pub width: u32,
	pub height: u32,
	pub offset: sdl2::rect::Point,
	pub scale: f64,
}

impl World {
	pub fn new(width: u32, height: u32) -> Self {
		Self {
			width,
			height,
			offset: sdl2::rect::Point::new(0, 0),
			scale: 0.0,
		}
	}

	pub fn resized(&mut self, canvas: &mut WindowCanvas) {
		//Clear the screen
		// canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
		// canvas.clear();

		let (canvas_width, canvas_height) = canvas.output_size().unwrap();

		if (canvas_width as i32 - self.width as i32) < (canvas_height as i32 - self.height as i32) {
			self.scale = canvas_width as f64 / self.width as f64;
			self.offset = sdl2::rect::Point::new(
				0,
				(canvas_height as i32 - (self.height as f64 * self.scale) as i32) / 2,
			);
		} else {
			self.scale = canvas_height as f64 / self.height as f64;
			self.offset = sdl2::rect::Point::new(
				(canvas_width as i32 - (self.width as f64 * self.scale) as i32) / 2,
				0,
			);
		};

		canvas.set_clip_rect(Some(Rect::new(
			self.offset.x,
			self.offset.y,
			(self.width as f64 * self.scale) as u32,
			(self.height as f64 * self.scale) as u32,
		)));

		// canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
		// canvas
		// 	.fill_rect(Rect::new(0, 0, canvas_width, canvas_height))
		// 	.unwrap();

		// canvas.present();
	}
}

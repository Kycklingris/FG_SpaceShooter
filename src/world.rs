use sdl2::{rect::Rect, render::WindowCanvas};

pub struct World {
	pub width: u32,
	pub height: u32,
}

impl World {
	pub fn new(width: u32, height: u32) -> Self {
		Self { width, height }
	}

	pub fn render(&self, canvas: &mut WindowCanvas) {
		//Clear the screen
		canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
		canvas.clear();

		let (canvas_width, canvas_height) = canvas.output_size().unwrap();

		let world_scale = if (canvas_width as i32 - self.width as i32)
			< (canvas_height as i32 - self.height as i32)
		{
			let scale = canvas_width as f32 / self.width as f32;
			canvas.set_clip_rect(Some(Rect::new(
				0,
				(canvas_height as i32 - (self.height as f32 * scale) as i32) / 2,
				canvas_width,
				(self.height as f32 * scale) as u32,
			)));

			scale
		} else {
			let scale = canvas_height as f32 / self.height as f32;
			canvas.set_clip_rect(Some(Rect::new(
				(canvas_width as i32 - (self.width as f32 * scale) as i32) / 2,
				0,
				(self.width as f32 * scale) as u32,
				canvas_height,
			)));

			scale
		};

		canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
		canvas
			.fill_rect(Rect::new(0, 0, canvas_width, canvas_height))
			.unwrap();

		canvas.present();
	}
}

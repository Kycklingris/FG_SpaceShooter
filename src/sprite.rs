use sdl2::pixels::PixelFormatEnum;

pub struct Sprite<'a> {
	pub texture: &'a sdl2::render::Texture<'a>,
	pub src_rect: Option<sdl2::rect::Rect>,
	width: u32,
	height: u32,
	position: (f64, f64),
	pub rotation: f64,
}

impl<'a> Sprite<'a> {
	pub fn new(
		texture: &'a sdl2::render::Texture<'a>,
		src_rect: Option<sdl2::rect::Rect>,
		width: u32,
		height: u32,
	) -> Self {
		Self {
			texture,
			src_rect,
			width,
			height,
			position: (0.0, 0.0),
			rotation: 0.0,
		}
	}

	#[inline]
	pub fn update_position(&mut self, x: f64, y: f64) {
		self.position.0 += x;
		self.position.1 += y;
	}

	#[inline]
	pub fn set_position(&mut self, x: f64, y: f64) {
		self.position = (x, y);
	}

	#[inline]
	pub fn get_position(&self) -> (f64, f64) {
		self.position
	}

	#[inline]
	pub fn get_dst_rect(&self) -> sdl2::rect::Rect {
		sdl2::rect::Rect::new(
			self.position.0 as i32 - self.width as i32 / 2,
			self.position.1 as i32 - self.height as i32 / 2,
			self.width,
			self.height,
		)
	}

	pub fn load_texture<Context>(
		texture_creator: &'a sdl2::render::TextureCreator<Context>,
		image_buffer: &[u8],
	) -> sdl2::render::Texture<'a> {
		let data =
			image::load_from_memory_with_format(image_buffer, image::ImageFormat::Png).unwrap();

		let data = data.into_rgba8();

		let mut texture = texture_creator
			.create_texture_static(
				PixelFormatEnum::RGBA32, // I could not tell you why RGBA32 works but RGBA8888 only gives the red channel.
				data.width(),
				data.height(),
			)
			.unwrap();

		let width = data.width() as usize;

		texture.update(None, data.as_raw(), width * 4).unwrap();

		texture
	}
}
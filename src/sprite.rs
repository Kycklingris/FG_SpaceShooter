use sdl2::pixels::PixelFormatEnum;

pub struct Sprite<'a> {
	pub texture: sdl2::render::Texture<'a>,
	pub rect: Option<sdl2::rect::Rect>,
	pub angle: f64,
}

impl<'a> Sprite<'a> {
	pub fn new<Context>(
		texture_creator: &'a sdl2::render::TextureCreator<Context>,
		image_buffer: &[u8],
		rect: Option<sdl2::rect::Rect>,
	) -> Self {
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

		Self {
			texture,
			rect,
			angle: 0.0,
		}
	}
}

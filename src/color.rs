#[derive(Debug)]
pub struct Color {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32 
}

impl Color {
	pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
		Color {
			r: r,
			g: g,
			b: b,
			a: a
		}
	}

	pub fn as_array(&self) -> [f32; 4] {
		[self.r, self.g, self.b, self.a]
	}
}
use crate::color::Color;

pub struct ATracer {
    width: u32,
    height: u32,
    color_buffer: Vec<Color>,
}

impl ATracer {
    pub fn new(width: u32, height: u32) -> Self {
        ATracer {
            width,
            height,
            color_buffer: vec![Default::default(); (width * height) as usize],
        }
    }

    pub fn update(&mut self) {}

    /// Resize the buffers of the tracer
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.color_buffer = vec![Default::default(); (width * height) as usize];
    }

    /// Draw the current color buffer of the tracer to the supplied frame
    pub fn draw(&mut self, frame: &mut [u8]) {
        assert!(
            self.color_buffer.len() * 4 == frame.len(),
            "Size of color buffer and supplied frame didn't match in draw call."
        );

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            pixel.copy_from_slice(&(<[u8; 4]>::from(self.color_buffer[i])))
        }
    }
}

use crate::{color::Color, scene::Scene};

pub struct ATracer {
    width: u32,
    height: u32,
    color_buffer: Vec<Color>,
    render_status: RenderStatus,
    scene: Scene,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RenderStatus {
    Ready,
    Rendering,
    Finished,
}

impl ATracer {
    pub fn new(width: u32, height: u32) -> Self {
        ATracer {
            width,
            height,
            color_buffer: vec![Default::default(); (width * height) as usize],
            render_status: RenderStatus::Ready,
            scene: Scene::default(),
        }
    }

    pub fn update(&mut self) {}

    /// Resize and clear all buffers of the tracer
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.color_buffer = vec![Default::default(); (width * height) as usize];
    }

    /// Start rendering the current scene with the current settings to the color buffer
    pub fn start_render(&mut self) {
        if self.render_status == RenderStatus::Ready {
            for i in 0..self.width {
                for j in 0..self.height {
                    let h = i as f32 / (self.width - 1) as f32;
                    let v = 1.0 - (j as f32 / (self.height - 1) as f32);

                    let color = self.scene.render_pixel(h, v);
                    let index = (j * self.width + i) as usize;
                    self.color_buffer[index] = color;
                }
            }
        }
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

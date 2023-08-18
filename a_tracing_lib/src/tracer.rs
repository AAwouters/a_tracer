use crate::{
    color::{Color, BLACK},
    sampling::{RegularSampler, SampleGenerator},
    scene::Scene,
};

pub struct ATracer {
    width: u32,
    height: u32,
    color_buffer: Vec<Color>,
    render_status: RenderStatus,
    scene: Scene,
    sampler: Box<dyn SampleGenerator>,
    number_of_samples: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RenderStatus {
    NeedsQuickrender,
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
            render_status: RenderStatus::NeedsQuickrender,
            scene: Scene::default(),
            sampler: Box::new(RegularSampler::new(3)),
            number_of_samples: 9,
        }
    }

    pub fn update(&mut self) {
        match self.render_status {
            RenderStatus::NeedsQuickrender => self.quick_render(),
            RenderStatus::Ready => {}
            RenderStatus::Rendering => {}
            RenderStatus::Finished => {}
        }
    }

    /// Resize and clear all buffers of the tracer
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.color_buffer = vec![Default::default(); (width * height) as usize];
        self.scene
            .camera
            .set_aspect_ratio(width as f32 / height as f32);

        self.render_status = RenderStatus::NeedsQuickrender;
    }

    /// Start rendering the current scene with the current settings to the color buffer
    pub fn start_render(&mut self) {
        if self.render_status == RenderStatus::Ready {
            for i in 0..self.width {
                for j in 0..self.height {
                    let mut color = BLACK;

                    for s in 0..self.number_of_samples {
                        let sample = self.sampler.get_sample(s);

                        let h = (i as f32 + sample.x) / (self.width - 1) as f32;
                        let v = 1.0 - ((j as f32 + sample.y) / (self.height - 1) as f32);

                        let sample_color = self.render_pixel(h, v);

                        color += sample_color;
                    }

                    color /= self.number_of_samples as f32;

                    let index = (j * self.width + i) as usize;
                    self.color_buffer[index] = color;
                }
            }
        }

        self.render_status = RenderStatus::Finished;
    }

    fn render_pixel(&self, h: f32, v: f32) -> Color {
        let scene = &self.scene;
        let ray = scene.camera.get_ray(h, v);
        scene.trace_ray(&ray)
    }

    pub fn quick_render(&mut self) {
        for i in 0..self.width {
            for j in 0..self.height {
                let h = i as f32 / (self.width - 1) as f32;
                let v = 1.0 - (j as f32 / (self.height - 1) as f32);

                let color = self.quick_render_pixel(h, v);

                let index = (j * self.width + i) as usize;
                self.color_buffer[index] = color;
            }
        }

        self.render_status = RenderStatus::Ready;
    }

    fn quick_render_pixel(&self, h: f32, v: f32) -> Color {
        let scene = &self.scene;
        let ray = scene.camera.get_ray(h, v);
        scene.first_hit_color(&ray)
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

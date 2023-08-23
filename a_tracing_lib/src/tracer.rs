use crate::{
    color::{Color, BLACK},
    sampling::{RegularSampler, SampleGenerator},
    scene::Scene,
};

pub struct ATracer {
    render_settings: RenderSettings,
    color_buffer: Vec<Color>,
    render_status: RenderStatus,
    scene: Scene,
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
        let render_settings = RenderSettings {
            resolution: Resolution { width, height },
            number_of_samples: 9,
            sampler: Box::new(RegularSampler::new(3)),
        };

        ATracer {
            render_settings,
            color_buffer: vec![Default::default(); (width * height) as usize],
            render_status: RenderStatus::NeedsQuickrender,
            scene: Scene::default(),
        }
    }

    pub fn get_scene_mut(&mut self) -> &mut Scene {
        self.render_status = RenderStatus::NeedsQuickrender;
        &mut self.scene
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
        self.render_settings.resolution.width = width;
        self.render_settings.resolution.height = height;
        self.color_buffer = vec![Default::default(); (width * height) as usize];
        self.get_scene_mut()
            .camera
            .set_aspect_ratio(width as f32 / height as f32);
    }

    /// Start rendering the current scene with the current settings to the color buffer
    pub fn start_render(&mut self) {
        if self.render_status != RenderStatus::Ready {
            return;
        }

        let width = self.render_settings.resolution.width;
        let height = self.render_settings.resolution.height;

        for i in 0..width {
            for j in 0..height {
                let mut color = BLACK;
                let nb_samples = self.render_settings.number_of_samples;

                for s in 0..nb_samples {
                    let sample = self.render_settings.sampler.get_sample(s);

                    let h = (i as f32 + sample.x) / (width - 1) as f32;
                    let v = 1.0 - ((j as f32 + sample.y) / (height - 1) as f32);

                    let sample_color = self.render_pixel(h, v);

                    color += sample_color;
                }

                color /= nb_samples as f32;

                let index = (j * width + i) as usize;
                self.color_buffer[index] = color;
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
        let width = self.render_settings.resolution.width;
        let height = self.render_settings.resolution.height;

        for i in 0..width {
            for j in 0..height {
                let h = i as f32 / (width - 1) as f32;
                let v = 1.0 - (j as f32 / (height - 1) as f32);

                let color = self.quick_render_pixel(h, v);

                let index = (j * width + i) as usize;
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

pub struct RenderSettings {
    pub resolution: Resolution,
    pub number_of_samples: u32,
    pub sampler: Box<dyn SampleGenerator>,
}

pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

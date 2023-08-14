use glam::Vec2;

pub trait SampleGenerator {
    /// generates the requested sample with x and y in range [-0.5, 0.5] if there are samples left
    fn get_sample(&self, sample_number: u32) -> Vec2;
}

pub struct RegularSampler {
    samples_per_side: u32,
}

impl RegularSampler {
    pub fn new(samples_per_side: u32) -> Self {
        assert!(samples_per_side > 0);

        Self { samples_per_side }
    }
}

impl SampleGenerator for RegularSampler {
    fn get_sample(&self, sample_number: u32) -> Vec2 {
        let sample_number = sample_number % (self.samples_per_side * self.samples_per_side);

        let stride = 1.0 / (self.samples_per_side as f32 + 1.0);

        let row = sample_number / self.samples_per_side + 1;
        let column = sample_number % self.samples_per_side + 1;

        let x = stride * column as f32 - 0.5;
        let y = stride * row as f32 - 0.5;

        Vec2::new(x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::vec2;

    #[test]
    #[should_panic]
    fn test_regular_sampler_0() {
        let _sampler = RegularSampler::new(0);
    }

    #[test]
    fn test_regular_sampler_1_zero() {
        let sampler = RegularSampler::new(1);
        assert_eq!(sampler.get_sample(0), vec2(0.0, 0.0));
    }

    #[test]
    fn test_regular_sampler_1_any() {
        let sampler = RegularSampler::new(1);
        assert_eq!(sampler.get_sample(7), vec2(0.0, 0.0));
    }

    #[test]
    fn test_regular_sampler_4_zero() {
        let sampler = RegularSampler::new(4);
        assert_eq!(sampler.get_sample(0), vec2(-0.3, -0.3));
    }

    #[test]
    fn test_regular_sampler_4_three() {
        let sampler = RegularSampler::new(4);
        assert_eq!(sampler.get_sample(3), vec2(0.3, -0.3));
    }

    #[test]
    fn test_regular_sampler_4_fifteen() {
        let sampler = RegularSampler::new(4);
        assert_eq!(sampler.get_sample(15), vec2(0.3, 0.3));
    }
}

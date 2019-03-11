mod ffi;

pub struct HaltonSampler {
    hs: ffi::HaltonSampler,
}

impl HaltonSampler {
    pub fn new_faure() -> HaltonSampler {
        let hs = unsafe {
            ffi::Halton_sampler_create_faure()
        };

        HaltonSampler {
            hs
        }
    }

    pub fn get_num_dimensions(&self) -> u32 {
        unsafe {
            ffi::Halton_sampler_get_num_dimensions(self.hs)
        }
    }

    pub fn sample(&self, dimension: u32, index: u32) -> f32 {
        unsafe {
            ffi::Halton_sampler_sample(self.hs, dimension, index)
        }
    }
}

impl Drop for HaltonSampler {
    fn drop(&mut self) {
        unsafe {
            ffi::Halton_sampler_destroy(self.hs)
        }
    }
}

pub fn sobol_sample(index: u64, dimension: u32, scramble: u32) -> f32 {
    unsafe { ffi::sobol_sample(index, dimension, scramble) }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let hs = crate::HaltonSampler::new_faure();
        let samples = (0..4).map(|s| {
            (hs.sample(0, s), hs.sample(1, s))
        }).collect::<Vec<_>>();
        assert_eq!(samples, [
            (0.0, 0.0), 
            (0.5, 0.3333333), 
            (0.25, 0.6666666), 
            (0.75, 0.1111111)]);

        use crate::sobol_sample;
        let samples = (0..16).map(|s| {
            println!("{} {}", sobol_sample(s, 0, 0), sobol_sample(s, 1, 0));
        }).collect::<Vec<_>>();
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct HaltonSampler_api {
    _unused: [u8; 0],
}
pub(crate) type HaltonSampler = *mut HaltonSampler_api;

#[link(name="halton", kind="static")]
extern "C" {
pub(crate) fn Halton_sampler_create_faure() -> HaltonSampler;
pub(crate) fn Halton_sampler_get_num_dimensions(hs: HaltonSampler) -> u32;
pub(crate) fn Halton_sampler_sample(hs: HaltonSampler, dimension: u32, index: u32) -> f32;
pub(crate) fn Halton_sampler_destroy(hs: HaltonSampler);
}

#[link(name="sobol", kind="static")]
extern "C" {
pub fn sobol_sample(index: u64, dimension: u32, scramble: u32) -> f32;
}
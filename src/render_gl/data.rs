#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32x3 {
    pub d0: f32,
    pub d1: f32,
    pub d2: f32,
}

impl f32x3 {
    pub fn new(d0: f32, d1: f32, d2: f32) -> f32x3 {
        f32x3 {
            d0, d1, d2
        }
    }
}

impl From<(f32, f32, f32)> for f32x3 {
    fn from(other: (f32, f32, f32)) -> Self {
        f32x3::new(other.0, other.1, other.2)
    }
}

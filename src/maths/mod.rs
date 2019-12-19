#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i32x3 {
    pub d0: i32,
    pub d1: i32,
    pub d2: i32,
}

impl i32x3 {
    pub fn new(d0: i32, d1: i32, d2: i32) -> i32x3 {
        i32x3 {
            d0,
            d1,
            d2,
        }
    }
}

impl From<(i32, i32, i32)> for i32x3 {
    fn from(other: (i32, i32, i32)) -> Self {
        i32x3::new(other.0, other.1, other.2)
    }
}

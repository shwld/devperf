pub struct NonZeroU32(u32);
impl NonZeroU32 {
    pub fn new(number: u32) -> Result<Self, String> {
        if number > 0 {
            Ok(NonZeroU32(number))
        } else {
            Err("Integer must be greater than zero".to_string())
        }
    }
}

pub struct NonZeroF32(f32);
impl NonZeroF32 {
    pub fn new(number: f32) -> Result<Self, String> {
        if number > 0.0 {
            Ok(NonZeroF32(number))
        } else {
            Err("Integer must be greater than zero".to_string())
        }
    }
}

// Error types
#[derive(Debug, Clone)]
pub struct WriteConfigError;

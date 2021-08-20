use std::convert::TryInto;

pub trait IntoClampedI64 {
    fn into_i64(self) -> i64;
}

impl IntoClampedI64 for u64 {
    fn into_i64(self) -> i64 {
        match self.try_into() {
            Ok(as_i64) => as_i64,
            Err(_) => i64::MAX,
        }
    }
}
pub trait IntoClampedI32 {
    fn into_i32(self) -> i32;
}

impl IntoClampedI32 for u64 {
    fn into_i32(self) -> i32 {
        match self.try_into() {
            Ok(as_i32) => as_i32,
            Err(_) => i32::MAX,
        }
    }
}

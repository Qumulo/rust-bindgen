#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct future {
    _unused: [u8; 0],
}
pub type future_t = future;
unsafe extern "C" {
    pub fn schedule(f: *mut future_t);
}

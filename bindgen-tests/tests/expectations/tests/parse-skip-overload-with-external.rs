#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
unsafe extern "C" {
    #[link_name = "\u{1}_Z8do_thingP5alphaS0_"]
    pub fn do_thing(arg1: *mut alpha, arg2: *mut alpha);
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z8do_thingP4betaS0_"]
    pub fn do_thing_ptr_struct_beta_ptr_struct_beta(arg1: *mut beta, arg2: *mut beta);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct alpha {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct beta {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gamma {
    _unused: [u8; 0],
}

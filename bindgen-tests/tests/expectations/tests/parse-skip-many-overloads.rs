#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
unsafe extern "C" {
    #[link_name = "\u{1}_Z8do_thingP5alphaS0_"]
    pub fn do_thing(arg1: *mut alpha, arg2: *mut alpha);
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z8do_thingP4betaS0_"]
    pub fn do_thing_ptr_struct_beta_ptr_struct_beta(arg1: *mut beta, arg2: *mut beta);
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z8do_thingP5gammaS0_"]
    pub fn do_thing_ptr_struct_gamma_ptr_struct_gamma(
        arg1: *mut gamma,
        arg2: *mut gamma,
    );
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z8do_thingP5deltaS0_"]
    pub fn do_thing_ptr_struct_delta_ptr_struct_delta(
        arg1: *mut delta,
        arg2: *mut delta,
    );
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z8do_thingP7epsilonS0_"]
    pub fn do_thing_ptr_struct_epsilon_ptr_struct_epsilon(
        arg1: *mut epsilon,
        arg2: *mut epsilon,
    );
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
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct delta {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct epsilon {
    _unused: [u8; 0],
}

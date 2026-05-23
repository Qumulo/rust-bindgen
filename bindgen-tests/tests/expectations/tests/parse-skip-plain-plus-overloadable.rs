#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
unsafe extern "C" {
    pub fn my_func(arg1: *mut alpha, arg2: *mut alpha) -> bool;
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z7my_funcP4betaS0_"]
    pub fn my_func_ptr_struct_beta_ptr_struct_beta(
        arg1: *mut beta,
        arg2: *mut beta,
    ) -> bool;
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z7my_funcP5gammaS0_"]
    pub fn my_func_ptr_struct_gamma_ptr_struct_gamma(
        arg1: *mut gamma,
        arg2: *mut gamma,
    ) -> bool;
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z7my_funcP5deltaS0_"]
    pub fn my_func_ptr_struct_delta_ptr_struct_delta(
        arg1: *mut delta,
        arg2: *mut delta,
    ) -> bool;
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z7my_funcP7epsilonS0_"]
    pub fn my_func_ptr_struct_epsilon_ptr_struct_epsilon(
        arg1: *mut epsilon,
        arg2: *mut epsilon,
    ) -> bool;
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

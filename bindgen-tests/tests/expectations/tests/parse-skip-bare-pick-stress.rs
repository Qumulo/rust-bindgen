#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct str_a {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct str_b {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct str_c {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct str_d {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct str_e {
    _unused: [u8; 0],
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z6str_eqP5str_aS0_"]
    pub fn str_eq(arg1: *mut str_a, arg2: *mut str_a) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z6str_eqP5str_bS0_"]
    pub fn str_eq_ptr_struct_str_b_ptr_struct_str_b(
        arg1: *mut str_b,
        arg2: *mut str_b,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z6str_eqP5str_cS0_"]
    pub fn str_eq_ptr_struct_str_c_ptr_struct_str_c(
        arg1: *mut str_c,
        arg2: *mut str_c,
    ) -> ::std::os::raw::c_int;
}

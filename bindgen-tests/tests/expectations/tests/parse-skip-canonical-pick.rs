#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iostream {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tcp_socket {
    _unused: [u8; 0],
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z13iostream_castP10tcp_socket"]
    pub fn iostream_cast_ptr_struct_tcp_socket(arg1: *mut tcp_socket) -> *mut iostream;
}

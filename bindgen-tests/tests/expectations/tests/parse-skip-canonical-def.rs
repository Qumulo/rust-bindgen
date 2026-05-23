#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct payload {
    pub data: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of payload"][::std::mem::size_of::<payload>() - 4usize];
    ["Alignment of payload"][::std::mem::align_of::<payload>() - 4usize];
    ["Offset of field: payload::data"][::std::mem::offset_of!(payload, data) - 0usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}_Z4procP7payloadi"]
    pub fn proc_(arg1: *mut payload, arg2: ::std::os::raw::c_int);
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z4procP7payloadl"]
    pub fn proc_long(arg1: *mut payload, arg2: ::std::os::raw::c_long);
}

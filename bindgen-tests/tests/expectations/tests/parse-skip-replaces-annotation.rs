#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct error {
    _unused: [u8; 0],
}
unsafe extern "C" {
    pub fn do_thing(handle: *const opaque_handle) -> *mut error;
}
/// <div rustbindgen replaces="opaque_thing_desc"></div>
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct opaque_thing_desc {}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of opaque_thing_desc"][::std::mem::size_of::<opaque_thing_desc>() - 0usize];
    [
        "Alignment of opaque_thing_desc",
    ][::std::mem::align_of::<opaque_thing_desc>() - 1usize];
};

#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct handle_value {
    pub opaque: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of handle_value"][::std::mem::size_of::<handle_value>() - 4usize];
    ["Alignment of handle_value"][::std::mem::align_of::<handle_value>() - 4usize];
    [
        "Offset of field: handle_value::opaque",
    ][::std::mem::offset_of!(handle_value, opaque) - 0usize];
};
unsafe extern "C" {
    pub fn get_handle() -> *const handle_value;
}
unsafe extern "C" {
    pub fn take_handle(h: *const handle_value);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct holder {
    pub held: *const handle_value,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of holder"][::std::mem::size_of::<holder>() - 8usize];
    ["Alignment of holder"][::std::mem::align_of::<holder>() - 8usize];
    ["Offset of field: holder::held"][::std::mem::offset_of!(holder, held) - 0usize];
};
impl Default for holder {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

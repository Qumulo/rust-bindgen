#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
unsafe extern "C" {
    pub static mut g_count: ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct widget {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of widget"][::std::mem::size_of::<widget>() - 8usize];
    ["Alignment of widget"][::std::mem::align_of::<widget>() - 4usize];
    ["Offset of field: widget::x"][::std::mem::offset_of!(widget, x) - 0usize];
    ["Offset of field: widget::y"][::std::mem::offset_of!(widget, y) - 4usize];
};
unsafe extern "C" {
    pub fn widget_init(arg1: *mut widget);
}

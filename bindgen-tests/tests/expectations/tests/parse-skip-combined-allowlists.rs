#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct named_via_file_allowlist {
    pub x: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of named_via_file_allowlist",
    ][::std::mem::size_of::<named_via_file_allowlist>() - 4usize];
    [
        "Alignment of named_via_file_allowlist",
    ][::std::mem::align_of::<named_via_file_allowlist>() - 4usize];
    [
        "Offset of field: named_via_file_allowlist::x",
    ][::std::mem::offset_of!(named_via_file_allowlist, x) - 0usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct named_via_type_allowlist {
    pub a: ::std::os::raw::c_int,
    pub b: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of named_via_type_allowlist",
    ][::std::mem::size_of::<named_via_type_allowlist>() - 8usize];
    [
        "Alignment of named_via_type_allowlist",
    ][::std::mem::align_of::<named_via_type_allowlist>() - 4usize];
    [
        "Offset of field: named_via_type_allowlist::a",
    ][::std::mem::offset_of!(named_via_type_allowlist, a) - 0usize];
    [
        "Offset of field: named_via_type_allowlist::b",
    ][::std::mem::offset_of!(named_via_type_allowlist, b) - 4usize];
};

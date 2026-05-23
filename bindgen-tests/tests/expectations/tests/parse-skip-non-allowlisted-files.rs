#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct DefinedInDroppedFile {
    pub dropped_field: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of DefinedInDroppedFile",
    ][::std::mem::size_of::<DefinedInDroppedFile>() - 4usize];
    [
        "Alignment of DefinedInDroppedFile",
    ][::std::mem::align_of::<DefinedInDroppedFile>() - 4usize];
    [
        "Offset of field: DefinedInDroppedFile::dropped_field",
    ][::std::mem::offset_of!(DefinedInDroppedFile, dropped_field) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InAllowlistedFile {
    pub other: *mut DefinedInDroppedFile,
    pub x: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of InAllowlistedFile"][::std::mem::size_of::<InAllowlistedFile>() - 16usize];
    [
        "Alignment of InAllowlistedFile",
    ][::std::mem::align_of::<InAllowlistedFile>() - 8usize];
    [
        "Offset of field: InAllowlistedFile::other",
    ][::std::mem::offset_of!(InAllowlistedFile, other) - 0usize];
    [
        "Offset of field: InAllowlistedFile::x",
    ][::std::mem::offset_of!(InAllowlistedFile, x) - 8usize];
};
impl Default for InAllowlistedFile {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z20allowlisted_functionv"]
    pub fn allowlisted_function();
}

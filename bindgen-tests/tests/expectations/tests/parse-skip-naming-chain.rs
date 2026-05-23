#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct leaf_value_s {
    pub v: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of leaf_value_s"][::std::mem::size_of::<leaf_value_s>() - 4usize];
    ["Alignment of leaf_value_s"][::std::mem::align_of::<leaf_value_s>() - 4usize];
    [
        "Offset of field: leaf_value_s::v",
    ][::std::mem::offset_of!(leaf_value_s, v) - 0usize];
};
pub type leaf_value = leaf_value_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct inner_ref_s {
    pub leaf: *mut leaf_value,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of inner_ref_s"][::std::mem::size_of::<inner_ref_s>() - 8usize];
    ["Alignment of inner_ref_s"][::std::mem::align_of::<inner_ref_s>() - 8usize];
    [
        "Offset of field: inner_ref_s::leaf",
    ][::std::mem::offset_of!(inner_ref_s, leaf) - 0usize];
};
impl Default for inner_ref_s {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type inner_ref = inner_ref_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct middle_ref_s {
    pub inner: *mut inner_ref,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of middle_ref_s"][::std::mem::size_of::<middle_ref_s>() - 8usize];
    ["Alignment of middle_ref_s"][::std::mem::align_of::<middle_ref_s>() - 8usize];
    [
        "Offset of field: middle_ref_s::inner",
    ][::std::mem::offset_of!(middle_ref_s, inner) - 0usize];
};
impl Default for middle_ref_s {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type middle_ref = middle_ref_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct outer_ref_s {
    pub middle: *mut middle_ref,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of outer_ref_s"][::std::mem::size_of::<outer_ref_s>() - 8usize];
    ["Alignment of outer_ref_s"][::std::mem::align_of::<outer_ref_s>() - 8usize];
    [
        "Offset of field: outer_ref_s::middle",
    ][::std::mem::offset_of!(outer_ref_s, middle) - 0usize];
};
impl Default for outer_ref_s {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type outer_ref = outer_ref_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct entry_in_allowlisted_file {
    pub outer: *mut outer_ref,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of entry_in_allowlisted_file",
    ][::std::mem::size_of::<entry_in_allowlisted_file>() - 8usize];
    [
        "Alignment of entry_in_allowlisted_file",
    ][::std::mem::align_of::<entry_in_allowlisted_file>() - 8usize];
    [
        "Offset of field: entry_in_allowlisted_file::outer",
    ][::std::mem::offset_of!(entry_in_allowlisted_file, outer) - 0usize];
};
impl Default for entry_in_allowlisted_file {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn get_outer() -> *mut outer_ref;
}

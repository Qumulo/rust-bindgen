#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct user_b {
    pub ptr: *mut dup_target,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of user_b"][::std::mem::size_of::<user_b>() - 8usize];
    ["Alignment of user_b"][::std::mem::align_of::<user_b>() - 8usize];
    ["Offset of field: user_b::ptr"][::std::mem::offset_of!(user_b, ptr) - 0usize];
};
impl Default for user_b {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/// <div rustbindgen replaces="dup_target"></div>
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct dup_target {
    pub field_a: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of dup_target"][::std::mem::size_of::<dup_target>() - 4usize];
    ["Alignment of dup_target"][::std::mem::align_of::<dup_target>() - 4usize];
    [
        "Offset of field: dup_target::field_a",
    ][::std::mem::offset_of!(dup_target, field_a) - 0usize];
};
/// <div rustbindgen replaces="dup_target"></div>
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct dup_target {
    pub field_b1: ::std::os::raw::c_int,
    pub field_b2: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of dup_target"][::std::mem::size_of::<dup_target>() - 8usize];
    ["Alignment of dup_target"][::std::mem::align_of::<dup_target>() - 4usize];
    [
        "Offset of field: dup_target::field_b1",
    ][::std::mem::offset_of!(dup_target, field_b1) - 0usize];
    [
        "Offset of field: dup_target::field_b2",
    ][::std::mem::offset_of!(dup_target, field_b2) - 4usize];
};

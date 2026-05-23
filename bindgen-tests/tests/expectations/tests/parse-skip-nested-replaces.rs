#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct user_of_real_target {
    pub ptr: *mut real_target,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of user_of_real_target",
    ][::std::mem::size_of::<user_of_real_target>() - 8usize];
    [
        "Alignment of user_of_real_target",
    ][::std::mem::align_of::<user_of_real_target>() - 8usize];
    [
        "Offset of field: user_of_real_target::ptr",
    ][::std::mem::offset_of!(user_of_real_target, ptr) - 0usize];
};
impl Default for user_of_real_target {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/// <div rustbindgen replaces="real_target"></div>
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct real_target {
    pub field: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of real_target"][::std::mem::size_of::<real_target>() - 4usize];
    ["Alignment of real_target"][::std::mem::align_of::<real_target>() - 4usize];
    [
        "Offset of field: real_target::field",
    ][::std::mem::offset_of!(real_target, field) - 0usize];
};

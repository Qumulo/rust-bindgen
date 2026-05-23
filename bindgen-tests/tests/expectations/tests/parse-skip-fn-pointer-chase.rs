#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type cb_t = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut arg_type) -> *mut ret_type,
>;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct user {
    pub callback: cb_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of user"][::std::mem::size_of::<user>() - 8usize];
    ["Alignment of user"][::std::mem::align_of::<user>() - 8usize];
    ["Offset of field: user::callback"][::std::mem::offset_of!(user, callback) - 0usize];
};

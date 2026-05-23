#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct fs_thing {
    pub field: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fs_thing"][::std::mem::size_of::<fs_thing>() - 4usize];
    ["Alignment of fs_thing"][::std::mem::align_of::<fs_thing>() - 4usize];
    [
        "Offset of field: fs_thing::field",
    ][::std::mem::offset_of!(fs_thing, field) - 0usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct vfs_thing {
    pub field: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of vfs_thing"][::std::mem::size_of::<vfs_thing>() - 4usize];
    ["Alignment of vfs_thing"][::std::mem::align_of::<vfs_thing>() - 4usize];
    [
        "Offset of field: vfs_thing::field",
    ][::std::mem::offset_of!(vfs_thing, field) - 0usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}_Z4initP8fs_thingPK9vfs_thing"]
    pub fn init_ptr_const_struct_vfs_thing(arg1: *mut fs_thing, arg2: *const vfs_thing);
}

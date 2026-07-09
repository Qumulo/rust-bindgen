#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// <div rustbindgen pinned></div>
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Pinned {
    pub a: ::std::os::raw::c_int,
    pub __bindgen_pinned: ::std::marker::PhantomPinned,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Pinned"][::std::mem::size_of::<Pinned>() - 4usize];
    ["Alignment of Pinned"][::std::mem::align_of::<Pinned>() - 4usize];
    ["Offset of field: Pinned::a"][::std::mem::offset_of!(Pinned, a) - 0usize];
};

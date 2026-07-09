#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// <div rustbindgen derive="Copy" derive="Clone"></div>
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct myCopyStruct {
    pub i: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of myCopyStruct"][::std::mem::size_of::<myCopyStruct>() - 4usize];
    ["Alignment of myCopyStruct"][::std::mem::align_of::<myCopyStruct>() - 4usize];
    [
        "Offset of field: myCopyStruct::i",
    ][::std::mem::offset_of!(myCopyStruct, i) - 0usize];
};
/// <div rustbindgen derive="Copy" derive="Clone"></div>
pub type myTypedef = myCopyStruct;
#[repr(C)]
pub union myUnion {
    pub i: myCopyStruct,
    pub t: myTypedef,
    pub primitiveInt: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of myUnion"][::std::mem::size_of::<myUnion>() - 4usize];
    ["Alignment of myUnion"][::std::mem::align_of::<myUnion>() - 4usize];
    ["Offset of field: myUnion::i"][::std::mem::offset_of!(myUnion, i) - 0usize];
    ["Offset of field: myUnion::t"][::std::mem::offset_of!(myUnion, t) - 0usize];
    [
        "Offset of field: myUnion::primitiveInt",
    ][::std::mem::offset_of!(myUnion, primitiveInt) - 0usize];
};
impl Default for myUnion {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

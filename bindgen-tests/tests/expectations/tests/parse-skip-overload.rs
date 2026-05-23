#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct third_arg {
    pub c: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of third_arg"][::std::mem::size_of::<third_arg>() - 4usize];
    ["Alignment of third_arg"][::std::mem::align_of::<third_arg>() - 4usize];
    ["Offset of field: third_arg::c"][::std::mem::offset_of!(third_arg, c) - 0usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}_Z7as_sinkP9third_arg"]
    pub fn as_sink_ptr_struct_third_arg(z: *mut third_arg);
}

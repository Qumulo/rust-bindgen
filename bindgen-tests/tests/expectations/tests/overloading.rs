#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
unsafe extern "C" {
    #[link_name = "\u{1}_Z8Evaluatec"]
    pub fn Evaluate(r: ::std::os::raw::c_char) -> bool;
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z8Evaluateii"]
    pub fn Evaluate_int_int(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int) -> bool;
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z13CanonicalLastc"]
    pub fn CanonicalLast_char(r: ::std::os::raw::c_char);
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z13CanonicalLastv"]
    pub fn CanonicalLast();
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z6CommonPii"]
    pub fn Common(arg1: *mut ::std::os::raw::c_int, y: ::std::os::raw::c_int) -> f32;
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z6CommonPiPb"]
    pub fn Common_ptr_bool(arg1: *mut ::std::os::raw::c_int, y: *mut bool) -> f32;
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z6CommonPiPKc"]
    pub fn Common_ptr_const_char(
        arg1: *mut ::std::os::raw::c_int,
        y: *const ::std::os::raw::c_char,
    ) -> f32;
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z23CanonicalLastWithCommonPii"]
    pub fn CanonicalLastWithCommon_int(
        arg1: *mut ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z23CanonicalLastWithCommonPi"]
    pub fn CanonicalLastWithCommon(arg1: *mut ::std::os::raw::c_int);
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN3foo10MyFunctionEv"]
    pub fn foo_MyFunction();
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN3bar10MyFunctionEv"]
    pub fn bar_MyFunction();
}

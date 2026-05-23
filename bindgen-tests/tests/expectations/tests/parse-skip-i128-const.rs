#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
unsafe extern "C" {
    pub static my_min_i128: i128_;
}
unsafe extern "C" {
    pub static my_max_i128: i128_;
}
unsafe extern "C" {
    pub static my_min_u128: u128_;
}
unsafe extern "C" {
    pub static my_max_u128: u128_;
}
pub type int128_t = __int128_t;
pub type __int128_t = i128;
pub type uint128_t = __uint128_t;
pub type __uint128_t = u128;
pub type i128_ = int128_t;
pub type u128_ = uint128_t;

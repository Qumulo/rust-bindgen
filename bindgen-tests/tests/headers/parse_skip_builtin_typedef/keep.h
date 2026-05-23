// `__int128_t` is a clang builtin typedef whose cursor location is in a
// synthetic file (not in any user-supplied header). The allowlisted code
// here aliases it. With the flag on, the lazy-parse pre-pass must still
// emit `pub type __int128_t = i128;` so the downstream alias resolves.
typedef __int128_t int128_alias;
typedef __uint128_t uint128_alias;

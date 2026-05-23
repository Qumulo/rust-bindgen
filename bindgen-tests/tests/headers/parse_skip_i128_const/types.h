#pragma once

// Two-step typedef chain through clang builtin `__int128_t` /
// `__uint128_t`. The chain forces the typedef-chain fallback in
// `var.rs` to resolve through clang's canonical type when bindgen's
// own IR has the type as an UnresolvedTypeRef.
typedef __int128_t int128_t;
typedef __uint128_t uint128_t;
typedef int128_t i128;
typedef uint128_t u128;

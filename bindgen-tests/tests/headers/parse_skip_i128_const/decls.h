#pragma once

#include "types.h"

// `static const T x = initializer` with T wider than 64 bits.
// libclang's `CXEvalResult` accessors return i64/u64 and silently
// truncate wider values. Both modes must emit `pub static` here,
// not `pub const = 0` / `= -1` with truncated literals.
static const i128 my_min_i128 = (i128)((u128)1 << (128 - 1));
static const i128 my_max_i128 = (i128)~my_min_i128;
static const u128 my_min_u128 = 0;
static const u128 my_max_u128 = (u128)~my_min_u128;

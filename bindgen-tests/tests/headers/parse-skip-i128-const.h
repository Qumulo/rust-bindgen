// bindgen-flags: --parse-skip-non-allowlisted-files --allowlist-file ".*/parse_skip_i128_const/.*" -- -x c -std=c2x -Itests/headers
//
// When a `static const T x = initializer` has `T` wider than 64 bits
// (`i128` / `u128`), the typedef-chain fallback in `var.rs` must NOT
// promote `canonical_kind` to `Int(I128 | U128)`. Doing so would feed
// the initializer to `cursor.evaluate()`, whose `as_int` /
// `as_unsigned` accessors are `clang_EvalResult_getAsLongLong` /
// `clang_EvalResult_getAsUnsigned` — both i64/u64, both silently
// truncate values that don't fit. Bindgen would emit `pub const x:
// i128_ = 0;` (or `= -1`) with WRONG values. Falling back to
// `pub static x: i128_;` matches OFF and gives consumers a working
// extern binding (just without a compile-time literal).

#include "parse_skip_i128_const/decls.h"

// bindgen-flags: --parse-skip-non-allowlisted-files --distrust-clang-mangling --allowlist-file ".*/parse_skip_overload_no_mangling/decls\\.h" -- -x c++ -Itests/headers
//
// Regression coverage for the `parse_overloads_of_allowlisted` dedup
// bug under `--distrust-clang-mangling` (which makes
// `cursor_mangling` return `None`, so every Function Item's
// `mangled_name()` is `None`). Before the fix, the dedup set built
// from `f.mangled_name()` was empty and every non-allowlisted sibling
// cursor got re-parsed once per redeclaration. The fix dedups by USR
// (cursor identity) instead, which is independent of whether mangling
// is enabled.

#include "parse_skip_overload_no_mangling/decls.h"

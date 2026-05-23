// bindgen-flags: --parse-skip-non-allowlisted-files --no-recursive-allowlist --allowlist-file ".*/parse_skip_fn_pointer_chase/decls\\.h" -- -Itests/headers
//
// A function-pointer typedef whose argument and return types live in
// a non-allowlisted file. `chase_canonical_decls` walks through
// `args()` and `ret_type()` on function-typed cursors so the
// canonical declarations of arg / return struct types are registered
// in the IR's USR cache, matching what the flag-off path does. The
// emitted Rust is the same in both modes — `--no-recursive-allowlist`
// filters the arg / return structs out of codegen — but pinning the
// expectation catches IR-state regressions in either path.

#include "parse_skip_fn_pointer_chase/decls.h"

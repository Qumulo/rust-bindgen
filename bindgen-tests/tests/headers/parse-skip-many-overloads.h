// bindgen-flags: --parse-skip-non-allowlisted-files --allowlist-file ".*/parse_skip_many_overloads/.*" -- -Itests/headers
//
// With multiple `__attribute__((overloadable))` declarations of the
// same name, the ON path must emit exactly one bare-name overload
// plus N suffixed, matching what flag OFF emits. The canonical pick
// must point to an overload that's in codegen_items (i.e., from an
// allowlisted file) so the bare emission actually happens.

#include "parse_skip_many_overloads/decls.h"

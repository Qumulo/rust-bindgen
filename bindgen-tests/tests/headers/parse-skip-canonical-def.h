// bindgen-flags: --parse-skip-non-allowlisted-files --allowlist-file ".*/parse_skip_canonical_def/.*" -- -Itests/headers
//
// The struct's definition is here in the non-allowlisted entry header.
// Its forward decl (chosen by clang as canonical) is in another
// non-allowlisted header. The allowlisted file declares two overloads
// of `proc` with the same first arg type (`struct payload *`). With the
// canonical-cursor pre-pass, the chase_canonical_decls path must pass
// the *definition* cursor (not the canonical forward decl) to Item::parse,
// or the struct never registers in `self.types` and downstream overload
// suffix computation produces mismatched first args across the two
// overloads.

struct payload {
    int data;
};
#include "parse_skip_canonical_def/keep.h"

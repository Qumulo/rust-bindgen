// bindgen-flags: --parse-skip-non-allowlisted-files --allowlist-file ".*/parse_skip_overload_with_external/.*" -- -Itests/headers
//
// Mixed overload set, source-first overload IS allowlisted:
//   - Two allowlisted overloads (alpha, beta) included FIRST.
//   - One non-allowlisted overload (gamma) declared AFTER in this entry header.
//
// Source order: alpha, beta (via include), then gamma. alpha is the
// source-first overload and IS allowlisted. Both modes should pick
// alpha as canonical → bare emission. beta gets suffix.
// gamma also gets suffix but isn't emitted (non-allowlisted).
//
// Expected: both OFF and ON emit 1 bare (alpha) + 1 suffixed (beta).
// Bug A regression on ON would show 0 bare + 2 suffixed in ON.

#include "parse_skip_overload_with_external/keep.h"

__attribute__((overloadable)) void do_thing(struct gamma *, struct gamma *);

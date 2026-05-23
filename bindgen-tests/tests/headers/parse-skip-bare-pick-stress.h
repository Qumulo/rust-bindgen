// bindgen-flags: --parse-skip-non-allowlisted-files --allowlist-file ".*/parse_skip_bare_pick_stress/.*" -- -Itests/headers
//
// The canonical pick must point to an overload that is actually
// EMITTED, so the bare-name emission isn't suppressed by codegen's
// `seen_function` dedup. With many overloads split between
// allowlisted and non-allowlisted files where the source-first
// allowlisted overload is the right canonical, ON must emit exactly
// 1 bare + N suffixed — matching OFF. A naive source-location-string
// sort could pick the wrong overload (alphabetical filename order
// disagrees with include-aware source order); the candidate-pick
// sort uses clang TU walk position, which matches what main parse
// sees in OFF.

#include "parse_skip_bare_pick_stress/keep.h"

__attribute__((overloadable)) int str_eq(struct str_d *, struct str_d *);
__attribute__((overloadable)) int str_eq(struct str_e *, struct str_e *);

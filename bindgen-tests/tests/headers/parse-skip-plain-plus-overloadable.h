// bindgen-flags: --parse-skip-non-allowlisted-files --allowlist-file ".*/parse_skip_plain_plus_overloadable/.*" -- -x c -std=c2x -Itests/headers
//
// Trigger: a plain (non-overloadable) function declared in TWO
// allowlisted files, alongside N `[[clang::overloadable]]` siblings.
// The two plain redeclarations have the SAME mangled name; codegen's
// `seen_function` dedup emits only one of them. The candidate pick in
// `compute_overload_suffixes` MUST be whichever redeclaration codegen
// emits first (sorted by source location under the flag), otherwise
// the suffix-cleared candidate is the one that gets dropped —
// silently removing the bare-name binding.

#include "parse_skip_plain_plus_overloadable/decls.h"

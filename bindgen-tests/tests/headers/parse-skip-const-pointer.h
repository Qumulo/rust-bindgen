// bindgen-flags: --parse-skip-non-allowlisted-files --allowlist-file ".*/parse_skip_const_pointer/.*" -- -Itests/headers

// `handle_value` is defined in a non-allowlisted file. The allowlisted file
// references it only through `const struct handle_value*`. On-demand
// materialization must preserve the pointee's const-ness so the generated
// Rust pointer stays `*const`, not `*mut`. (Bug pattern 2 from the
// integration report.)
struct handle_value {
    int opaque;
};

#include "parse_skip_const_pointer/keep.h"

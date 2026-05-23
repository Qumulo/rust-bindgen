// bindgen-flags: --parse-skip-non-allowlisted-files --no-recursive-allowlist --allowlist-file ".*/parse_skip_builtin_typedef/.*" -- -Itests/headers
//
// Clang builtin typedefs referenced by allowlisted code must be
// emitted. They live in a synthetic clang `<builtin>` file whose
// file-name lookup returns None, so the file-based skip filter drops
// them at parse time. The lazy-parse fix-up chases them through the
// referrer's location instead.

#include "parse_skip_builtin_typedef/keep.h"

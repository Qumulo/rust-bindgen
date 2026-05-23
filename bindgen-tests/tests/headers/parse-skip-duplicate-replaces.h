// bindgen-flags: --parse-skip-non-allowlisted-files --no-recursive-allowlist --allowlist-file ".*/parse_skip_duplicate_replaces/decls\\.h" -- -x c -std=c2x -Itests/headers
//
// Two `<div rustbindgen replaces="dup_target"></div>` annotations in
// separate non-allowlisted files. The lazy-parse pre-pass collects
// both annotated cursors and materializes them — and the ON output
// must match what the flag-off path produces with the same input.
// (Vanilla bindgen emits both `_for_rust` bodies renamed to the
// replacement name, producing duplicate `pub struct` decls; that's a
// pre-existing quirk, not something this flag fixes. The test pins
// the behavior so future changes can't silently diverge.)

#include "parse_skip_duplicate_replaces/decls.h"

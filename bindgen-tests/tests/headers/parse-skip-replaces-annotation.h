// bindgen-flags: --parse-skip-non-allowlisted-files --no-recursive-allowlist --allowlist-file ".*/parse_skip_replaces_annotation/decls\\.h" -- -x c -std=c2x -Itests/headers
//
// When a `<div rustbindgen replaces="X">` annotation lives in a
// non-allowlisted file, the main parse skips both the annotated
// `_for_rust` cursor (so `self.replacements` is never populated) and
// the target `X` (so `process_replacements` has nothing to swap).
// `parse_replacement_decls` walks the TU before main parse, finds
// cursors whose raw doc comment carries a `rustbindgen replaces=`
// annotation, and materializes both the `_for_rust` decl and the
// named target — restoring the rename machinery.

#include "parse_skip_replaces_annotation/decls.h"

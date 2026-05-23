// bindgen-flags: --parse-skip-non-allowlisted-files --no-recursive-allowlist --allowlist-file ".*/parse_skip_nested_replaces/decls\\.h" -- -x c++ -Itests/headers
//
// `<div rustbindgen replaces="X">` annotations don't have to be on
// top-level declarations — they can sit on members of an enclosing
// class or struct. The lazy-parse cursor walks
// (`collect_replacement_cursors` and `collect_target_cursors`) must
// descend into struct/class/union/enum bodies, otherwise an
// annotation on a member type silently fails to fire and the
// renamed binding is dropped.

#include "parse_skip_nested_replaces/decls.h"

// bindgen-flags: --parse-skip-non-allowlisted-files --allowlist-file ".*/parse_skip_combined_allowlists/decls\\.h" --allowlist-type "named_via_type_allowlist" -- -Itests/headers
//
// Regression coverage for the safety carve-out in `should_parse_cursor`.
// When `--parse-skip-non-allowlisted-files` is combined with ANY
// non-file allowlist (`--allowlist-type`, `--allowlist-function`,
// `--allowlist-var`, `--allowlist-item`), the file-level skip must
// disable itself. Otherwise a type named by the type-allowlist that
// lives outside the file-allowlist regex would be dropped at parse
// time and never make it to codegen.
//
// Both `named_via_file_allowlist` (matched by `--allowlist-file`) and
// `named_via_type_allowlist` (matched by `--allowlist-type`, in a
// non-matching file) must be emitted.

#include "parse_skip_combined_allowlists/decls.h"

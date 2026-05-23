// bindgen-flags: --parse-skip-non-allowlisted-files --allowlist-file ".*/parse_skip_non_allowlisted_files/.*" -- -Itests/headers

// Definition of a struct whose forward declaration lives in an allowlisted
// file. This entry header is NOT allowlisted, so the definition cursor is
// skipped at parse time. CompInfo::from_ty must still find the definition
// when materializing on demand.
struct DefinedInDroppedFile {
    int dropped_field;
};

// Decls below should not appear in the output: they live in a non-allowlisted
// file and nothing in an allowlisted file references them.
class IgnoredClass {
    char c;
};

void ignored_function();

#include "parse_skip_non_allowlisted_files/keep.hpp"

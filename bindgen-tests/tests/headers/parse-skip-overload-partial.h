// bindgen-flags: --parse-skip-non-allowlisted-files --allowlist-file ".*/parse_skip_overload_partial/decls\\.h" -- -x c++ -Itests/headers
//
// Overload set split across allowlisted / non-allowlisted files,
// where ONLY ONE of three overloads is allowlisted. The
// `--no-mangling` companion fixture (`parse-skip-overload-no-mangling`)
// covers the no-mangling code path; this one covers the more common
// case where mangling is enabled and the candidate pick must select
// an overload whose source location lies inside the allowlist regex.

#include "parse_skip_overload_partial/decls.h"

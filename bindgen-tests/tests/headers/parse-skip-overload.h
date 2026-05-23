// bindgen-flags: --parse-skip-non-allowlisted-files --allowlist-file ".*/parse_skip_overload/.*" -- -x c++ -Itests/headers
//
// Overload disambiguation across allowlist boundaries: two `as_sink`
// overloads live in the non-allowlisted entry header; the third
// (allowlisted) is in keep.h. `compute_overload_suffixes` names each
// overload by its argument types — but only after seeing all
// overloads. With the parse-skip flag the non-allowlisted overloads'
// cursors aren't parsed by the main walk, so the suffix on the
// visible one may shift unless `parse_overloads_of_allowlisted` pulls
// the siblings back in.

struct first_arg { int a; };
struct second_arg { int b; };
struct third_arg { int c; };

void as_sink(struct first_arg* x);
void as_sink(struct second_arg* y);

#include "parse_skip_overload/keep.h"

// bindgen-flags: --parse-skip-non-allowlisted-files --allowlist-file ".*/parse_skip_opaque_extra/.*" -- -Itests/headers
//
// Bug pattern 4 from the integration report: extra opaque structs
// appearing with the flag on. The `struct future` is forward-declared in
// a non-allowlisted file with no definition anywhere. Both runs should
// emit the same opaque struct (or none, if neither references it
// transitively in a way that materializes it).

struct future;

#include "parse_skip_opaque_extra/keep.h"

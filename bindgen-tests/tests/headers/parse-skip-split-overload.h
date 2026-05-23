// bindgen-flags: --parse-skip-non-allowlisted-files --allowlist-file ".*/parse_skip_split_overload/.*" -- -Itests/headers
//
// A plain function decl in a non-allowlisted file and a
// `__attribute__((overloadable))` declaration in an allowlisted file
// share the same name. `compute_overload_suffixes` disambiguates them
// by argument types — but only after seeing both overloads. With the
// flag on, the plain decl is reached via
// `parse_overloads_of_allowlisted` and both overloads must end up in
// the IR with identical canonical-type representations so the
// common-prefix detection finds matching args.

struct fs_thing {
    int field;
};

struct vfs_thing {
    int field;
};

void init(struct fs_thing *);

#include "parse_skip_split_overload/keep.h"

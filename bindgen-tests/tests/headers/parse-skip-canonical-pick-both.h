// bindgen-flags: --parse-skip-non-allowlisted-files --allowlist-file ".*/parse_skip_canonical_pick_both/.*" -- -Itests/headers
//
// Bug C: when multiple overloads in DIFFERENT allowlisted files share a
// name, both flag OFF and flag ON must pick the same canonical overload
// (the one that gets the bare name). With the fix, the pick is the
// first overload in clang's TU walk order (= include order), matching
// what main parse would see in OFF mode.

#include "parse_skip_canonical_pick_both/stream.h"
#include "parse_skip_canonical_pick_both/tcp.h"

// bindgen-flags: --parse-skip-non-allowlisted-files --allowlist-file ".*/parse_skip_canonical_pick/.*" -- -Itests/headers
//
// The canonical-overload pick (which overload gets the bare name) must
// be identical between flag OFF and flag ON. Without a source-order
// candidate pick, flag ON picks the allowlisted overload (lowest
// ItemId in main parse order) while flag OFF picks the source-first
// overload, which may be in a non-allowlisted file.
//
// In this fixture the source-first overload (`stream_socket`) is in
// the non-allowlisted entry header. The allowlisted file in keep.h
// has only the `tcp_socket` overload. The bare-name pick should match
// the source-first overload (stream_socket) in both modes.

struct stream_socket;
struct iostream;

__attribute__((overloadable))
struct iostream *iostream_cast(struct stream_socket *);

#include "parse_skip_canonical_pick/keep.h"

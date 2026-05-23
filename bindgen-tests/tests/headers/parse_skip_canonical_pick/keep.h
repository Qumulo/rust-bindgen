// Only the `tcp_socket` overload of `iostream_cast` is defined here in the
// allowlisted file. The `stream_socket` overload is in the (non-allowlisted)
// entry header — declared FIRST in source order. With the bug, the
// `parse_overloads_of_allowlisted` phase parses non-allowlisted overloads
// AFTER main parse, so the allowlisted-file overload ends up with the lowest
// ItemId and becomes the canonical bare-name pick — even though baseline
// (flag OFF) picks the source-first stream_socket overload.

struct tcp_socket;
struct iostream;

__attribute__((overloadable))
struct iostream *iostream_cast(struct tcp_socket *);

#pragma once

// Simulates a system header that's NOT covered by the allowlist
// regex for this test. Three pieces together form the
// `replaces=`-in-skipped-file pattern:
//
//   1. A `_for_rust` struct definition carrying the `<div rustbindgen
//      replaces="X">` annotation. Provides the desired Rust-side body.
//   2. A forward declaration of the target `X`. The thing that ends up
//      named in the output.
//   3. A wrapper struct that points to `X`, so allowlisted code can
//      reach `X` indirectly without referencing `_for_rust` by name.

/// <div rustbindgen replaces="opaque_thing_desc"></div>
struct opaque_thing_desc_for_rust {
    // empty
};

struct opaque_thing_desc;

struct opaque_handle {
    struct opaque_thing_desc *inner;
};

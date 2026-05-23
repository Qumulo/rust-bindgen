#pragma once

// The `replaces=` annotation lives on a member nested inside a class
// body that is never referenced from allowlisted code. The target
// `real_target` IS reachable — via a top-level forward declaration
// in this file — but only the chased forward decl ever gets walked.
//
// Without recursion into struct/class bodies, the lazy-parse cursor
// walk would never see `real_target_for_rust`, the replacement entry
// would never get registered, and the chased `real_target` would
// emit as an opaque struct instead of taking the `_for_rust` body.

struct unreferenced_container {
    /// <div rustbindgen replaces="real_target"></div>
    struct real_target_for_rust {
        int field;
    };
};

struct real_target;

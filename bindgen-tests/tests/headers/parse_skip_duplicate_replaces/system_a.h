#pragma once

// First replacement candidate for `dup_target`.
/// <div rustbindgen replaces="dup_target"></div>
struct dup_target_from_a {
    int field_a;
};

struct dup_target;

struct user_a {
    struct dup_target *ptr;
};

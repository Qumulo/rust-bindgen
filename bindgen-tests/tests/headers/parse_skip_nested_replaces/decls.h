#pragma once
#include "system.h"

// Reach `real_target` via a pointer — chases the top-level forward
// declaration in system.h, but never touches
// `unreferenced_container`.
struct user_of_real_target {
    struct real_target *ptr;
};

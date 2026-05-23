#pragma once
#include "system.h"

// `cb_t` is reachable through allowlisted code, but `arg_type` and
// `ret_type` are only reachable through the function-pointer
// signature. The chase has to follow `args()` / `ret_type()` to
// materialize them.
typedef struct ret_type *(*cb_t)(struct arg_type *);

struct user {
    cb_t callback;
};

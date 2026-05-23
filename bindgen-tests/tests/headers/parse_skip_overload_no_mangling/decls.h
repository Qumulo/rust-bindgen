#pragma once
#include "siblings.h"

// Allowlisted overload of `do_thing` — paired with the
// `[[clang::overloadable]]` siblings in siblings.h.
[[clang::overloadable]] int do_thing(int);

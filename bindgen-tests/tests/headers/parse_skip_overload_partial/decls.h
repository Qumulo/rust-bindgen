#pragma once
#include "system.h"

// Single allowlisted overload of `widget_do`. `parse_overloads_of_allowlisted`
// must materialize the int and long siblings from system.h so the
// suffix-pick in `compute_overload_suffixes` sees the full overload
// set. The candidate (the one that emits with the bare name) must
// land on a member of an allowlisted file — otherwise codegen
// suppresses the bare and only the suffixed overload survives.
[[clang::overloadable]] int widget_do(char);

#pragma once
#include "system.h"

// Allowlisted code that uses the opaque type indirectly.
struct error;
struct error *
do_thing(const struct opaque_handle *handle);

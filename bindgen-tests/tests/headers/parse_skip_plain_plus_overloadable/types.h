#pragma once

// One half of the cross-file plain-redeclaration pattern: this file
// forward-declares the plain (non-overloadable) function alongside
// the struct it operates on. The other half (decls.h) re-declares
// `my_func` with the same signature and then layers the overloadable
// siblings on top.

struct alpha;
struct beta;
struct gamma;
struct delta;
struct epsilon;

// `_Bool` (not `bool`) so the fixture compiles without `<stdbool.h>`,
// which isn't reliably picked up under the older libclang versions
// in the test matrix.
_Bool my_func(struct alpha *, struct alpha *);

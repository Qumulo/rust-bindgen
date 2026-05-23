#pragma once

#include "types.h"

// Plain redeclaration: same signature as the one in `types.h`, but a
// distinct source location. Both cursors have the same mangled name,
// so codegen's `seen_function` dedup picks one and silently drops
// the other. The candidate-pick in `compute_overload_suffixes` must
// agree with codegen on which redeclaration survives.
_Bool my_func(struct alpha *, struct alpha *);

// `[[clang::overloadable]]` siblings of the plain decl. The full set
// (plain + these) participates in overload-suffix calculation.
[[clang::overloadable]] _Bool my_func(struct beta *, struct beta *);
[[clang::overloadable]] _Bool my_func(struct gamma *, struct gamma *);
[[clang::overloadable]] _Bool my_func(struct delta *, struct delta *);
[[clang::overloadable]] _Bool my_func(struct epsilon *, struct epsilon *);

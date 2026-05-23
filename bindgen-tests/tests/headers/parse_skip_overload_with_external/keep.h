#include "types.h"

// These two overloads are allowlisted (live in this file).
__attribute__((overloadable)) void do_thing(struct alpha *, struct alpha *);
__attribute__((overloadable)) void do_thing(struct beta *, struct beta *);

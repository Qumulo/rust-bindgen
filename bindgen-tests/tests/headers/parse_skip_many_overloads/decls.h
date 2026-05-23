#include "types.h"

// Many overloads of `do_thing`. Each in its own header would be more
// realistic, but a single header is enough to exercise the suffix-pick
// logic. The first one (alpha) should get the bare name in both flag
// modes.

__attribute__((overloadable)) void do_thing(struct alpha *, struct alpha *);
__attribute__((overloadable)) void do_thing(struct beta *, struct beta *);
__attribute__((overloadable)) void do_thing(struct gamma *, struct gamma *);
__attribute__((overloadable)) void do_thing(struct delta *, struct delta *);
__attribute__((overloadable)) void do_thing(struct epsilon *, struct epsilon *);

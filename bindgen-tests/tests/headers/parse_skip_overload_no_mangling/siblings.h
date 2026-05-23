#pragma once

// Non-allowlisted overload siblings. With `--no-mangling`, the
// allowlisted `do_thing(int)` from decls.h ends up as a Function Item
// whose `mangled_name()` is `None`. The previous dedup logic in
// `parse_overloads_of_allowlisted` walked the items collecting
// `f.mangled_name()` into a `HashSet<String>` and then skipped any
// cursor whose `cursor.mangling()` was in that set — but since the
// set was empty, every redeclaration of `do_thing(int)` would be
// re-parsed, producing duplicate Function Items and an inconsistent
// overload-suffix table. The fix routes both sides through the
// USR-based dedup so the mangling flag is irrelevant.
[[clang::overloadable]] int do_thing(char);
[[clang::overloadable]] int do_thing(long);

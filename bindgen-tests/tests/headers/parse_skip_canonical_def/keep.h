#include "../parse_skip_canonical_def_other/forward.h"

// Two overloads share a name. Their first arg is the same type:
// a pointer to `struct payload`. The overload-suffix logic computes
// `arg_names` via `canonical_type(ctx).sanitized_name(ctx)` for each
// arg. If chase_canonical_decls registered `payload` in self.types
// via add_item (USR-keyed), the elaborated-type lookup in
// builtin_or_resolved_ty creates a wrapper Item with name
// "struct payload" → sanitize "struct_payload" → "ptr_struct_payload"
// for both overloads' first arg → common_prefix=1. If the
// registration is skipped (e.g. when we fed a canonical *forward*
// decl cursor through Item::parse), the lookup falls through to
// building a struct Item directly with name "payload" → "fs_payload"
// → mismatched first args across overloads → common_prefix=0 →
// divergent suffix.

__attribute__((overloadable))
void proc(struct payload *, int);
__attribute__((overloadable))
void proc(struct payload *, long);

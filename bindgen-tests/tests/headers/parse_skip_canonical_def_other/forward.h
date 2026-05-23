// Forward declaration in this non-allowlisted header. This is the
// `canonical` declaration of the struct (clang picks the first decl as
// canonical). When chase_canonical_decls collects this cursor and feeds
// it to Item::parse, the types branch sees `cursor.definition() !=
// cursor` and falls back to an UnresolvedTypeRef placeholder instead of
// materializing the struct — bypassing add_item and the USR cache.
// That breaks elaborated-type wrapper creation for downstream
// `struct foo *` references in allowlisted code.
struct payload;

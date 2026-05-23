// Many overloads of `str_eq`. The first one in source order is in this
// allowlisted file. Several siblings live in the non-allowlisted entry
// header below. Both modes must:
//   - pick the source-first overload (in this file) as canonical.
//   - emit it bare.
//   - emit allowlisted siblings with their type-based suffixes.

struct str_a;
struct str_b;
struct str_c;
struct str_d;
struct str_e;

__attribute__((overloadable)) int str_eq(struct str_a *, struct str_a *);
__attribute__((overloadable)) int str_eq(struct str_b *, struct str_b *);
__attribute__((overloadable)) int str_eq(struct str_c *, struct str_c *);

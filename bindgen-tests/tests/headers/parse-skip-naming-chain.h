// bindgen-flags: --parse-skip-non-allowlisted-files --allowlist-file ".*/parse_skip_naming_chain/.*" -- -Itests/headers

// These typedefs all live in a non-allowlisted file. The allowlisted entry
// struct in keep.h references `outer_ref_t`, which transitively references
// the chain. Each on-demand materialization creates a wrapper Item parented
// by the referrer, so without the fix the chain leaks into the wrapper's
// canonical name.
struct leaf_value_s { int v; };
typedef struct leaf_value_s leaf_value;

struct inner_ref_s { leaf_value* leaf; };
typedef struct inner_ref_s inner_ref;

struct middle_ref_s { inner_ref* inner; };
typedef struct middle_ref_s middle_ref;

struct outer_ref_s { middle_ref* middle; };
typedef struct outer_ref_s outer_ref;

#include "parse_skip_naming_chain/keep.h"

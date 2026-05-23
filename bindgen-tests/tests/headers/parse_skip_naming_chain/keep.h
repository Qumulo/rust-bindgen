// Allowlisted file. Has both a function returning a non-allowlisted typedef
// and a struct field referencing one — both should keep the canonical type
// name in the generated bindings.
typedef struct outer_ref_s outer_ref;

struct entry_in_allowlisted_file {
    outer_ref* outer;
};

outer_ref* get_outer(void);

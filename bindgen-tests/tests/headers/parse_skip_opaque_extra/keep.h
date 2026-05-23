// `future_t` is a typedef of `struct future`; both live in a
// non-allowlisted file. `struct future` has no definition anywhere in
// the TU. The allowlisted file references the typedef only through a
// pointer. The expectation is that flag-on and flag-off agree on the
// shape of the opaque `future` struct.
typedef struct future future_t;

void schedule(future_t* f);

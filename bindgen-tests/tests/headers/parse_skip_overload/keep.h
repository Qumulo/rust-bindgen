// One `as_sink` overload lives in the allowlisted file; the other two
// live in the non-allowlisted entry header. `compute_overload_suffixes`
// names overloads using argument types as suffixes — but only after
// seeing every overload. With the flag on, the other overloads'
// cursors are skipped, so the suffix on the visible one may differ
// unless `parse_overloads_of_allowlisted` materializes the siblings.
struct first_arg;
struct second_arg;
struct third_arg;

void as_sink(struct third_arg* z);

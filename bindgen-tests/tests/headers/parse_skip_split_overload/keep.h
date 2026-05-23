// The clang-overloadable version of `init` lives here in the allowlisted
// file. The plain version is in the non-allowlisted entry header. Both
// share the name "init" so they form a 2-element overload set.
struct fs_thing;
struct vfs_thing;

__attribute__((overloadable))
void init(struct fs_thing *, const struct vfs_thing *);

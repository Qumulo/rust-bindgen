// Forward declaration of a struct whose definition lives in a
// non-allowlisted file. With --parse-skip-non-allowlisted-files,
// the definition cursor is not visited at parse time, so this exercises
// the on-demand materialization path in CompInfo::from_ty.
struct DefinedInDroppedFile;

struct InAllowlistedFile {
    DefinedInDroppedFile* other;
    int x;
};

void allowlisted_function();

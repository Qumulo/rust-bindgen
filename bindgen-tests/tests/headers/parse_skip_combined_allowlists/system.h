#pragma once

// A type the user wants bindings for, but which lives in a file the
// `--allowlist-file` regex does NOT match. The user instead targets
// it via `--allowlist-type`. The lazy-parse skip MUST NOT drop this
// cursor — `should_parse_cursor` has a safety carve-out that
// disables the file-level skip whenever any non-file allowlist is
// non-empty, precisely to avoid this misfire.
struct named_via_type_allowlist {
    int a;
    int b;
};

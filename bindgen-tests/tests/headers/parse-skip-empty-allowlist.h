// bindgen-flags: --parse-skip-non-allowlisted-files
//
// `--parse-skip-non-allowlisted-files` without `--allowlist-file` must
// be a no-op. The skip filter has nothing to compare cursor source
// locations against, so leaving any cursor unparsed would silently
// drop user-requested bindings. Verify the output is the same shape
// as a vanilla bindgen run on the same header.

int g_count;

struct widget {
    int x;
    int y;
};

void widget_init(struct widget *);

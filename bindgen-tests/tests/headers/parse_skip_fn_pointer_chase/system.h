#pragma once

// Types reachable only through a function pointer's argument and
// return types. With the flag on, the chase must walk through
// function-type signatures, not just pointer indirections, or these
// types stay un-materialized.

struct arg_type {
    int a;
};

struct ret_type {
    int r;
};

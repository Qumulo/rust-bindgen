#pragma once

// Two of the three `widget_do` overloads live in this non-allowlisted
// file. The third is in decls.h.
[[clang::overloadable]] int widget_do(int);
[[clang::overloadable]] int widget_do(long);

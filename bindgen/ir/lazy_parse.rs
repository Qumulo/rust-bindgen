//! Pre-codegen passes that run when `--parse-skip-non-allowlisted-files`
//! is on, materializing items the main parse skipped but downstream
//! analyses still need.
//!
//! The flag, defined in `--parse-skip-non-allowlisted-files`, tells
//! `should_parse_cursor` to drop any cursor whose source location is
//! outside every `--allowlist-file` regex. That avoids building IR for
//! the bulk of a typical translation unit on codebases that pass huge
//! transitive include closures to bindgen while only allowlisting a
//! tiny output surface. The trade-off is that several existing
//! analyses implicitly assume they see every relevant cursor: overload
//! disambiguation, `<div rustbindgen replaces="X">` annotation
//! handling, and type-reference resolution. The passes in this module
//! re-introduce exactly the cursors those analyses need, one category
//! at a time:
//!
//! - `parse_replacement_decls` walks the TU for cursors whose raw
//!   comment carries `rustbindgen replaces="X"` and materializes them
//!   and their targets.
//! - `parse_overloads_of_allowlisted` finds non-allowlisted overload
//!   siblings of allowlisted functions so `compute_overload_suffixes`
//!   sees the full set.
//! - `parse_referenced_non_allowlisted` chases through pointer,
//!   array-element, typedef, canonical-type, and function-prototype
//!   (argument and return) indirections from allowlisted IR,
//!   materializing any reachable types that the skip dropped.
//! - `materialize_replacement_targets` cleans up the case where the
//!   chase introduces a `_for_rust` cursor whose target wasn't picked
//!   up by `parse_replacement_decls`.
//!
//! Each pass is gated on `parse_skip_non_allowlisted_files` and
//! returns early when no allowlist file regex is set, so this module's
//! footprint on the flag-off path is zero.

use super::context::{BindgenContext, TypeKey};
use super::item_kind::ItemKind;
use super::ty::TypeKind;
use crate::clang::{self, Cursor};
use crate::{HashMap, HashSet};

use super::context::{ModuleId, TypeId};

impl BindgenContext {
    /// Walk the TU for cursors whose raw doc comment carries a
    /// `<div rustbindgen replaces="X"></div>` annotation (and for the
    /// targets `X` themselves), and parse them regardless of whether
    /// their source location is allowlisted.
    ///
    /// `process_replacements` runs later and uses the table populated
    /// by `Item::from_ty_with_id` when it sees the annotation. With
    /// the flag off, both the `_for_rust` decl and its target are
    /// parsed by the normal walk, so the replacement entry lands in
    /// the table and `process_replacements` swaps the target's body
    /// for the `_for_rust` one (e.g. emitting
    /// `pub struct gss_OID_desc_struct {}` with the empty body from
    /// `gss_OID_desc_struct_for_rust`).
    ///
    /// With the flag on, both the annotated cursor and its target are
    /// often in a non-allowlisted system header. Without this pass,
    /// neither item gets created and the renamed binding silently
    /// vanishes from the output.
    pub(crate) fn parse_replacement_decls(&mut self) {
        if !self.options().parse_skip_non_allowlisted_files {
            return;
        }
        if self.options().allowlisted_files.is_empty() {
            return;
        }
        let _t = self.timer("parse_replacement_decls");

        let tu_cursor = self.translation_unit().cursor();
        let mut replacement_cursors: Vec<Cursor> = Vec::new();
        let mut targets: HashSet<String> = HashSet::default();
        collect_replacement_cursors(
            &tu_cursor,
            &mut replacement_cursors,
            &mut targets,
        );

        let mut target_cursors: Vec<Cursor> = Vec::new();
        if !targets.is_empty() {
            collect_target_cursors(&tu_cursor, &targets, &mut target_cursors);
        }

        if replacement_cursors.is_empty() && target_cursors.is_empty() {
            return;
        }

        self.force_parse_all = true;
        let mut seen_usrs: HashSet<String> = HashSet::default();
        for cursor in replacement_cursors.into_iter().chain(target_cursors) {
            if let Some(usr) = cursor.usr() {
                if !seen_usrs.insert(usr) {
                    continue;
                }
            }
            let module_id = self.module_for_cursor_parent(cursor);
            self.with_module(module_id, |ctx| {
                crate::parse_one(ctx, cursor, Some(module_id.into()));
            });
        }
        self.force_parse_all = false;
    }

    /// Walk the TU for function declarations whose spelling matches an
    /// allowlisted function's, and parse them too. Without this pass,
    /// `compute_overload_suffixes` sees only the allowlisted overload
    /// from each set and emits naming that doesn't match what bindgen
    /// produces with the flag off. The extra functions are
    /// non-allowlisted, so `compute_allowlisted_and_codegen_items`
    /// excludes them from emission — they only exist to inform naming.
    pub(crate) fn parse_overloads_of_allowlisted(&mut self) {
        if !self.options().parse_skip_non_allowlisted_files {
            return;
        }
        if self.options().allowlisted_files.is_empty() {
            return;
        }
        let _t = self.timer("parse_overloads_of_allowlisted");

        // Walk the TU once, grouping every function-declaration cursor
        // by its raw spelling. Two passes through the groups:
        //
        // 1. Identify "overload sets of interest" — groups where at
        //    least one cursor is in an allowlisted file. The
        //    non-allowlisted cursors in those groups are overload
        //    siblings of an emitted function; without them,
        //    `compute_overload_suffixes` sees an incomplete overload
        //    set and either emits no disambiguation suffix at all or
        //    emits an inconsistent one.
        //
        // 2. Parse the non-allowlisted siblings, deduplicating by USR
        //    so redeclarations of the same overload (e.g. one in a
        //    header and one in an inline `_gen.h`) only produce one
        //    Item.
        //
        // Grouping by cursor `spelling()` rather than
        // `Function::name()` matters because bindgen rewrites some
        // function names at item construction time (destructors become
        // `Foo_destructor`, `generated_name_override` callbacks rename
        // arbitrarily). Those rewrites would make name comparison miss
        // the corresponding cursors. Deduping by USR (and not by
        // mangled name) means this works when
        // `--distrust-clang-mangling` is on, where
        // `Function::mangled_name` is `None` for every Function Item.
        let tu_cursor = self.translation_unit().cursor();
        let mut by_spelling: HashMap<String, Vec<Cursor>> =
            HashMap::default();
        collect_function_cursors_by_spelling(&tu_cursor, &mut by_spelling);

        let in_allowlist = |c: &Cursor| -> bool {
            let (file, _, _, _) = c.location().location();
            file.name()
                .is_some_and(|n| self.options().allowlisted_files.matches(&n))
        };

        let mut to_parse: Vec<Cursor> = Vec::new();
        let mut already_parsed_usrs: HashSet<String> = HashSet::default();
        for group in by_spelling.values() {
            if !group.iter().any(in_allowlist) {
                continue;
            }
            for c in group {
                if in_allowlist(c) {
                    if let Some(usr) = c.usr() {
                        already_parsed_usrs.insert(usr);
                    }
                    continue;
                }
                if let Some(usr) = c.usr() {
                    if !already_parsed_usrs.insert(usr) {
                        continue;
                    }
                }
                to_parse.push(*c);
            }
        }

        if to_parse.is_empty() {
            return;
        }

        self.force_parse_all = true;
        for cursor in to_parse {
            let module_id = self.module_for_cursor_parent(cursor);
            self.with_module(module_id, |ctx| {
                crate::parse_one(ctx, cursor, Some(module_id.into()));
            });
        }
        self.force_parse_all = false;
    }

    /// Iteratively chase types referenced from allowlisted items
    /// (through pointers, typedefs, and canonical-type indirections)
    /// and materialize their declarations even when the main parse
    /// skipped them. Loops to a fixed point so newly-materialized
    /// items get their references chased too.
    pub(crate) fn parse_referenced_non_allowlisted(&mut self) {
        if !self.options().parse_skip_non_allowlisted_files {
            return;
        }
        if self.options().allowlisted_files.is_empty() {
            return;
        }
        let _t = self.timer("parse_referenced_non_allowlisted");

        let mut parsed_usrs: HashSet<String> = HashSet::default();

        loop {
            let mut wanted: Vec<(Cursor, String)> = Vec::new();
            let mut wanted_seen: HashSet<String> = HashSet::default();

            let unresolved: Vec<(clang::Type, Cursor)> = self
                .items
                .iter()
                .filter_map(|opt| opt.as_ref())
                .filter_map(|item| {
                    let ty = item.kind().as_type()?;
                    if let TypeKind::UnresolvedTypeRef(ref t, loc, _) =
                        *ty.kind()
                    {
                        Some((*t, loc))
                    } else {
                        None
                    }
                })
                .collect();

            for (ty, loc) in unresolved {
                chase_canonical_decls(
                    &ty,
                    &loc,
                    &parsed_usrs,
                    &self.types,
                    &mut wanted_seen,
                    &mut wanted,
                );
            }

            if wanted.is_empty() {
                break;
            }

            // Resolve a natural module per cursor (mutable borrow
            // needed because Namespace lookup may insert Module
            // Items).
            let with_modules: Vec<(Cursor, ModuleId, String)> = wanted
                .into_iter()
                .map(|(cursor, usr)| {
                    let module_id = self.find_natural_module(cursor);
                    (cursor, module_id, usr)
                })
                .collect();

            self.force_parse_all = true;
            for (cursor, module_id, usr) in with_modules {
                parsed_usrs.insert(usr);
                self.with_module(module_id, |ctx| {
                    crate::parse_one(ctx, cursor, Some(module_id.into()));
                });
            }
            self.force_parse_all = false;
        }
    }

    /// Scan `self.replacements` for any target names whose target Item
    /// hasn't been materialized yet, and parse a matching cursor for
    /// each from the TU. Returns `true` if at least one cursor was
    /// parsed (the caller will then re-run
    /// `parse_referenced_non_allowlisted` to chase any non-allowlisted
    /// types reachable from the newly-materialized targets).
    ///
    /// This handles the case where `parse_referenced_non_allowlisted`
    /// materialized a `_for_rust` cursor (registering a new entry in
    /// `self.replacements`) whose target wasn't named by the initial
    /// `parse_replacement_decls` scan.
    pub(crate) fn materialize_replacement_targets(&mut self) -> bool {
        if !self.options().parse_skip_non_allowlisted_files {
            return false;
        }
        if self.options().allowlisted_files.is_empty() {
            return false;
        }
        if self.replacements.is_empty() {
            return false;
        }
        let _t = self.timer("materialize_replacement_targets");

        // The target names are the last segment of each replacement
        // path (see `process_replacements`).
        let mut targets: HashSet<String> = self
            .replacements
            .keys()
            .filter_map(|path| path.last().cloned())
            .collect();

        // Drop targets that an Item already represents — by name,
        // scoped to the same path under root. `process_replacements`
        // will match these correctly; we don't need to materialize a
        // second time.
        let existing: HashSet<String> = self
            .items
            .iter()
            .filter_map(|opt| opt.as_ref())
            .filter_map(|item| match item.kind() {
                ItemKind::Type(ref ty) => ty.name().map(String::from),
                _ => None,
            })
            .collect();
        targets.retain(|t| !existing.contains(t));

        if targets.is_empty() {
            return false;
        }

        let tu_cursor = self.translation_unit().cursor();
        let mut target_cursors: Vec<Cursor> = Vec::new();
        collect_target_cursors(&tu_cursor, &targets, &mut target_cursors);

        if target_cursors.is_empty() {
            return false;
        }

        // Track whether `parse_one` actually materialized a new Item
        // for any target by watching `self.items` grow. Using a naive
        // counter would spin the outer fixed-point loop forever (well,
        // until any harness bound trips) on a malformed `replaces=`
        // target whose `parse_one` consistently no-ops.
        let items_before = self.items.len();
        let mut seen_usrs: HashSet<String> = HashSet::default();
        self.force_parse_all = true;
        for cursor in target_cursors {
            if let Some(usr) = cursor.usr() {
                if !seen_usrs.insert(usr) {
                    continue;
                }
            }
            let module_id = self.module_for_cursor_parent(cursor);
            self.with_module(module_id, |ctx| {
                crate::parse_one(ctx, cursor, Some(module_id.into()));
            });
        }
        self.force_parse_all = false;
        self.items.len() > items_before
    }

    /// Resolve the module under which to parse a cursor whose parent
    /// chain we want to walk. Returns the root module if the cursor
    /// has no semantic parent (top-level in the TU) or if the parent
    /// chain doesn't lead through a recognizable namespace.
    fn module_for_cursor_parent(&mut self, cursor: Cursor) -> ModuleId {
        cursor
            .fallible_semantic_parent()
            .map_or(self.root_module, |p| self.find_natural_module(p))
    }

    /// Translate a clang cursor's semantic-parent chain into a bindgen
    /// `ModuleId`. Used by the lazy-parse passes to choose the natural
    /// scope to parse a referenced cursor under. Falls back to the
    /// root module if the chain doesn't lead through namespaces we can
    /// resolve.
    fn find_natural_module(&mut self, cursor: Cursor) -> ModuleId {
        use clang_sys::*;
        // Hard cap on the parent-walk depth. Real C++ namespace
        // nesting tops out in the low single digits; a clang AST that
        // returns itself as its own parent (which we've never seen but
        // can't rule out across libclang versions) would otherwise
        // loop forever here. Fall back to the root module if we
        // exceed it.
        const MAX_PARENT_DEPTH: usize = 256;
        let Some(mut sem_parent) = cursor.fallible_semantic_parent() else {
            return self.root_module;
        };
        for _ in 0..MAX_PARENT_DEPTH {
            match sem_parent.kind() {
                CXCursor_TranslationUnit => return self.root_module,
                CXCursor_Namespace => {
                    return self.module(sem_parent);
                }
                _ => {}
            }
            let Some(next) = sem_parent.fallible_semantic_parent() else {
                return self.root_module;
            };
            sem_parent = next;
        }
        self.root_module
    }
}

/// Walk the TU collecting cursors whose raw doc comment carries a
/// `<div rustbindgen replaces="X"></div>` annotation, and the set of
/// target names `X`. Container cursors (`Namespace`, `LinkageSpec`,
/// `UnexposedDecl`, and the struct/class/enum kinds that can hold
/// nested type declarations) are descended into; everything else is
/// leaf-checked.
fn collect_replacement_cursors(
    cursor: &Cursor,
    replacements: &mut Vec<Cursor>,
    targets: &mut HashSet<String>,
) {
    use clang_sys::*;
    cursor.visit(|child| {
        if let Some(comment) = child.raw_comment() {
            // Cheap pre-filter before the full `Annotations` parse:
            // most cursors have no doc comment, and of those that do,
            // the `replaces=` literal is unmistakable.
            if comment.contains("rustbindgen") &&
                comment.contains("replaces")
            {
                if let Some(ann) =
                    crate::ir::annotations::Annotations::new(&child)
                {
                    if let Some(path) = ann.use_instead_of() {
                        replacements.push(child);
                        // Replacement targets are matched by the last
                        // path segment (the type name) under
                        // `process_replacements`. Capture just that
                        // segment here.
                        if let Some(name) = path.last() {
                            targets.insert(name.clone());
                        }
                    }
                }
            }
        }
        match child.kind() {
            // Container kinds that may hold member declarations
            // carrying a `replaces=` annotation. Class/struct bodies
            // matter because upstream's own `replaces_double.hpp` and
            // `template.hpp` fixtures put `replaces=` annotations on
            // members of an enclosing class. Enum bodies don't hold
            // type decls in C/C++, but we descend them so the visitor
            // is symmetric with `collect_target_cursors` below.
            CXCursor_Namespace |
            CXCursor_LinkageSpec |
            CXCursor_UnexposedDecl |
            CXCursor_StructDecl |
            CXCursor_UnionDecl |
            CXCursor_ClassDecl |
            CXCursor_ClassTemplate |
            CXCursor_EnumDecl => {
                collect_replacement_cursors(&child, replacements, targets);
            }
            _ => {}
        }
        CXChildVisit_Continue
    });
}

/// Walk the TU collecting struct / union / enum / typedef cursors
/// whose name matches one of `targets`. These are the targets named
/// by a `replaces="X"` annotation; we materialize them so
/// `process_replacements` has something to rewrite into the
/// replacement body.
fn collect_target_cursors(
    cursor: &Cursor,
    targets: &HashSet<String>,
    into: &mut Vec<Cursor>,
) {
    use clang_sys::*;
    cursor.visit(|child| {
        match child.kind() {
            CXCursor_StructDecl |
            CXCursor_UnionDecl |
            CXCursor_EnumDecl |
            CXCursor_TypedefDecl |
            CXCursor_ClassDecl |
            CXCursor_ClassTemplate
                if targets.contains(&child.spelling()) =>
            {
                into.push(child);
            }
            _ => {}
        }
        match child.kind() {
            // Same container kinds as `collect_replacement_cursors`:
            // a `replaces=` target named by a nested annotation may
            // itself be nested inside a class body.
            CXCursor_Namespace |
            CXCursor_LinkageSpec |
            CXCursor_UnexposedDecl |
            CXCursor_StructDecl |
            CXCursor_UnionDecl |
            CXCursor_ClassDecl |
            CXCursor_ClassTemplate |
            CXCursor_EnumDecl => {
                collect_target_cursors(&child, targets, into);
            }
            _ => {}
        }
        CXChildVisit_Continue
    });
}

/// Walk the cursor tree collecting every function-declaration cursor,
/// grouped by its raw `cursor.spelling()`. Descends through both
/// namespace-like containers (`Namespace`, `LinkageSpec`,
/// `UnexposedDecl`) and class/struct/union bodies (`StructDecl`,
/// `UnionDecl`, `ClassDecl`, `ClassTemplate`) — the latter so C++
/// methods, constructors, and destructors declared inside class
/// bodies get picked up as members of their parent's overload set.
fn collect_function_cursors_by_spelling(
    cursor: &Cursor,
    into: &mut HashMap<String, Vec<Cursor>>,
) {
    use clang_sys::*;
    cursor.visit(|child| {
        match child.kind() {
            CXCursor_FunctionDecl |
            CXCursor_CXXMethod |
            CXCursor_Constructor |
            CXCursor_Destructor |
            CXCursor_FunctionTemplate => {
                into.entry(child.spelling()).or_default().push(child);
            }
            _ => {}
        }
        match child.kind() {
            CXCursor_Namespace |
            CXCursor_LinkageSpec |
            CXCursor_UnexposedDecl |
            CXCursor_StructDecl |
            CXCursor_UnionDecl |
            CXCursor_ClassDecl |
            CXCursor_ClassTemplate => {
                collect_function_cursors_by_spelling(&child, into);
            }
            _ => {}
        }
        CXChildVisit_Continue
    });
}

/// Recursively follow a clang `Type` chasing pointer / array /
/// typedef indirections, collecting any not-yet-parsed canonical
/// declaration cursors into `into`. `seen` deduplicates within the
/// current round; `parsed_usrs` and `cache` filter out
/// already-handled USRs.
fn chase_canonical_decls(
    ty: &clang::Type,
    loc: &Cursor,
    parsed_usrs: &HashSet<String>,
    cache: &HashMap<TypeKey, TypeId>,
    seen: &mut HashSet<String>,
    into: &mut Vec<(Cursor, String)>,
) {
    if let Some(decl) = ty.canonical_declaration(Some(loc)) {
        // Prefer the definition cursor over the canonical declaration
        // cursor. Item::parse short-circuits a forward decl whose
        // definition lives elsewhere into an `UnresolvedTypeRef`
        // placeholder rather than fully materializing the type —
        // which would leave `self.types` without a USR entry and
        // cause later elaborated-type lookups (e.g. `struct foo *`
        // arg types) to miss the cache and bypass the wrapper-Item
        // path, producing divergent sanitized names like
        // `ptr_fs_foo` instead of `ptr_struct_fs_foo`.
        let cursor = decl.cursor().definition().unwrap_or(*decl.cursor());
        if let Some(usr) = cursor.usr() {
            if !parsed_usrs.contains(&usr) &&
                !cache.contains_key(&TypeKey::Usr(usr.clone())) &&
                !cache.contains_key(&TypeKey::Declaration(cursor)) &&
                seen.insert(usr.clone())
            {
                into.push((cursor, usr));
            }
        }
    }
    if let Some(pointee) = ty.pointee_type() {
        chase_canonical_decls(&pointee, loc, parsed_usrs, cache, seen, into);
    }
    if let Some(elem) = ty.elem_type() {
        chase_canonical_decls(&elem, loc, parsed_usrs, cache, seen, into);
    }
    // Function (and function-pointer) types hide their argument /
    // return types in `args()` and `ret_type()` — neither
    // `pointee_type` nor `elem_type` nor `canonical_type` exposes
    // them. Without chasing here, a typedef like
    // `typedef int (*cb_t)(struct skipped *)` leaves `skipped`
    // un-materialized when only the typedef is allowlisted.
    if let Some(args) = ty.args() {
        for arg in &args {
            chase_canonical_decls(arg, loc, parsed_usrs, cache, seen, into);
        }
    }
    if let Some(ret) = ty.ret_type() {
        chase_canonical_decls(&ret, loc, parsed_usrs, cache, seen, into);
    }
    let canonical = ty.canonical_type();
    if canonical != *ty {
        chase_canonical_decls(
            &canonical,
            loc,
            parsed_usrs,
            cache,
            seen,
            into,
        );
    }
}

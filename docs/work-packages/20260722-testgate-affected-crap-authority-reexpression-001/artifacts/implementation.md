# Implementation

Static: scaffold commit `ae5f33b1` predates all aggregate implementation edits.

Static: `CargoGraph` now retains a production-owner set derived from reconstructed
Cargo target metadata. Admission requires a direct `crates/*` member, a nonempty
target-kind set wholly within library/binary/proc-macro, and an owned Rust target
under that package's `src/` tree. Root, test-only-under-src, and out-of-tree
targets remain measurement-only and force global quality when directly changed.

Static: executor fixtures use one `global_quality_gate_definition` helper.
Planner tests use one `assert_measurement_only_global_risk` helper. Existing
stage, receipt, mutation, affected-production, and global-selection assertions
remain intact.

Static: the preceding checker correction parses and hashes one retained scope
byte buffer and rechecks its digest immediately before PASS publication.

Static: final production ownership additionally requires an exact plain
`crates/*/Cargo.toml`, a plain source-root directory, and an existing plain
lowercase-`.rs` target reached only through normal path components. Missing
sources, lexical traversal, target symlinks, symlinked source ancestors,
uppercase extensions, test-only targets, root packages, and out-of-tree targets
fail closed without restoring per-target filesystem canonicalization.

Static: exact reviewed correction commit is
`85d706ed4fefc2011bd23c56f60688a7ba5e63ba`.

# Git Object Archaeology

Evidence class: `Ran`

The read-only search covered all refs and reflogs, both `refs/codex` turn-diff
refs, `git fsck --full --unreachable --no-reflogs`, and every object reported by
`git cat-file --batch-all-objects --batch-check`. There are no stash entries,
no unreachable commits, and no alternate V3/V5 generator/vector/definition
versions in path history or reflogs.

The full object database contains 46,976 blobs. A SHA-256 scan found none of
the previously recorded superseded generator/vector/definition identities,
including V3 `50a6366e...` / `bf0edfa9...`, V5 initial `104be4a2...` /
`50bb7457...`, or V5 NO-GO `77f19fe5...` / `b243658f...` / `1b5d05ec...`.
The 13 unreachable blobs contain Child-3 logs/roadmap text, not authority code
or fixtures.

Git cannot recover bytes that were never hashed, were garbage-collected, lived
outside this object database, or were overwritten in the filesystem. Within
the local repository and shared object database, no historical matching
generator or generated fixture survives.

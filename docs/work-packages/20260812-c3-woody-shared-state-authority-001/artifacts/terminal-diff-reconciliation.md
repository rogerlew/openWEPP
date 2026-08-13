# Terminal Diff Reconciliation

Status: `complete / terminal verified`

Evidence mode: `Static`

Scoped authority changes are limited to this package tree,
`SC-VEGETATION-001.md`, the registry, and package catalog. Concurrent Rust
implementation changes are outside this package and excluded from its
authority ownership. V1/V2/V3 definitions remain unchanged at their protected
digests. Current V4 identities are definition
`571bac78b6f116078b463021ec0a36a5206cbe14a94d9fdc76bc32c0a7cde327`,
fixture `6862b507cf54b57606304d4a7b01cffe55dd3f90b2a2b0d44601fe103e2841a7`,
and generator `5ac8dfea31270a7cd7e213e29ffff9efc7cde8bb5e9333aa69add5100b0872c3`.

Exact scoped paths are this package tree, `SC-VEGETATION-001.md`, the science
contract registry, and the package catalog. The three modified vegetation Rust
files shown by the whole-worktree diff are concurrent implementation-campaign
ownership and are explicitly outside this package's write set; the heavy gates
nevertheless validated the complete current workspace containing them.

All failed, interrupted, and ENOSPC heavy attempts are preserved. The final
capacity-correct run is
`artifacts/v4-closure-final-stable-20260813-004136/`: workspace Clippy PASS,
full-workspace nextest PASS (3,217 seconds), workspace doctests PASS, deny PASS,
format PASS, and diff hygiene PASS. Final science reviews A and B are GO with no
unresolved material finding. The active prompt remains correctly retained until
both terminal verifiers returned PASS. The kickoff prompt was then moved
byte-for-byte from `prompts/active/` to `prompts/archived/`, preserving SHA-256
`7f31e3a82634aaab31aa9de2d4bf5ac9bfd34c11241671fb3a80685b6839df25`.

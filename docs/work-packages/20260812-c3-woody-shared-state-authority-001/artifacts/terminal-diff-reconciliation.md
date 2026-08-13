# Terminal Diff Reconciliation

Status: `complete / terminal verified`

Evidence mode: `Static`

Scoped authority changes are limited to this package tree,
`SC-VEGETATION-001.md`, the registry, and package catalog. Concurrent Rust
implementation changes are outside this package and excluded from its
authority ownership. V1/V2/V3 definitions remain unchanged at their protected
digests. Current remediation-candidate V4 identities are definition
`8ace38d1148f95261306cd6b0bf6f22e23ac8ead4cb6897dbdb53061b78ee437`,
fixture `3072226f1d80359c548d87c1fa222be0c20b01627d9117e39163c39d9eb8824d`,
and generator `422f0a6fb778de73568259b0d1bad19f63e5b6fcac5fd608accace45b316bcd2`.
The previously terminal-verified `571bac78.../6862b507.../5ac8dfea...`
checkpoint remains historical evidence but is superseded as implementation
authority because its whole-state oracle flattened occupancy identity.

Exact scoped paths are this package tree, `SC-VEGETATION-001.md`, the science
contract registry, and the package catalog. Vegetation Rust files shown by the
whole-worktree diff are concurrent implementation-campaign ownership and are
explicitly outside this package's write set.

The final remediation baseline is clean commit
`916f24181e250d1cee5b17d9985bb082b7b53a3f`. That commit includes the matching
production registry definition and implementation bindings reviewed by the
independent authority reviewers, but this package claims no Rust ownership or
runtime activation. The post-baseline authority-package diff contains only the
new heavy-run evidence and this package's closure evidence. It does not change
the frozen contract, definition, fixture, generator, Rust, implementation
package, runtime selector, or predecessor authority.

Historical failed, interrupted, and ENOSPC heavy attempts are preserved. The
previously accepted capacity-correct run was
`artifacts/v4-closure-final-stable-20260813-004136/`: workspace Clippy PASS,
full-workspace nextest PASS (3,217 seconds), workspace doctests PASS, deny PASS,
format PASS, and diff hygiene PASS, against the superseded V4 identity. That
historical run does not substitute for the corrected closure. The prior
archived prompt remains immutable historical evidence.

The authoritative post-remediation heavy campaign is
`artifacts/run-20260813T030632Z-ZzxORt/`, bound to commit `916f2418...` and
definition/fixture/generator `8ace38d1.../3072226f.../422f0a6f...`. All six
commands passed in one uninterrupted campaign. Dual science rereviews A and B
are GO with no unresolved material finding. Two fresh terminal verifiers
assessed these exact bytes and both returned PASS with no unresolved material
finding. The package-only closure diff contains the heavy run directory, final
evidence updates, and the two fresh verifier artifacts; it contains no Rust,
implementation-package, contract, definition, fixture, generator, selector, or
predecessor-authority change.

# R7G Winter-Column Mechanical Containment Prompt

You are working in `/home/workdir/openWEPP` on the package
`docs/work-packages/20260623-r7g-winter-column-mechanical-containment-001/`.

Execute the package end to end. This is a mechanical containment package, not a
solver migration. Add the ADR-0026 winter-column module boundary outside
`direct_runtime` phase modules, add inert direct-frame ownership hooks, prove no
compatibility/symbol-surface authority enters the new module, and run the
package closure gates. Do not move snow/frost solver math or delete existing
direct-runtime snow/frost retrofit code in this package.

Subagent authorization: this package explicitly authorizes spawning/delegating
to read-only reviewer and verifier subagents for architecture-boundary review,
mechanical-refactor review, no-compatibility scan review, line-count governance,
and final disposition. Subagents may not edit files. Findings must be
dispositioned in `artifacts/review-disposition.md` and
`artifacts/verification.md`.

Required gates:

1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo deny check`

Record evidence in package artifacts and close only as `COMPLETE` when every
current-scope gate passes. If a gate cannot run, record the command-level
blocker and close as `HOLD`.

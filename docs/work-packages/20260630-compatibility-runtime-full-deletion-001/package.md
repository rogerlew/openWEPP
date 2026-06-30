# Compatibility Runtime Full Deletion

Status: EXECUTED-HOLD-SYMBOL-SURFACE-SUPPORT-BOUNDARY

Package id: `20260630-compatibility-runtime-full-deletion-001`

## Objective

Delete the remaining symbol-map compatibility runtime and the explicit
`--compatibility-runtime` seam so the typed `DirectRunFrame` is the sole
production runtime representation from parse to output.

## Authority

- [ADR-0031](../../decisions/0031-delete-compatibility-runtime-single-authority-terminal.md)
  ratifies the full-deletion decision and supersedes ADR-0030's seam-retention
  clause.
- [ADR-0030](../../decisions/0030-r7-terminal-contract-and-compatibility-runtime-deletion.md)
  remains the prior partial-deletion authority and is superseded only on the
  retained seam.
- [Array-native runtime spec](../../architecture/array-native-runtime-specification.md)
  §0 and §8.2 define the single-authority terminal state.
- `20260630-typed-day-zero-seed-computation-001/` proves the typed seed cutover
  and records the prior hold at the ADR-0030 seam boundary.

## Scope

1. Remove the `Compatibility` runtime selection and `--compatibility-runtime`
   CLI flag.
2. Delete the executable symbol-map scheduler/day-frame runtime and legacy tests
   it exclusively backs.
3. Delete carrier types and vestigial runner/orchestrator references where they
   are executable alternate-runtime machinery.
4. Keep symbol-keyed types only where a real intake/output serialization edge
   still requires them, and document any survivors.
5. Update ROADMAP item W and the array-native spec to record the realized
   deletion state or the hold boundary.

## Non-Scope

- No physics change.
- No output schema change.
- No watershed CLI behavior change except inheriting the runtime that produced
  hillslope outputs.
- No direct-native replay implementation.

## Gates

- ADR-0031 ratified before code deletion.
- H2637 plus one multi-OFE and one Wave-2 fixture preserve protected outputs:
  HBP/loss/manifest byte-identical; WAT/PASS schema, row-count, and value
  identity.
- `--compatibility-runtime` is gone from CLI/API/runtime selection.
- No production entrypoint reaches the deleted symbol-map runtime; direct
  manifests keep `compatibility_edge_invocations=0`.
- H2637 RSS remains run-length-flat and endpoint time is recorded.
- Full closure gates: `cargo fmt --check`, `cargo clippy --workspace
  --all-targets -- -D warnings`, `cargo nextest run --workspace --profile full`,
  `cargo deny check`, authority anti-evasion, required-suite obligation guard,
  Markdown lint/validate, and `git diff --check`.

## Stop Conditions

- Any output identity break.
- A production path still reaches symbol-map execution.
- A non-environment full-gate failure.
- A symbol-keyed type that cannot be deleted because a genuine I/O edge needs it;
  document the edge and hold rather than forcing removal.

## Disposition

Result: `EXECUTED-HOLD-SYMBOL-SURFACE-SUPPORT-BOUNDARY`.

ADR-0031 is ratified and the public compatibility selector is removed from the
Rust API, `openwepp-cli-hill`, and the observed frost harness. The focused
runner surface is green under the direct-only selector. Full deletion of
`scheduler.rs`, `day_frame.rs`, and the carrier types is held because removing
the unreachable scheduler execution branch exposed a broad compiled support
surface: scheduler lifecycle, WB13 scheduler publication, HPHYS trace helpers,
and legacy scheduler tests become dead code and must be deleted or replaced as a
coherent support-boundary removal. Per the stop condition, this package does not
force-delete symbol-keyed support types that are still genuine test/intake/output
edges.

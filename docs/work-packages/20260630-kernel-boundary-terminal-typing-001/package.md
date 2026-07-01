# Kernel-Boundary Terminal Typing

Status: EXECUTED-COMPLETE-TERMINAL-SINGLE-AUTHORITY

Package id: `20260630-kernel-boundary-terminal-typing-001`

## Objective

Complete the kernel-boundary typing re-architecture as one coordinated program:
make the typed `DirectRunFrame` and its typed publication/event streams the sole
production kernel-invocation authority, remove the compiled symbol-map
scheduler boundary, and prove the terminal single-authority state.

## Authority

- [Array-native runtime specification](../../architecture/array-native-runtime-specification.md)
  §8.2: kernel-boundary typing program and terminal proof requirements.
- [ADR-0031](../../decisions/0031-delete-compatibility-runtime-single-authority-terminal.md):
  compatibility runtime deletion authority and kernel-boundary follow-on.
- `20260630-kernel-boundary-survivor-inventory-001/`: survivor classes,
  deletion targets, and no-compatibility proof shape.
- `20260630-kernel-boundary-typed-diagnostic-events-001/`: carried-forward
  typed direct-runtime diagnostic event payloads.

## Scope

In scope:

- Remove unreachable non-direct runner execution and scheduler-era publication
  support.
- Remove `scheduler.rs`, `day_frame.rs`, scheduler trace consumers, carrier
  exports, and tests that exist only for the compiled symbol-map runtime.
- Preserve typed direct production execution, publication, manifest, and
  diagnostics.
- Rescan `BoundarySymbol`/`BoundaryValue` survivors and classify any remaining
  references as I/O/intake adapters or tests, not executable runtime authority.

Out of scope:

- No physics change.
- No output-column schema change.
- No watershed CLI changes.

## Execution Plan

1. Baseline scans and package scaffold.
2. Delete the unreachable non-direct runner branch and scheduler-era execution
   result structs.
3. Delete scheduler/day-frame modules, carrier exports, and scheduler-only
   tests.
4. Compile-guided cleanup of orphan imports, helpers, and documentation.
5. Run no-compatibility source scan, identity gates, full workspace gates, and
   record final evidence.

## Gates

| Gate | Requirement |
| --- | --- |
| Identity | H2637 + multi-OFE + Wave-2 protected outputs unchanged; HBP/loss/manifest byte-identical, WAT/PASS value/schema/row-count identical. |
| No compatibility | `compatibility_edge_invocations=0`, no public selector, no production entrypoint reaches scheduler/day-frame/carrier runtime. |
| RSS/time | H2637 remains run-length-flat and <=10x; record the <=5x re-measure. |
| Rust full gates | `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo nextest run --workspace --profile full`, `cargo deny check`. |
| Release guards | Authority anti-evasion and required-suite obligation guard. |
| Docs | Markdown lint/validate for touched docs. |

## Disposition

Complete. The symbol-map scheduler/day-frame runtime and carrier boundary were
deleted from the production build; production direct execution is the only
hillslope runtime representation. Remaining `BoundarySymbol`/`BoundaryValue`
survivors are typed guard/reporting helpers or watershed/intake I/O adapters,
not executable alternate-runtime authority.

Closure evidence is recorded in `artifacts/gates.md`,
`artifacts/verification.md`, `artifacts/progress.md`, and
`artifacts/review.md`.

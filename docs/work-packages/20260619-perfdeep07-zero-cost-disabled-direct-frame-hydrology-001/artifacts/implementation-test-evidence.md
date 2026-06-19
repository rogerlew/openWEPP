# Implementation and Test Evidence

Status: HOLD.
Evidence mode: Static/Ran.

## Production Edits Retained

- `HillslopeKernelRequest::indexed_state_value` and `indexed_flux_value`
  bypass dense-slot probing when no dense surface exists.
- Hydrology state/flux access only attempts dense lookup when dense surfaces
  are present.
- `HotSymbolTables` hot symbol maps use `HashMap` instead of `BTreeMap`.
- Runner indexed scheduler resources are optional at the context boundary and
  required fail-closed only when indexed execution or explicit diagnostics need
  them.
- A kernel-contract test covers indexed requests with no dense slots.

## Timing Evidence

Best retained candidate:

- Dense-absent indexed bypass plus `HashMap` hot tables:
  `685.85 s`, RSS `229004 KB`.

Rejected experiments:

- Plain no-indexed path: `753.38 s`, then `755.48 s`.
- Forced indexed-surface rebuild: `1035.90 s`.
- Indexed-surface report propagation: `1054.71 s`.
- Hot-absent bypass variant: `688.54 s`.

P0 result: FAIL. The required disabled-path median threshold is
`<= 676.67 s`; no viable single-run candidate reached the threshold, so the
three-run median gate was not completed.

## Identity Evidence

Compared retained output
`/tmp/perfdeep07/hash-hot/rep1/h2637_same` against
`/tmp/perfdeep05/default/h2637_same`:

- HBP SHA-256 equal:
  `44acc83b025b7a7ed9df3ad77f2d595a17f7e59ae923a1224f8ee294ad09bfe8`.
- WAT SHA-256 equal:
  `c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474`.
- PASS Arrow schema/table equality: pass, `12419` rows each.
- WAT Arrow schema/table equality: pass, `235961` rows each.
- Loss JSON and plot sidecar text differed only by `run_name`.

## Commands Run

- `cargo fmt --check`
- `cargo test -p openwepp-kernel-contract indexed_request_without_dense_slots_keeps_dense_surface_absent`
- `cargo test -p openwepp-hillslope-orchestrator writeback`
- `cargo test -p openwepp-runner`
- `cargo test -p openwepp-hillslope-orchestrator perfidx03b_persistent_state_refreshes_indexed_writeback_surface`
- `markdown-doc lint --path docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001 --format json`
- `git diff --check`

Final command status is recorded in `gate-results.md`. The scoped markdown
lint scanned `28` files with `0` errors and `0` warnings.

## Skipped

- Direct-frame opt-in implementation and H2637 opt-in identity/timing:
  blocked by P0 default-disabled timing failure.
- `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check`: not run because this is a
  HOLD disposition, not an implementation completion claim.

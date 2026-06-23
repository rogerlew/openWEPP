# R7D4 Direct MOFE Dynamic Carry Transfer

Status: executed-held.

Package type: Array-native runtime defect-closure implementation package.

Objective: close `HOLD-R7D3-DIRECT-MOFE-DYNAMIC-CARRY-TRANSFER-ABSENT` and
continue `R7D-DIRECT-PRODUCTION-PUBLICATION-PARITY` blocker closure.

Rationale: R7D3 promoted direct WB14/R4K infiltration/depression producer
authority and made H2637 direct production execute to completion, but H2637
HBP/WAT/PASS parity remains blocked because direct MOFE same-day transfer is
absent. Direct R4O/R4L can produce hourly current-lane lateral and
surface-saturation carry, but `DirectFrameExecutor` commits each lane only to
itself. Downstream lanes therefore start R3A/R4J with zero dynamic
`ui_SUrunf`/`ui_LfUrf`, which drives zero `UpStrmQ`, zero `SubRIn`, zero
downstream `QOFE`, PASS `runvol` divergence, and large WAT storage/runoff
residuals.

Included scope:

- Add typed direct same-day MOFE upstream-to-downstream transfer production in
  `DirectFrameExecutor`.
- Publish current-lane R4O/R4L `ui_LfCrf` and `ui_SCrunf` arrays from direct
  shadows into transfer output buffers.
- Copy those arrays with declared area-ratio provenance into the downstream
  lane as `ui_LfUrf` and `ui_SUrunf` before downstream R3A/R4J execution.
- Make R4J consume typed dynamic transfer buffers rather than constructor
  handoffs alone.
- Preserve current single-OFE behavior as the zero-upstream specialization.
- Add focused anti-alias fixtures proving downstream `UpStrmQ`, `SubRIn`,
  `QOFE`, and storage/runoff closure respond to upstream hourly carry arrays.
- Re-run H2637 direct/default parity. If the next blocker is still
  in-envelope, keep correcting and rerunning until R7D closes or a named
  out-of-envelope blocker is proven.

Excluded scope:

- Default activation; compatibility remains default.
- R7G performance/RSS remediation unless needed to keep H2637 evidence
  runnable.
- Sediment-coupled EROD14 `qin/qout` particle-fraction handoff. Water-transfer
  closure may improve PASS `runvol`/`sbrunv`, but true sediment coupling
  remains out of scope unless it becomes the only remaining parity blocker.
- Using compatibility WB13 rows, compatibility public-output builders,
  aggregate runtime-surface carry aliases, or stale logical state as production
  direct authority.

Intended write set:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime*.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `crates/openwepp-runner/src/hillslope/04_direct_publication.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260622-r7d4-direct-mofe-dynamic-carry-transfer-001/**`

Dependencies:

- R7D3 direct WB14/R4K infiltration producer and R4L saturation-addback
  consumption.
- `SC-RUNOFFPART-001` `INV-RUNOFFPART-013`, `014`, `028`, and `029`.
- `SC-WATBAL-001` MOFE hourly carry-array and inter-OFE conservation
  addenda.

Correction authority envelope:

- Defect: `R7D-DIRECT-PRODUCTION-PUBLICATION-PARITY`.
- Observed failure: H2637 direct production exits 0 with
  `compatibility_edge_invocations=0`, but WAT/PASS/HBP parity fails. Direct
  manifest `mofe_hourly_carry.current_carry_total_m` and
  `upstream_carry_total_m` are both `0.0`; default reports
  `0.2205447764353141`.
- In-scope corrections: direct executor lane sequencing, typed transfer-buffer
  state, R4J dynamic transfer consumption, R3A/R4A/R4B/R4PQZ operand wiring
  required to preserve same-day water-transfer closure, tests, and H2637
  evidence.
- Protected boundaries: do not read compatibility scheduler results,
  compatibility `execution.wb13_rows`, compatibility public-output builders, or
  aggregate carry aliases as direct production authority.

Phase plan:

1. Read R7D3 handoff and relevant SC contract addenda.
2. Add focused failing/current-state tests for a two-lane direct frame where
   upstream hourly carry must reach downstream R3A/R4J before downstream R4A.
3. Implement direct dynamic transfer output publication from R4O/R4L shadows
   and downstream input mutation in the executor loop.
4. Make R4J resolve runon/carry from typed dynamic transfer buffers with
   constructor handoffs retained only for explicit non-dynamic fixtures.
5. Re-run focused tests and H2637 direct/default parity. If H2637 exposes a
   new in-envelope direct producer blocker, implement it and rerun.
6. Complete review, verification, line-count, docs, and final
   complete-or-hold disposition.

Anti-premature-stop rule:

- Do not stop after adding transfer structs, diagnostics, or one focused test.
- Do not stop when H2637 improves but WAT/PASS/HBP parity still fails and the
  next failure remains in the direct producer/transfer/publication envelope.
- A hold is allowed only for an out-of-envelope process family, missing or
  contradictory canonical authority, true sediment-coupled EROD14 handoff once
  water-transfer parity is otherwise closed, or an invalid upstream input that
  correctly fails closed. The hold must name exact residual fields and first
  code action.

Acceptance gates:

- Focused multi-lane fixture proves upstream `ui_SCrunf` and `ui_LfCrf` arrays
  reach downstream `ui_SUrunf` and `ui_LfUrf` before downstream R3A/R4J.
- Downstream R4J consumes dynamic transfer authority and publishes nonzero
  `runon_input_m`/`subsurface_carry_m` when upstream arrays are nonzero.
- Direct R3C ledger and manifest carry totals report nonzero current/upstream
  carry on a multi-OFE fixture.
- H2637 direct production still exits 0 with
  `compatibility_edge_invocations=0`.
- H2637 HBP/WAT/PASS/loss/manifest parity passes, or the package closes in a
  named hold with exact residual fields and an out-of-envelope blocker.
- Default compatibility behavior remains unchanged.
- Rust closure gates pass before `complete`: `cargo fmt --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check`, unless the package closes
  in a named hold before full R7D closure.
- Scoped Markdown lint and `git diff --check` pass.

Security-impact gate:

- No secrets, tokens, credentials, or machine-local absolute paths are committed
  as normative config.
- Direct production remains explicit opt-in and fail-closed.

Review requirements:

- Dual local reviews with explicit finding disposition.
- Verification artifact labels `Static:` and `Ran:` evidence.
- Conservation/publication anti-tautology review before any parity claim.
- `.rs` line-count governance: `2000+` lines is `WARN`; non-exempt `3000+`
  production files block closure.

Final disposition:
`HOLD-R7D4-HBP-EROD14-SEDIMENT-PRODUCER-ABSENT`.

Execution summary:

- Direct same-day MOFE dynamic carry transfer is implemented and exercised by
  focused direct-runtime fixtures.
- H2637 focused default/direct parity (`cleaned-r7d4`) now has byte-identical
  `H2637.wat.parquet` and `H2637.pass.parquet`.
- `H2637.loss.json` and `H2637.plot.parquet` differ only by `run_name`.
- Direct H2637 still exits 0 with direct runtime
  `compatibility_edge_invocations = 0`.
- Remaining HBP delta is sediment-family payload only: default HBP contains
  nonzero event sediment concentration/detachment/deposition payload where
  direct HBP publishes zero authority. This is the package-declared
  sediment-coupled EROD14/EROD15 hold boundary once water-transfer parity is
  otherwise closed.
- Follow-up package:
  `../20260623-r7d5-direct-erod14-sediment-publication-001/`.

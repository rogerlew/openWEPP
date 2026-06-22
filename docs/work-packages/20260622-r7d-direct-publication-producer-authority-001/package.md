# R7D Direct Publication Producer Authority

Status: executed-held.

Package type: Array-native runtime defect-closure implementation package.

Objective: make opt-in production direct publication parity-clean by ensuring
HBP, WAT, PASS, loss, and manifest outputs are emitted from executor-owned
typed direct publication rows, with compatibility WB13 rows retained only as
comparison evidence.

Rationale: R7C proved that `DirectProductionExecutor` runs through
`DirectFrameExecutor` with nonzero direct counters and zero compatibility-edge
invocations, but H2637 direct-production HBP/PASS/WAT checksums differed from
default compatibility. R7D must close that publication producer/parity blocker
or stop at a named hold with exact residual deltas and first follow-up action.

Included scope:

- Reproduce and classify R7C direct-production HBP/WAT/PASS deltas on the
  focused fixture and H2637 when needed.
- Fix in-envelope direct publication row, writer, identity, metadata, calendar,
  area, runoff, subsurface, storage, ET, erosion, and manifest projection
  mismatches.
- Keep direct-production output consumers reading `DirectRunPublicationFrame`
  artifacts, not `execution.wb13_rows`.
- Add focused tests proving direct-production publication parity or exact
  residual hold classification.
- Add static no-compatibility scans for production direct publication writers.
- Record independent reconstruction/anti-alias evidence for the corrected
  output families.

Excluded scope:

- Default activation; compatibility remains the default runner mode.
- R7G performance/RSS remediation.
- R7F deletion/isolation of compatibility modules beyond static proof for the
  direct-production publication consumer path.
- New science-authority changes for unresolved erosion physics. If parity
  requires new process authority outside existing direct publication producers,
  close in `HOLD` with exact blocker evidence.

Intended write set:

- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `crates/openwepp-runner/src/hillslope/04_direct_publication.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260622-r7d-direct-publication-producer-authority-001/**`

Dependencies:

- R7C production direct executor path.
- R6J direct-publication cutover parity evidence.
- Canonical publication operand ledger in
  `docs/architecture/array-native-runtime-specification.md`.

Correction authority envelope:

- Defect: `R7D-DIRECT-PRODUCTION-PUBLICATION-PARITY`.
- Observed failure: R7C same-binary H2637 direct production differed from
  compatibility for HBP, PASS, and WAT while loss and plot matched.
- In-scope corrections: direct publication row projection, direct output
  writer mappings, direct publication metadata/provenance, calendar/identity
  normalization, direct executor publication operands, and focused fixtures.
- Protected boundaries: do not use compatibility WB13 rows, runtime surfaces,
  or compatibility output builders as production direct authority. They may be
  used only for comparison, tests, and shadow evidence.

Phase plan:

1. Scaffold package, catalog entry, prompt, and evidence placeholders.
2. Reproduce focused direct-production publication deltas and classify fields.
3. Add failing/current-state R7D tests for direct-production HBP/WAT/PASS
   parity and no-compatibility publication source scans.
4. Implement in-envelope direct publication producer/writer corrections.
5. Run focused tests and, if focused parity is clean, H2637 same-binary
   default/direct publication parity evidence.
6. Complete dual local reviews, verification artifacts, line-count governance,
   and final disposition.

Acceptance gates:

- Direct-production public outputs read `DirectRunPublicationFrame` artifacts
  only. Static scans must reject production direct publication reads from
  `execution.wb13_rows`, compatibility output builders, or stale logical state
  as direct authority.
- Focused fixture HBP bytes, WAT Arrow rows/schema/metadata, PASS Arrow
  rows/schema/metadata, loss JSON, and manifest provenance are parity-clean or
  the package closes in `HOLD` with exact residual fields.
- H2637 HBP/WAT/PASS/loss/manifest parity passes for direct production or the
  package closes in `HOLD` with exact residual fields and blocker authority.
- Nonzero peak-runoff/event-duration and erosion publication authority is
  either covered by fixture evidence or explicitly held as the remaining
  erosion-authority blocker.
- Default compatibility behavior remains unchanged.
- R7C direct executor counters remain nonzero with
  `compatibility_edge_invocations=0`.
- Rust closure gates pass: `cargo fmt --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check`, unless the package closes
  in a named hold before full closure.
- Scoped Markdown lint and `git diff --check` pass.

Security-impact gate:

- No secrets, tokens, credentials, or machine-local absolute paths are committed
  as normative config.
- The mode remains explicit opt-in and fail-closed.

Review requirements:

- Dual local reviews with explicit finding disposition.
- Verification artifact labels `Static:` and `Ran:` evidence.
- `.rs` line-count governance: `2000+` lines is `WARN`; non-exempt `3000+`
  files block closure.

Final disposition:
`HOLD-R7D-MULTIOFE-DIRECT-LANE-SEED-AUTHORITY-ABSENT`.

Execution summary:

- Scaffolded the R7D package and reproduced the R7C publication parity state.
- Focused one-OFE fixture is parity-clean for HBP, WAT, PASS, loss, and
  manifest-provenance source.
- H2637 multi-OFE direct production remains non-parity for HBP, WAT, and PASS;
  loss and plot match.
- Static source review shows the direct-production consumer path already writes
  direct outputs from `DirectRunPublicationFrame`, not `execution.wb13_rows`.
- The remaining blocker is producer authority: `DirectProductionExecutor`
  constructs topology/area-only direct lane frames, then
  `DirectPublicationDayInputBuilder` seeds every lane from a single aggregate
  `HillslopeWritebackSurface` plus default direct lane state. The existing
  per-OFE static runtime surfaces are retained only inside
  `OfeLanePersistentStateSequence` for the compatibility scheduler path and are
  not converted into typed `DirectLaneConstructorInputs`.

First follow-up action: close defect
`R7D-DIRECT-PRODUCTION-PUBLICATION-PARITY` by adding lane-indexed direct
constructor seed authority from parsed per-OFE soil/slope/management/PMET,
snow, frost, initial layer, ET, transfer, and publication operands; wire that
authority into `DirectProductionExecutor`; and replace production use of the
single-surface `DirectPublicationDayInputBuilder` with typed lane-indexed
constructor/day-input producers. The follow-up must include a multi-OFE
anti-alias fixture proving lane 1 and lane 2 seed differently before H2637
parity can close.

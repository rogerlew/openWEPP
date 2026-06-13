# worker handoff

Status: M-E3 complete; M-E4 implementation next

Evidence mode: Ran + Static

## Summary

M-E3 completed the dynamic state persistence increment:

- Added `OfeLanePersistentState` and `OfeLanePersistentStateSequence`.
- Wired multi-OFE runner execution to carry OFE-local dynamic state across
  days behind the sequential OFE lane executor.
- The daily shadow lifecycle overlays current climate, seeds WB11/calendar/PL
  runtime surfaces, executes ordered OFE lanes, and replaces persistent state
  only after sequence success.
- Public WB13/WAT publication remains aggregate-only:
  `publication_ofe_policy=single-row-canonicalized-hillslope-aggregate` and
  `per_ofe_record_count=0`.
- Multi-OFE smoke manifests report
  `per_ofe_state_policy=persistent-dynamic-state-shadow` and dynamic flags
  true; single-OFE anchor manifests keep `shadow-static-slices-only` and
  dynamic flags false.
- Required H smoke passed: H1/H6/H9/H11 exited zero under
  `/tmp/openwepp_mofe01_me3_runtime_h1`.
- Local owcmp ran without comparator subagent per operator instruction:
  execution PASS for H1/H6/H9/H11; semantic FAIL remains expected at the
  aggregate-publication boundary with focus-column diffs zero.
- Single-OFE anchors H8/H15/H19/H20/H22/H23/H28 are byte-identical to M-E2
  outputs for `.hbp`, `.loss.json`, `.plot.parquet`, and `.wat.parquet`.
- Full H1-H36 was not rerun after M-E3 wiring because the new N-lane shadow
  path is debug-mode expensive. M-E3's staged runtime gate was the named
  H1/H6/H9/H11 smoke set; full-cohort replay remains M-E6/performance scope.

Next increment: M-E4 should produce authoritative internal per-OFE WB13
records from the persisted lane state. Do not flip public WAT publication in
M-E4 unless its own scope is explicitly amended; M-E5 owns the publication
policy flip.

## M-E2 summary

M-E2 completed the sequential OFE lane executor increment:

- Added `OfeLaneExecutionInput`, `OfeLaneExecutionReport`,
  `OfeLaneSequenceExecutionReport`, and `OfeLaneSequenceError`.
- Added `HillslopePhaseScheduler::execute_ofe_sequence_with_kernel`, which
  runs the existing scheduler phase graph once per ordered OFE lane.
- Added explicit transfer overlay/extraction for `TransferInput` and
  `TransferOutput`, including downstream area-ratio scaling.
- Added fail-closed guards for lane order, source/recipient identity, stale
  current output arrays, malformed transfer slots, and overflowed transfer
  totals.
- Final gates passed: fmt, focused M-E2 tests, existing writeback tests,
  M-E1 runner tests, per-OFE contract tests, clippy, full workspace tests,
  cargo-deny, authority anti-evasion script, final H1-H36 batch,
  no-publication-flip audit, and single-OFE anchor comparison.
- Local owcmp ran without comparator subagent per operator instruction:
  execution PASS; semantic FAIL remains expected at the publication boundary
  (`semantic_pass_count=0/36`, row-key failures `350720`).
- M-E2 did not wire the executor into public CLI publication, persist dynamic
  per-OFE state across days, or produce per-OFE WB13 records.

## M-E1 summary

M-E1 completed the data-model shadow-state increment:

- Added typed `TransferInput`/`TransferOutput` and
  `PerOfeDailyWaterBalanceRecord`/`PerOfeDailyWaterBalanceCollection`.
- Added static per-OFE lane slices for slope/soil/management topology.
- Constrained the legacy aggregate adapter to N=1 only; multi-OFE aggregate
  derivation remains blocked.
- Preserved aggregate WB13/WAT publication. Final manifests report
  `static_per_ofe_slice_count == contributor_ofe_count`,
  `per_ofe_record_count = 0`, and dynamic per-OFE state flags false.
- Final gates passed: fmt, clippy, focused M-E1 tests, per-OFE contract test,
  full workspace tests, cargo-deny, authority anti-evasion script, final
  H1-H36 batch, no-publication-flip audit, and single-OFE anchor comparison.
- Local owcmp ran without comparator subagent per operator instruction:
  execution PASS; semantic FAIL remains expected at the publication boundary
  (`semantic_pass_count=0/36`, row-key failures `350720`).
- Review A/B findings were accepted and fixed before final gates.

## M-E0 summary

M-E0 installed the contract/test scaffold and stopped at the required red gate:

- `SC-RUNOFFPART-001` version 43 adds `INV-RUNOFFPART-029`.
- `SC-WATBAL-001` version 155 adds `INV-WATBAL-097`.
- `SC-SYSTEM-001` version 79 adds `INV-SYSTEM-030`.
- `tests/integration/mofe01_per_ofe_state_contract.rs` is registered in
  `Cargo.toml`.
- `mofe01_me0_contract_authority_is_present` passes.
- Full `cargo test --test mofe01_per_ofe_state_contract -- --nocapture` fails
  as intended because current production code has no structural per-OFE daily
  state collection, transfer input/output payloads, or per-OFE publication
  policy manifest gate.
- No production Rust runtime implementation was edited and no runtime
  comparison was run.
- `cargo clippy --workspace --all-targets -- -D warnings` and
  `cargo deny check` pass.
- The M-E0 red workspace state has been retired by M-E1; full
  `cargo test --workspace` now passes.

## M-D summary

M-D completed the design-only architecture increment:

- `mofe-per-ofe-state-architecture.md` defines the
  `PerOfeDailyWaterBalanceCollection` target shape, per-OFE record contents,
  lifecycle, and aggregate derivation rule.
- M-D selects per-OFE lane iteration over the existing scheduler phase graph,
  not `TopologyGraph` N-node encoding, because current topology nodes are
  hillslope/channel/impoundment system nodes.
- The design maps legacy `irs`/`rochek` continuation, WATBAL per-plane rows,
  and hourly carry copy-forward obligations to explicit per-OFE transfer state.
- M-D required M-E0 to amend `SC-RUNOFFPART-001`, `SC-WATBAL-001`, and
  `SC-SYSTEM-001` and install failing contract tests before production code;
  M-E0 has since done that and left the red test in place.
- No production code, science contracts, or tests were edited in M-D.

## M-C2 summary

M-C2 executed the scoping and comparison boundary and found a hard
runtime-state blocker:

- Existing MOFE hourly carry arrays are real hour-indexed transfer state, but
  not per-OFE daily WB output state.
- Current scheduler/writeback has one aggregate state surface and one aggregate
  flux surface; no OFE-keyed daily state collection exists.
- H1-H36 still complete with 36/36 exit code `0` under
  `/tmp/openwepp_mofe01_mc2`.
- Local `owcmp` was run without the comparator subagent under explicit operator
  direction because GPT-5.3-Codex-Spark weekly quota was exhausted:
  `execution_verdict=PASS`, `semantic_verdict=FAIL`, `semantic_pass_count=0/36`,
  `structural_row_key_failures=350720`.
- Direct parquet audit shows all 29 multi-OFE surfaces still publish one
  `OFE=1` row/day, `UpStrmQ=0`, and `QOFE=Q`.
- Single-OFE anchors H8/H15/H19/H20/H22/H23/H28 stayed byte-identical to M-B.
- M-C2 dual review/verification completed. Findings on write-set separation,
  exact gate taxonomy, and docs-lint counts were accepted and fixed.
- No production code was edited; the publication implementation path remains
  blocked until real per-OFE daily state exists.

## M-C summary

M-C executed the current boundary and found a hard publication-state blocker:

- H1-H36 still complete with 36/36 exit code `0`.
- Local `owcmp` was run without the comparator subagent under explicit operator
  direction because GPT-5.3-Codex-Spark weekly quota was exhausted:
  `execution_verdict=PASS`, `semantic_verdict=FAIL`, `semantic_pass_count=0/36`,
  `structural_row_key_failures=350720`.
- Direct parquet audit shows all 29 multi-OFE surfaces still publish one
  `OFE=1` row/day, `UpStrmQ=0`, and `QOFE=Q`.
- Single-OFE anchors H8/H15/H19/H20/H22/H23/H28 stayed byte-identical to M-B.
- No production code was edited; synthesizing per-OFE WAT rows from aggregate
  state would be surrogate physics.

## M-B summary

M-B retired the multi-OFE hydrology execution blocker:

- Current openWEPP completes H1-H36 with 36/36 exit code `0`.
- All 29 multi-OFE surfaces now complete the full 2192-day run.
- Single-OFE anchors H8/H15/H19/H20/H22/H23/H28 stayed byte-identical to M-A outputs.
- Local owcmp execution passed, but semantic comparison remains failed due WAT row-key/per-OFE publication structure. That is M-C scope.
- No comparator subagent was used; comparisons were run locally per operator direction.
- Full three-identity acceptance is not proven; transfer and true per-element
  identities remain blocked until real per-OFE publication exists.

## M-A summary

M-A established the current execution boundary and the legacy routing calibration:

- Current openWEPP passes all seven 1-OFE H surfaces.
- Current openWEPP fails all 29 multi-OFE H surfaces before publication.
- Legacy H1-H36 WAT outputs preserve downstream `UpStrmQ`/`SubRIn` handoff to printed precision.
- No 15-OFE `pw0.wat.dat` exists for WAT closure.

## Local evidence left on disk

Local-only temp lane:

- `/tmp/openwepp_mofe01_mb/current_after_fix/exit-codes.tsv`
- `/tmp/openwepp_mofe01_mb/current_after_fix/logs/H*.stderr.txt`
- `/tmp/openwepp_mofe01_mb/current_after_fix/manifests/H*.json`
- `/tmp/openwepp_mofe01_mb/output/H*.{hbp,loss.json,plot.parquet,wat.parquet}`
- `/tmp/openwepp_mofe01_mb/owcmp_after_fix/summary.json`
- `/tmp/openwepp_mofe01_mb/owcmp_after_fix/reports/semantic/H*.semantic.json`
- `/tmp/openwepp_mofe01_mc/exit-codes.tsv`
- `/tmp/openwepp_mofe01_mc/output/H*.{hbp,loss.json,plot.parquet,wat.parquet}`
- `/tmp/openwepp_mofe01_mc/manifests/H*.json`
- `/tmp/openwepp_mofe01_mc/owcmp/summary.json`
- `/tmp/openwepp_mofe01_mc/owcmp/reports/semantic/H*.semantic.json`
- `/tmp/openwepp_mofe01_mc/m-c-publication-audit.json`
- `/tmp/openwepp_mofe01_mc2/exit-codes.tsv`
- `/tmp/openwepp_mofe01_mc2/output/H*.{hbp,loss.json,plot.parquet,wat.parquet}`
- `/tmp/openwepp_mofe01_mc2/manifests/H*.json`
- `/tmp/openwepp_mofe01_mc2/owcmp/summary.json`
- `/tmp/openwepp_mofe01_mc2/owcmp/reports/semantic/H*.semantic.json`
- `/tmp/openwepp_mofe01_mc2/m-c2-publication-audit.json`
- `/tmp/openwepp_mofe01_me1_final/exit-codes.tsv`
- `/tmp/openwepp_mofe01_me1_final/output/H*.{hbp,loss.json,plot.parquet,wat.parquet}`
- `/tmp/openwepp_mofe01_me1_final/manifests/H*.manifest.json`
- `/tmp/openwepp_mofe01_me1_final/owcmp/summary.json`
- `/tmp/openwepp_mofe01_me1_final/owcmp/summary.md`
- `/tmp/openwepp_mofe01_me1_final/single-ofe-anchor-cmp.tsv`
- `/tmp/openwepp_mofe01_me2_final/exit-codes.tsv`
- `/tmp/openwepp_mofe01_me2_final/output/H*.{hbp,loss.json,plot.parquet,wat.parquet}`
- `/tmp/openwepp_mofe01_me2_final/manifests/H*.manifest.json`
- `/tmp/openwepp_mofe01_me2_final/owcmp/summary.json`
- `/tmp/openwepp_mofe01_me2_final/owcmp/summary.md`
- `/tmp/openwepp_mofe01_me2_final/m-e2-publication-audit.json`
- `/tmp/openwepp_mofe01_me2_final/single-ofe-anchor-cmp.tsv`
- `/tmp/openwepp_mofe01_ma/current/exit-codes.tsv`
- `/tmp/openwepp_mofe01_ma/current/logs/H*.stderr.txt`
- `/tmp/openwepp_mofe01_ma/current/manifests/H*.json` for passing 1-OFE surfaces.
- `/tmp/openwepp_mofe01_ma/output/H*.wat.parquet` for passing 1-OFE surfaces.

These are not committed artifacts.

## Next worker focus

Execute M-E3 per the M-D breakdown. Start from the M-E2 sequential executor and
persist real OFE-local dynamic daily state without flipping WAT publication
prematurely or manufacturing records from aggregate WB13/WAT state.

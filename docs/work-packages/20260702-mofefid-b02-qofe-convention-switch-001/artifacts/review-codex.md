# Codex Review - MOFEFID-B02 QOFE Convention Switch

Review date: 2026-07-02
Reviewed head: `0110de00` on `worktree-mofefid-b02-qofe`
Scope: review only. No production code, tests, or contracts were modified.

## Evidence Classes

Static:

- Read `package.md`.
- Read the B02 diff (`HEAD^..HEAD`) in `01_publication.rs`, the migrated
  direct-runtime test, and `SC-RUNOFFPART-001.md`.
- Read the existing `QOFE` authority in `SC-WATBAL-001.md`,
  `SC-SYSTEM-001.md`, `docs/planning/mofe-fidelity-campaign-strategy.md`,
  and `/workdir/wepp-forest/docs/20260504-stakeholder-watbalance.md`.
- Searched runtime, runner, and tests for `QOFE`, `qofe_mm`, `runvol`, and
  retained direct-publication frame consumers.

Ran:

- `cargo test -p openwepp-hillslope-orchestrator r7d4_publication_qofe_equals_q_with_independent_runvol_basis -- --nocapture`
- `cargo test -p openwepp-runner --test totalwatsed3_cli_contract totalwatsed3_cli_uses_pass_runvol_and_outlet_lateral_flow -- --nocapture`
- `cargo test -p openwepp-runner r6a_direct_projection_consumers_read_publication_frame_operands -- --nocapture`
- `cargo test -p openwepp-hillslope-orchestrator -p openwepp-runner`
- `cargo fmt --check`
- `/home/workdir/openWEPP/.venv/bin/python` read-only scratch H2637 and
  single-OFE parquet/hash checks under
  `/tmp/claude-1000/-home-workdir-openWEPP/e46d9841-ba57-46c6-9ae7-061c6c19110b/scratchpad/`

Not run:

- I did not regenerate H2637 from source inputs or rebuild/replay `HEAD^`;
  the package does not include a committed gate recipe or baseline output
  manifest. I used existing scratch outputs only as independent context.
- I did not rerun clippy, deny, or nextest; the touched-crate `cargo test`
  suite and `fmt` passed.

## Findings

### B02-CX-001 - Accepted Candidate - Canonical contracts still hard-fail the new `QOFE == Q` convention

Evidence class: Static.

`SC-RUNOFFPART-001.md:158` adds `INV-RUNOFFPART-032`, and the WB13 coupling
table now requires `QOFE == Q` on all rows at
`SC-RUNOFFPART-001.md:371-380`. That is the B02 target and matches the
operator decision.

But two other active canonical contracts still encode the opposite rule:

- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md:286`
  says public `QOFE` is `runoff * efflen / slplen`, public `Q` is
  `runoff * efflen / totlen`, and downstream `QOFE == Q` where
  `slplen != totlen` hard-fails.
- `SC-WATBAL-001.md:494` repeats that hard-fail posture in the guard map.
- `SC-WATBAL-001.md:2187-2190` keeps the old M-F-REDO2 acceptance text.
- `SC-WATBAL-001.md:2459` records the old convention as the latest changelog
  entry for that invariant.
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md:148`,
  `SC-SYSTEM-001.md:185`, `SC-SYSTEM-001.md:841-844`, and
  `SC-SYSTEM-001.md:990` repeat the same old public-normalization
  requirement and hard-fail downstream `QOFE == Q` aliases.

This leaves openWEPP's authority set internally contradictory: one contract
requires B02's behavior while two active publication/consumer contracts reject
it. The implementation can be correct and still fail governance closure.

Disposition recommendation: amend or explicitly supersede the WATBAL/SYSTEM
M-F-REDO2 text so public `QOFE == Q` is accepted under `INV-RUNOFFPART-032`.
Keep the anti-clone requirement by binding it to lane-local raw runoff,
transfer lineage, and the retained local-length `runvol`/peak basis rather
than the old public `QOFE` local normalization.

### B02-CX-002 - Accepted Candidate - Gate evidence is not package-local, and one H2637 headline count does not reproduce

Evidence class: Static + Ran.

`package.md:3-7` and `package.md:55-69` record strong `Ran` claims:
single-OFE byte identity, H2637 `H.pass.parquet` byte invariance, all-row
`QOFE==Q`, 53,298 multi-OFE `QOFE` values changed, unchanged control columns,
and green suites. The package directory contains only `package.md`; there is no
committed gate log, command transcript, output hash table, run-name/baseline
manifest, or analysis script.

I found scratch outputs and checked them read-only. Against the comparable
pre-B02 scratch run `dc01-m3/out`:

- `H2637.pass.parquet` hash is byte-identical:
  `2c21e969144b1eab20c245090e3ac3e8705666191d1eebc0d14e2e4110a10dfe`.
- B02 `H2637.wat.parquet` has `QOFE == Q` on every row.
- `Q`, `latqcc`, `Total-Soil`, `Ep`, `Dp`, and `Snow-Water` are unchanged.
- Single-OFE scratch old/new hashes match for HBP, loss, plot, WAT, and PASS.
- The changed multi-OFE `QOFE` row count is `87,791`, not the package's
  `53,298`, while the same comparison shows all listed control columns
  unchanged.

This is not evidence that the semantic change is wrong. It is evidence that
the package evidence is not durable and at least one published scalar is stale,
baseline-dependent, or incorrect.

Disposition recommendation: add `artifacts/gate-log.md` or equivalent with the
exact commands, baseline/after paths, run names, hashes, and row-count query.
Correct the changed-row count or document the baseline/filter that produces
`53,298`.

### B02-CX-003 - Accepted Candidate - The `QOFE == Q` rule is producer-computed but not guarded at the publication boundary

Evidence class: Static + Ran.

The changed producer is directionally correct: `direct_publication_runoff_operands`
computes public `Q` from cumulative length at
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs:363-367`,
assigns `qofe_publication_mm = q_publication_mm` at `01_publication.rs:384`,
and keeps `runvol_m3`/peak on the local-length basis at
`01_publication.rs:377-386`.

The publication boundary does not enforce the new contract. The streaming sink
calls `require_direct_publication_output_family_authority_row(row)` before
writing WAT rows at
`crates/openwepp-runner/src/hillslope/04_direct_publication.rs:74-86`, but that
guard checks area, precipitation, runvol, and erosion fields only at
`crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs:488-539`.
It does not check `Q`, `QOFE`, or their equality. The WAT builder then maps the
two fields independently at
`crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs:679`
and `02_output_and_climate_helpers.rs:693`.

The retained-frame validation path is also count/identity only:
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs:209-223`
and `crates/openwepp-runner/src/hillslope/04_direct_publication.rs:787-815`.
The existing runner projection test still constructs `q_mm = 12.5` and
`qofe_mm = 10.0`, then asserts those values pass through to WAT/PASS at
`crates/openwepp-runner/src/hillslope/03_tests.rs:922-925` and
`03_tests.rs:960-962`; I reran that test and it passed.

Because production rows currently originate from the updated producer, this is
not a demonstrated H2637 output defect. It is a guard/test gap against the
package's "all WB13 rows" claim and `INV-RUNOFFPART-032` hard-fail posture.

Disposition recommendation: add a publication-boundary guard/test that rejects
or at least fails cutover when a WAT-bound direct publication row has
`runoff.q_mm.to_bits() != runoff.qofe_mm.to_bits()` under B02. If retained
frames are intentionally allowed to carry legacy values for test-only
projection checks, narrow that explicitly and update the stale fixture so it
does not read as an accepted B02 publication state.

## Accepted Checks

- I accept the convention decision itself. The campaign strategy identifies
  B7 as a convention/schema decision (`docs/planning/mofe-fidelity-campaign-strategy.md:199`
  and `:215-220`), and the wepp-forest brief says `wepp_260516` changed both
  QOFE write paths so `QOFE` equals `Q` on every row while `H.pass.runvol`
  remains unchanged (`/workdir/wepp-forest/docs/20260504-stakeholder-watbalance.md:243-249`).
- I accept the implementation shape in the direct producer: public `QOFE` now
  aliases public `Q`, while `runvol` and the near-zero peak decision use the
  retained local-length basis.
- The migrated producer test passed:
  `r7d4_publication_qofe_equals_q_with_independent_runvol_basis`.
- The downstream `totalwatsed3` consumer test passed and continues to consume
  PASS `runvol` for canonical runoff volume.
- Full `cargo test -p openwepp-hillslope-orchestrator -p openwepp-runner`
  passed: 148 orchestrator tests, 67 runner lib tests, 2 runner-bin tests, and
  runner integration tests including the 24 watershed CLI tests.
- `cargo fmt --check` passed.

## Review Outcome

Hold as written. The production diff appears to implement the intended B02
semantics, and the scratch outputs support the central invariance claims. But
the package is not review-clean until the contradictory WATBAL/SYSTEM contract
authority is reconciled, package-local gate evidence is preserved and corrected,
and the B02 `QOFE == Q` rule is guarded at the publication boundary or explicitly
scoped to producer-generated rows.

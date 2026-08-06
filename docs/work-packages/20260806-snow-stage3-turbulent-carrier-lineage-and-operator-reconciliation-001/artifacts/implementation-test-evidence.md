# Implementation And Test Evidence

Status: `implementation and admitted v3 execution PASS`.

Evidence mode: `Ran`.

Working directory: `/home/workdir/openWEPP`.

Inactive-day lifecycle amendment:

- `cargo nextest run -p openwepp-hillslope-orchestrator stage3 --no-fail-fast`
  — PASS (`8/8`).
- `cargo nextest run -p openwepp-runner stage3 --no-fail-fast` — PASS
  (`10/10`).
- `.venv/bin/python -m pytest -q docs/work-packages/20260806-snow-stage3-turbulent-carrier-lineage-and-operator-reconciliation-001/tools/test_run_operator_reconciliation.py`
  — PASS (`50/50`).
- `cargo nextest run --test snow_stage3_turbulent_operator_reconciliation_contract --test snow_stage3_shadow_observability_contract --no-fail-fast`
  — PASS (`13/13`).
- `.venv/bin/python tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  — PASS (`11/11`).
- `cargo clippy -p openwepp-meteorology -p openwepp-hillslope-orchestrator -p openwepp-runner --all-targets -- -D warnings`
  — PASS.
- `cargo fmt --all` and `git diff --check` — PASS.

The Rust consumer test proves the authoritative inactive partition is exactly
equal with evaluation disabled or enabled, the disabled record remains schema
v4 with no companion, and the enabled record carries schema v6, zero sentinel
identities, zero evaluated support, 24 requested hourly intervals, no tuples,
and 24 typed `operator_not_selected` statuses. The Python test independently
proves that this declared empty-support row is accepted by the exact analyzer;
rejects nonzero, mixed, missing, and tuple-bearing sentinel aliases, wrong
hourly reasons, and contradictory support fields; and proves through the real
site reduction that a paired/sequential inactive day changes only the typed
non-evaluated and zero-support inventories, never `S/F/Q`, support, unmatched,
partial, or projection estimands. A source-order seam test binds the inactive
return before typed hourly-forcing acquisition and rejects restoration of the
operator-disabled-only guard that caused the rejected v1 execution.

The corrected v2 cohort was admitted at exact clean
`e591d89c219d69f619e68f9aa7194f88d20f9a1c` after independent result-blind
science/Rust/consumer `PASS/PASS/PASS`. It completed all 12 lanes but failed
closed before results on exact sequential after-to-next-before layer-state
continuity. See `rejected-execution-v2.md`; no v2 metric or decision was
admitted.

Sequential transition-boundary correction:

- deterministic alignment, temperature normalization, and fragment
  coalescence now occur before the prior tuple's after-state is captured and
  are carried without repetition into the next substep;
- the Rust reconciliation validator independently requires exact layer counts,
  fingerprints, active/total mass and cold content, geometry, density, and
  surface-temperature continuity between every sequential tuple;
- a positive real-solver vector proves exact continuity and a fingerprint-bit
  mutation fails closed; a second real runner/JSON vector forces substeps 0 and
  1 within the same hour and proves every serialized after/before count,
  fingerprint, mass, cold, depth, density, and surface-temperature value is
  bit-exact after JSON round-trip;
- `cargo nextest run -p openwepp-hillslope-orchestrator stage3 --no-fail-fast`
  — PASS (`9/9`);
- `cargo nextest run -p openwepp-runner stage3 --no-fail-fast` — PASS
  (`11/11`);
- contract/observability — PASS (`13/13`);
- package consumer — PASS (`50/50`);
- affected-crate warnings-denied Clippy, formatting, and diff hygiene — PASS.

Exact committed source identity is
`5ebfc5135b80d250cb6b38d1b6241a7d2a72d6c5`. Independent result-blind science,
Rust, and consumer reviewers returned `PASS/PASS/PASS` before v3 execution.

The admitted v3 release execution completed all 12 lanes in `2516.26 s` and
wrote `143` retained artifacts. Independent `--verify-existing` completed in
`2176.59 s` and revalidated exact source, binary, input, receipt, inventory,
and result custody. Snowbird observed runner timing was `4.73 s` control,
`28.89 s` paired, and `118.46 s` sequential; no prospective performance
threshold was frozen, so these ratios are observational only.

Post-result classifier correction: a science reviewer found that the generic
legacy internal-conduction sign class was incorrectly gated on predecessor
reproduction even though the frozen protocol permits multilabel coexistence.
The retained result is unchanged because its external `Q_all` is positive.
After removing that unrelated gate and adding the missing coexistence vector,
the package consumer passes `51/51` and the focused contract/observability
selection passes `13/13`.

The first workspace-Clippy closure attempt found only test structure debt:
`v129_canonical_addendum_pins_exact_algorithm_units_and_failures` was 115 lines
against the 100-line lint. The exact assertion groups were mechanically split
into `assert_canonical_addendum` and `assert_canonical_tables`. No assertion,
contract text, runtime source, or result changed. Workspace all-target Clippy
with warnings denied now passes, as do the focused `13/13` and consumer
`51/51` selections.

The canonical assurance export guard predated the governed tracked
`review-drafts/**` lane and therefore failed every current checkout by treating
review inputs as approved public output. After prospective write-set amendment,
the guard excludes exactly `usersum/assurance/review-drafts/*` from its public
inventory. It still rejects any other extra public file and still requires the
sole `usersum/assurance/README.md`, zero export documents, disabled vendoring,
typed validation/check, and release-transition preflight. `bash -n`, direct
guard execution, and `render_assurance_review_drafts.py --check` pass.

The first claimed draft check was invalidated by independent review: its run
had not completed when the parent observed the preceding command output. The
reviewer correctly reproduced seven stale generated files. After prospective
admission, the canonical renderer applied those updates and a separately
completed `--check` returned `review rendering: PASS (98 files current)`.

# Implementation And Test Evidence

Status: `implementation loop PASS; result-blind re-review pending`.

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
  mutation fails closed;
- `cargo nextest run -p openwepp-hillslope-orchestrator stage3 --no-fail-fast`
  — PASS (`9/9`);
- `cargo nextest run -p openwepp-runner stage3 --no-fail-fast` — PASS
  (`10/10`);
- contract/observability — PASS (`13/13`);
- package consumer — PASS (`50/50`);
- affected-crate warnings-denied Clippy, formatting, and diff hygiene — PASS.

Exact committed source identity and renewed result-blind verdict will be
appended after the v3 implementation checkpoint. No v3 cohort has run.

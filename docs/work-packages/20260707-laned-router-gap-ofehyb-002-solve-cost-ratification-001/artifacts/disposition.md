# Disposition

Status: EXECUTED-COMPLETE-NO-PROMOTION. Evidence mode: Static + Ran.

## Outcome

`GAP-OFEHYB-002` is resolved for the current H2637 source-memory hybrid
solve-cost bottleneck:

- The exact bare-skin branch evaluator is contract-backed in
  `SC-OFEROUTE-002` rev 4.
- H2637 active hybrid equilibrium map evaluations moved from `151435969` to
  `0`.
- H2637 user time moved from `38.39 s` to `33.37 s`.
- The selector remains experimental/unpromoted; no default/D16 activation is
  made here.

## Review Disposition

### `review-codex-code.md`

High: direct bare-skin path can mask invalid inactive operands.

- Disposition: ACCEPTED and fixed.
- Fix: `CellParameters::validate()` is now visible to the implicit stepper and
  called before each implicit cell solve; invalid raw operands fail closed
  before any direct evaluation.
- Regression: `implicit_step_rejects_invalid_inactive_raw_operands_before_direct_path`.

High: package evidence insufficient for closure.

- Disposition: ACCEPTED and fixed in artifacts.
- Fix: timing, fidelity, ratification, implementation, gate, and final
  disposition artifacts now record the executed evidence.

### `review-codex-qa.md`

High: H2637 output deltas were real but undispositioned.

- Disposition: ACCEPTED and fixed.
- Fix: `SC-OFEROUTE-002` and package timing/fidelity artifacts now state that
  active hybrid output is not byte-identical and ratify the observed sparse
  numeric dust under the branch-equilibrium tolerance surface.

High: required gates/disposition evidence remained queued.

- Disposition: ACCEPTED and fixed.
- Fix: package artifacts are updated from QUEUED/NOT RUN to executed evidence;
  final gate rerun evidence is recorded in `gate-results.md`.

Medium: edge tests too narrow.

- Disposition: ACCEPTED and fixed.
- Fix: added composed `solve_cell` edge tests for nonzero rain term, zero
  `k_o`, and near-crossover bare-skin cases.

Medium: effective-zero wording ambiguous.

- Disposition: ACCEPTED and fixed.
- Fix: `SC-OFEROUTE-002` now says component absence is exact-zero based, not a
  near-zero threshold, and raw parameters still undergo validation.

Low: line-count governance missing.

- Disposition: ACCEPTED and recorded.
- Fix: line-count governance is recorded in `implementation.md` and
  `gate-results.md`.

## Residual Risk

The direct evaluator is ratified for bare skin-only cells. Generic non-bare
form/wave/vegetation direct solve optimization remains optional follow-on work
and must carry its own branch/equivalence proof if pursued.

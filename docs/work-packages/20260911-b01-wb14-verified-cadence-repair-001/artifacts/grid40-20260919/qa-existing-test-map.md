# Grid40 QA: frozen existing-test map

Evidence class: **Static.** Read-only source extraction from
`/home/roger/openwepp-experiments/b01-wb14-lse-first-trial-source-20260918`.
No mutable experiment source was reviewed, and no model, evaluator, or test run
was performed.

## Existing covered-solver controls

| Required experiment invariant | Frozen source test | Actual checked behavior |
| --- | --- | --- |
| Exact-bound outward direction | `crates/openwepp-land-surface-energy/src/solver_covered_solve.rs::covered_finite_difference_stencil_tests::exact_phase_bound_outward_newton_direction_has_no_admitted_backtracking_factor` | Constructs an admitted covered trial and asserts `covered_trial_is_valid` is false for every outward factor `b=0..20` at the exact phase bound. |
| Nonfinite residual refusal | `solver_covered_solve.rs::covered_halved_no_update_witness_tests::complete_residual_passing_full_and_later_domain_valid_poisons_refuse_the_witness` | Rejects a residual above one, `Inf`, and `NaN` for the no-update witness. |
| Incomplete first valid evaluation | `solver_covered_solve.rs::covered_halved_no_update_witness_tests::enclosing_preflight_does_not_skip_incomplete_or_failed_first_domain_valid_trial` | An `EvaluationIncomplete` or failed first domain-valid halved trial returns no witness and examines only `b1`. |
| Initial nonfinite input | `crates/openwepp-land-surface-energy/src/solver_tests.rs` (Stage-3 anchor control around the `invalid_initial_trial` assertion) | Sends `NaN` through `solve_covered_column` and asserts `ConstitutiveDomain("covered_initial_trial")`. |
| Strict-decrease failure despite sub-one residual | `solver_tests.rs::normalized_solver_rejects_stagnation_below_one` and `normalized_solver_rejects_increase_remaining_below_one` | Asserts `BacktrackingLimit` when the normalized residual remains `0.5` or increases to `0.75`; a sub-one norm alone is not acceptance. |
| First valid no-update witness selection | `solver_covered_solve.rs::covered_halved_no_update_witness_tests::enclosing_preflight_skips_domain_invalid_trials_and_returns_first_complete_witness` | After domain-invalid `b1` and `b2`, selects the first complete valid witness at `b3`; the related `each_full_witness_refusal_and_first_domain_valid_halving_admit_no_update` covers both full-witness refusal categories. |

## Explicit gaps

- No genuine covered-solver test for duplicate rounded strict-search trials was found.
- No direct covered-solver test injects an evaluator `Err` during strict search and asserts the typed terminal outcome and retained trace.
- Existing no-update controls are helper-level; they do not directly prove the full solver retains the current state and never installs the prospective trial.
- No covered-solver fixture joins a tiny governed step with an unacceptable complete residual.

## Navigation record

Navigation/tool failures: **0**. Broad `rg` output was truncated by the tool
display, but all targeted frozen-source reads completed successfully.

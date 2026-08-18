# Potential Candidate Evaluated as Fixed Final

Status: `OUTCOME A / physical fixed-final closure / numerical tie failure`

Evidence class: direct diagnostic evaluation of the complete committed WIP
equations at `aa8f55d93d58df6c62b5ae4eebb78245b5469fd6`. This is classification
evidence, not accepted authority and not an oracle expected result.

The diagnostic changed one thing in an isolated detached worktree: the final
solver's initial vector was the already accepted potential candidate rather
than the configured final warm start. The final evaluator still rebuilt from
`phase.beginning`, used the exact real-owner caps, froze no potential flux, and
reevaluated all 29 fixed-final residuals.

Result:

- all 29 residuals evaluated successfully;
- maximum absolute normalized residual was
  `1.4944954552325508e-5`, far below the acceptance bound `1`;
- root-source continuity residuals were below `1.6e-21 kg m^-2 s^-1`;
- component/shared/ground/soil energy residuals were below
  `4.0e-12 W m^-2`;
- the solver nevertheless rejected at iteration zero with pivot
  `2.2158381900172497e-18` and matrix infinity norm
  `2.0244335994229328`.

The rejection occurs because the historical solver requires a prior accepted
step even when the initial residual is already closed. It therefore forms a
Jacobian at a point where all eight root caps are equality-active and the
linear solve rejects as singular. This localizes the failure to that numerical
path; rank/null-space or counterfactual evidence is still required before
claiming that equality ties alone cause the rank loss.

This establishes a finite full-supply H0 fixed-final candidate satisfying all
existing residual tolerances under the complete repository equations. It
disproves the premise that new hydraulic attenuation physics is required
merely for residual closure of the canonical full-supply midnight case.
H1 remains only a separately reviewable candidate for required partial-supply
domains.

The exact dimensional/tolerance/f64-bit ledger is
`potential-as-final-evaluation.json`. The preserved raw command log is
`potential-as-final-raw.log`, SHA-256
`d162113a3a1f5d96d5c266b812dd1101abbc0be7b46f0f6334e568e519f0beed`.

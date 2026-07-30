# Review Agent A

Evidence class: `Static + Ran`.

Final verdict after correction re-review: `PASS`.

The central scientific diagnosis is sound: the current snow runtime does not
compose a complete surface-energy balance, Stage A/B sublimation is a separate
mass-side candidate path, canopy emission cannot simply be added over the full
area without displacing the obscured sky fraction, and latent energy and vapor
mass must represent one transfer. The asymmetric successor posture is also
defensible: EB-03 may proceed only as contract-first reconciliation, while
EB-02 and the full factorial remain held for load-bearing longwave authority.

## Correction Re-review

Ran:
`.venv/bin/python docs/work-packages/20260729-snow-surface-eb-01-reconciliation-factorial-design-001/tools/validate.py`
returned `PASS: 14 CSVs, 3 SVGs, sidecars, links, determinism`.
`git diff --check` also passed.

| Original finding | Resolution | Re-review evidence |
| --- | --- | --- |
| A-01 observation reconciliation/counts | `RESOLVED` | `observation-fixture-ledger.csv` now carries source/custody, location, per-stratum period, resolution, units, forcing uncertainty, modeled fixture/binding, operator, and separately named total/depth/SWE/density counts. The generator derives these counts from normalized files; Harvard totals are correctly `821` for all three strata. |
| A-02 readiness schema | `RESOLVED` | `calibration-readiness-matrix.md` reports the three canonical orthogonal statuses and all ten `science-contract-spec.md` readiness obligations with allowed gate values, evidence paths, and rationale. Prospective `NOT_CALIBRATION_READY` mechanisms are not misrepresented as a current-scope EB-01 block. |
| A-03 control-volume closure | `RESOLVED` | `mass-energy-operand-lineage.csv` now fixes control volume, area/time bases, source lineage, before/after total ice-plus-liquid storage, exact duration, thermal storage, and phase mass. `factorial-design.md` uses only ledger-named operands, explicitly treats melt/refreeze as internal mass transfers, and reconstructs signed latent-energy/vapor-mass equivalence independently. |
| A-04 operational decision rules | `RESOLVED` | `decision-rules.csv` and `stop-loss.csv` provide hard-gate operators, outcomes, protected-lane posture, claim limits, and explicit owners. Values that require successor authority—closure tolerances, snow-free threshold/window, runoff window, and tie rule—truthfully produce `HOLD_EB04` and must be frozen before results. EB-01 does not invent them or claim EB-04 is executable. |
| A-05 hard-coded evidence | `RESOLVED` | `tools/generate.py` reads candidate scores and sublimation magnitudes from the retained Stage B JSON, computes observation counts/periods from normalized CSVs, verifies manifest identities and units, and hashes all normalized source files in `dependency-manifest.csv`. The figures consume those reconstructed structures. |

`finding-disposition.md` accepts and corrects all five findings without
deferral. The corrected design meets the package's science-review exit
criteria: the observation roles are prospectively frozen, the factorial
estimands are orthogonal, authority and data holds are separated, physical
closure precedes performance interpretation, and unresolved result thresholds
are named pre-execution prerequisites rather than passed gates.

No new closure-blocking science, observation-role, identifiability, factorial,
or stop-loss finding was identified.

## Original Findings (Pre-correction Record)

### A-01 — High — Observation reconciliation is incomplete and contains a row-count error

`observation-fixture-ledger.csv` does not record several fields required by the
package's included scope: custody/source identity, location, period, temporal
resolution, observation units, forcing source/uncertainty, exact modeled
stratum, and comparison operator. Those omissions prevent a reviewer from
establishing whether a nominally bound lane is scale- and time-compatible with
the pre-registered response.

The Harvard `total_rows` values are also incorrect or mislabeled. The ledger
reports `448`, `449`, and `390`; retained exact-binding evidence in
`20260626-snowdensity-10-3-3-gradient-melt-adjudication-001/artifacts/gradient_melt_adjudication.md`
reports `821` rows for each Harvard stratum, consistent with the installed
HF237 organization. The reported values appear to be non-null depth counts,
not total rows.

Action: expand the observation ledger with the package-required provenance,
time/space/forcing/operator fields; derive or cite every count; correct Harvard
totals to `821` or rename the field to the exact quantity it represents. Keep
Harvard hemlock diagnostic-only and HJ Andrews missing/unbound.

### A-02 — High — The calibration-readiness matrix does not meet binding work-package governance

`calibration-readiness-matrix.md` uses a single free-form disposition column
(`READY_FOR_CONTRACT_WORK`, `IMPLEMENTATION_PREREQUISITE`, and similar) rather
than the three required orthogonal fields:
`science_implementation_status`, `calibration_evidence_status`, and
`identifiability_status`. It also does not disposition every applicable
science-contract-spec obligation as `PASS`, `BLOCKED`, or `NOT_APPLICABLE`
with an evidence path and rationale.

This directly conflicts with `docs/work-packages/AGENTS.md`, section “Science
Implementation And Calibration Readiness,” and leaves package exit criterion 9
unmet.

Action: rebuild the matrix with the three canonical status vocabularies and an
obligation-level gate column. Distinguish the science-authority hold for
longwave from the data-limited warm-maritime transfer claim. A current-scope
`BLOCKED` entry must be reconciled with the package disposition under the
non-deferral rule.

### A-03 — High — The future mass/energy closure is not independently reconstructable as written

`mass-energy-operand-lineage.csv` omits the required normalization/denominator,
area basis, time basis, exact source authority/path, and explicit beginning and
ending storage operands. In `factorial-design.md`, `retained_rain_change` is
used in the mass equation even though that operand does not exist in the
lineage. The equation does not unambiguously distinguish total snow water
storage from ice SWE and retained-liquid storage, so rain retained within the
pack can be counted as an input or storage change without a defined convention.
The energy side likewise needs explicit step duration and beginning/ending
cold-content definitions rather than only a prose conversion instruction.

This is not merely a presentation issue: a future producer and verifier could
implement different, individually plausible storage conventions and both claim
closure. It leaves exit criteria 6 and 7 unmet.

Action: define the control volume and storage state explicitly; add
`storage_before`, `storage_after`, retained-liquid change (if separate), exact
duration, area/time normalization, and producer/source lineage. Write the
independent equations only from ledger-named operands and require the verifier
to reconstruct latent-energy/vapor-mass equivalence from raw operands rather
than a producer residual.

### A-04 — High — Promotion and stop-loss language is not operationally falsifiable

`stop-loss.md` invokes “improves,” “targeted direct operands,” and “material
protected-lane regression” without pre-registering thresholds, aggregation,
protected lanes, or tie handling. `response-operator-ledger.csv` also leaves
key operators underdefined: “first persistent snow-free day” has no persistence
window; runoff centroid/peak lacks an evaluation window; and the observation
matching/uncertainty treatment is not specified. Consequently, two analysts
could reach different promote/reject decisions from the same EB-04 outputs.

Action: add a machine-readable decision table defining trace/closure hard
gates, primary responses, protected lanes, directionality, aggregation,
uncertainty/tie policy, persistence window, and the exact outcome for improve,
neutral, tradeoff, and regression cases. If numerical thresholds cannot be
admitted yet, assign their authority and freeze them before any result-bearing
run; do not call the present language a binding one-round adjudication rule.

### A-05 — Moderate — Several claimed generated facts are hard-coded rather than reconstructed

`tools/generate.py` hard-codes the candidate scores/magnitudes and observation
counts. It asserts only `stage_b_robust_fail_count == 15` from the retained
Stage B JSON, while the prior-candidate sidecar says the plotted values “are
read from” that JSON. The dependency manifest hashes fixture manifests but not
the normalized observation files from which counts would need to be derived.

Action: parse all plotted prior-candidate values from their retained evidence
or cite separate exact evidence for values absent from the JSON; compute
observation counts from the normalized sources; hash those source files; and
make the generator fail if expected identities, periods, units, or bindings
change.

## Positive Evidence

- `SC-SNOWFREEZE-001` invariants 073 and 076 support the nonpromotion posture
  and the requirement that sublimated vapor never enter liquid routing.
- `factorial-design.md` uses the correct two-factor interaction
  `Y(LS) - Y(L) - Y(S) + Y(B)` and requires independent longwave and
  sublimation selectors.
- `rejected-formulas.md` correctly rejects full-area additive canopy emission,
  net/incoming-longwave aliasing, latent double debit, vapor-to-liquid routing,
  and depth-as-SWE inference.
- Observation roles are prospectively separated; no lane is simultaneously
  assigned to calibration and independent validation.
- The source-acquisition list is bounded and appropriately distinguishes the
  HJ Andrews transfer-data gap from the longwave science-authority gap.

At the original review boundary, A-01 through A-04 required correction and
re-review, and A-05 was actionable because the figures and ledgers made
stronger provenance claims than the generator then proved. The correction
re-review above supersedes that original disposition.

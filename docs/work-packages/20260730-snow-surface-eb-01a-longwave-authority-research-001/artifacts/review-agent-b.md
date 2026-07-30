# Review B

Disposition: `PASS` for the review stage.

Evidence class: Ran + Static + primary-source verification.

Review iteration: final corrected derived-sky-view amendment tree.

Reviewer scope: disposition of `DSV-RB-01` through `DSV-RB-05`, plus exact
package/roadmap/catalog/ledger/figure/governance consistency.

## Evidence

Static inspection covered the corrected package, roadmaps, catalog, equation
and operand ledgers, source records, figure/sidecar, finding disposition, gate
evidence, final disposition, and both reviews. The existing terminal
verification artifacts were inspected as prior-tree evidence only; they must
be replaced by the planned amendment-aware terminal verification before
package closure.

Ran:

- amended CSV parse/shape:
  source authority `10 x 6`, equation `13 x 7`, acquisition `4 x 5`, operand
  readiness `15 x 6`, including headers;
- `wc -l package.md`: `197`; and
- `git diff --check`: `PASS`.

The official Essery et al. (2025) version-of-record was rechecked: Equation 14
gives `tau_d = exp(-1.6 k_ext VAI_eff)`, the default `k_ext = 0.5` applies to
randomly oriented canopy elements, and section 2.3 reuses diffuse transmission
for atmospheric longwave.

## Finding Disposition

### DSV-RB-01 — RESOLVED FOR REVIEW STAGE / TERMINAL VERIFICATION REQUIRED

Review A and Review B now independently pass the amended formulation,
structural-cover lineage, product constraint, roadmap hold, and stop-loss.
This resolves the review portion of `DSV-RB-01`.

The existing verification, gate, and final-disposition artifacts describe an
earlier tree and are not accepted as exact-tree closure evidence. Their
replacement is deliberately the next lifecycle phase after both reviews pass,
so their staleness is not a review-stage failure. Before closure, two
independent amendment-aware terminal verifications must inspect the final tree
and replace those stale artifacts.

### DSV-RB-02 — RESOLVED — Mapping-base equation ledger

`equation-ledger.csv` records FSM2 Equation 14, dimensionless units,
`k_ext = 0.5`, the whole-canopy candidate
`f_sky = exp(-0.8 VAI_eff)`, random-orientation/homogeneous-canopy scope, and
the explicit EB-02 hold on the openWEPP leaf-plus-stem `VAI_eff` composition.

### DSV-RB-03 — RESOLVED — Structural-cover runtime lineage

`operand-readiness-ledger.csv` records that structural cover is static
per-crop native-forest runner authority, already contributes the dynamic-cover
floor, and is not a distinct downstream daily-growth field. It requires
explicit EB-02 consumer binding and guards against double counting.

### DSV-RB-04 — RESOLVED — Roadmap hold and stop-loss

The top-level roadmap retains runtime `HOLD` until the sky-view, cloud, and
snow-temperature providers are contract-bound. The package stop-loss
prohibits an invented or site-fitted blend, a new user-entered sky-view
coefficient, or required remote data if an authority-backed existing-state
mapping cannot be formed.

### DSV-RB-05 — RESOLVED — Governing authority and included scope

`package.md` now names Essery et al. (2025) in governing science authority and
includes structural cover among the existing-state inputs in scope.

## Product, Runtime, And Figure Consistency

These surfaces pass:

- no new user sky-view coefficient is required;
- no hemispherical photograph, LiDAR product, canopy-height model, or other
  remote dataset is a runtime prerequisite;
- optional observations retain independent-validation and uncertainty roles;
- canopy cover and LAI are not relabeled as radiometric sky view;
- the exact `VAI_eff` composition remains an EB-02 canonical-contract
  prerequisite;
- runtime implementation remains `HOLD` for the derived sky-view operator,
  contract-matched cloud mapping, and active snow-temperature provider; and
- the readiness figure marks sky view amber, says “derive from canopy state,”
  and the sidecar explains the mapping and retained holds.

The generated figure remains consistent with the corrected operand posture.
No additional article or remote-data acquisition is needed for this review.

## Mandatory Terminal Condition

This `PASS` is not terminal package closure. Before closure:

1. two amendment-aware terminal verifications must replace the stale
   verification artifacts and evaluate the exact final tree;
2. both verifiers must check current artifact dimensions and package line
   count, the FSM2 mapping, structural-cover lineage and double-count guard,
   roadmap/catalog runtime hold, figure/sidecar consistency, and the
   prohibitions on a new user coefficient and required remote data; and
3. gate evidence, finding disposition, final disposition, package status, and
   roadmap/catalog closure claims must be reconciled only after both
   verifications pass.

If either terminal verifier finds a defect, the package must reopen and the
affected closure records must be corrected before completion.

## Final Verdict

`PASS` for the review stage. No substantive Review B finding remains. Exact-tree
closure is contingent on the planned dual amendment-aware terminal
verification and subsequent reconciliation of all downstream lifecycle
records.

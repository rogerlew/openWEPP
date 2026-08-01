# Independent Science/Governance Review B

Evidence class: `Static + independently reconstructed Reused Ran`.

Verdict: `GO_WITH_AMENDMENTS`.

## Scope Reviewed

Independently reviewed the package contract, frozen EB-04R protocol, committed
EB-04S retained adjudication, both cited snow contracts, prior post-partition
residual decomposition, package-local analysis tool, JSON/CSV outputs, three
SVG figures and sidecars, scientific attribution and criterion-fitness claims,
roadmap/catalog updates, declared gates, and protected diff. Reviewer A's
artifact was not read before this review.

## Independent Reconstruction

- All six recorded input hashes match their current read-only sources.
- Direct reconstruction from EB-04S recovers exactly `16` independent-lane
  baseline forcing-robust failures in the declared `9/2/2/2/1` signature split.
- All `64` B/L/S/LS ordinal labels are `fail`.
- Comparing the existing primary metric's exact distance from its unchanged
  target reproduces LS directions of `15 away / 1 unchanged / 0 toward`.
- The five declared target-sensitive timing cells independently reproduce
  `4 away / 1 unchanged / 0 toward`.
- L and B primary values are exactly equal for all 16 rows. S moves 15 primary
  errors away and one toward; LS moves 15 away and one is unchanged. These
  values agree with the CSV and JSON summaries.
- All three SVGs parse. Each has a same-stem Markdown sidecar.
- The analytical self-check passes, including its mutated-inventory rejection.
- The diff is confined to the new package plus the three declared roadmap and
  catalog paths. Production, tests, contracts, EB-04R, and EB-04S are clean.
  `git diff --check` passes.

## Findings

### B-01 — Sidecar heading reverses the timing sign convention

Severity: `minor`; disposition required before terminal closure: `accepted and
fixed`.

`figures/eb04t-target-sensitive-timing.md` is titled
"Observed-minus-modeled timing response", while its caption, plotted axis, and
retained metric are modeled minus observed. The caption and interpretation are
correct; rename the heading to modeled-minus-observed so a scanning reader is
not given the opposite sign convention.

### B-02 — Factorial association should remain explicit in process prose

Severity: `minor claim-boundary`; disposition required before terminal closure:
`accepted and fixed`.

`process-attribution.md` correctly says the package does not prove unique
causality, but then says sublimation "supplies the dominant adverse movement."
The factorial evidence establishes that S-enabled cells account for the
dominant adverse primary-metric movement in this retained population; it does
not uniquely assign the residual's cause to sublimation rather than interactions
with inherited state/process debt. Rephrase that sentence in associative terms.
The adjacent sentence may also state that L has **exactly zero effect on these
16 selected primary metrics**, rather than "essentially zero," while avoiding a
claim of zero effect on the broader trajectories.

## Science And Governance Assessment

The principal conclusion is supported. Eleven failure cells predominantly
measure density trajectory or depth-SWE geometry, making a total-failure-
reduction requirement an imperfect efficacy test for the target mechanisms.
However, the five peak/melt-out timing failures are directly or jointly
sensitive to energy/mass exchange, and their primary metrics provide no
favorable LS signal. Therefore the criterion is conservative and mixed in
alignment, but the retained evidence does not support treating EB-04S as a mere
governance technicality.

The ownership labels remain appropriately bounded as primary/mixed/indirect and
are supported by the cited contracts and prior diagnostic decomposition. Wind
redistribution and forcing representativeness remain hypotheses/routes, not
validated causes. The package preserves EB-04S's frozen nonpromotion outcome,
does not authorize default activation or another calibration/factorial round,
and limits any future promotion rule to prospective independent evaluation.

No blocking scientific, retained-input, arithmetic, protected-boundary,
roadmap, or catalog defect was found. After B-01 and B-02 are corrected and
recorded in finding disposition, this review supports `GO` to terminal
verification and diagnostic closure.

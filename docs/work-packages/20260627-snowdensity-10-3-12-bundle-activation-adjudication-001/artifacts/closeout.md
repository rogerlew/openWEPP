# Closeout

Evidence mode: Static + Ran.

Disposition: `HOLD-OPT-IN-BUNDLE`

## Result

The combined opt-in bundle was rerun through the real direct-production WAT path
across all seven snow-depth fidelity surfaces:

`coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1`

The bundle is the best current snow-depth path but is not activation-ready under
Activation Policy B because full-model-surface no-regression evidence was not
produced in this package.

- Default failures: `1147`.
- Holding-capacity-only failures: `761`.
- Bundle failures: `498`.
- Spring-densification failures: `502`.
- Paired rows: `1415`.
- Paired surfaces worse vs holding-only: `0`.
- Activation policy: `POLICY-B`.
- Activation ready: `false`.
- Frost attribution unblocked: `false`.

The result validates the bundle as an opt-in improvement and rejects default
activation for this package. Under the post-review policy, zero paired snow-
depth failures are not required for default activation; the binding activation
blocker is missing full-surface no-regression evidence for a global default
physics change. The paired snow-control gate still fails on `498/1415` rows and
therefore remains the frost-attribution blocker.

## Mechanism

The remaining failures are no longer a single compaction problem:

- Modeled over observed: `264` rows.
- Modeled under observed: `234` rows.
- March/April remaining failures: `197` rows.
- March/April under-persistence: `128` rows.
- March/April cap-limited or patchy depletion: `49` rows.
- March/April compaction-only headroom: `20` rows.

This confirms the 10.3.11 conclusion: the existing bulk compaction arm is useful,
but additional compaction-rate pressure is not the next lever. The near-symmetric
residual split means the remaining problem is no longer a single maritime over-
accumulation bias. The next diagnostic should try to falsify the lead hypothesis
that the under-persistence tail is a cost of the bulk-compaction arm, while
separately classifying the over-persistence tail for cap-limited open-surface
ablation or remaining mass excess.

## Post-Review Policy Incorporation

`artifacts/claude-review-activation-policy-b.md` was incorporated after the
initial closeout. `SC-SNOWFREEZE-001` v98 supersedes the earlier zero-failure
activation criterion with Activation Policy B:

- default activation requires strict improvement versus the current default on
  gate-eligible paired-snow surfaces;
- default activation also requires full-model-surface no-regression evidence
  across regression/identity suites, non-snow climates, erosion/water-balance
  surfaces, and watershed routing;
- remaining paired snow-depth failures do not automatically block default
  activation, but they do keep frost attribution blocked until snow control is
  good enough to isolate frost residuals.

The review also records the likely need to adjudicate a `550 kg m^-3` SNOBAL
density-cap re-anchor. This package did not change the `522 kg m^-3` cap; that
requires a separate contract-first package and Policy-B full-surface gate.

## Boundary Status

- Default activation changed: no.
- Full-surface no-regression evidence produced: no.
- Parser/runfile/user CLI selector added: no.
- Fixture inputs changed: no.
- Public output schema changed: no.
- New process physics added: no.
- Density cap changed: no.
- Observed depth/density consumed by runtime: no.
- Frost attribution authorized: no.

## Follow-Up

Keep `coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1`
opt-in. Do not default-activate it until a Policy-B package proves full-surface
no-regression. The next package should classify the residual tails, test whether
under-persistence is compaction-arm-induced, and define the full-surface
regression basis needed for a global activation decision. Do not add another
compaction-rate variant without new authority.

# Ratification Adjudication

Evidence class: `Static` plus prior package `Ran` evidence.

## Diagnostic Fraction

The `>0.25` systematic-timing-fraction cutoff used by the Step 1 routing script
is **not ratified** as an `INV-SNOWFREEZE-048` or `INV-SNOWFREEZE-050`
invariant. The accepted timing tolerance is `TOL-SNOWFREEZE-008` (`+/-14 d`).
Future routing scripts may use site-level fractions only as declared diagnostic
aggregation rules with sensitivity reporting.

## Metric Correspondence

The H1b diagnostic found the top-down thaw path is present. Branch 3 grows
`thdp`, the surface-thawed cap, while `frdp` remains the bottom extent of the
frozen domain. Therefore a late-thaw residual based on bare `frdp > 0` can be a
measurement-correspondence residual, not proof of missing top-down thaw physics.
`INV-SNOWFREEZE-047` now makes that distinction explicit.

## Bounded Residuals

Ratification accepts a vindicated model with named residuals, not a zero-residual
model:

- snow-buried thaw-late cells routed to forcing-limited or sparse-observation
  snow persistence, with no established Sleepers spring under-melt defect;
- two snow-free persistent thaw-late cells deferred to future wet-heat/Qwet work;
- two H1b `frdp` metric-edge cells plus `15` true-stall days out of `570`
  branch-3 warm/material days (`2.6%`);
- two early-onset cells carried as bounded onset residuals;
- Morris remains snow-control blocked for frost attribution, and Mandan/Reynolds
  remain inconclusive without paired snow depth.

## Activation Rule

The frost default follows the snow Policy B analog: default activation is allowed
after residuals are attributed and bounded and the full no-regression surface
passes. Supported modern single-OFE no-env runs select direct production. Current
multi-OFE/Wave-2 and legacy sidecar-discovery no-env runs fall back to
compatibility with an explicit fallback reason because those direct surfaces are
outside this package. Zero paired observation failures are not required because
that would fit the validation corpus rather than ratify robust model behavior.

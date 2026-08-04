# SNOWBIRD-RST-PREPEAK-FLUX-DIAGNOSTIC

Status: `complete / phase insufficient / pre-peak liquid release dominates / no correction`

Date: `2026-08-03`

Plan class: `Diagnostic sensitivity / observed-cohort attribution`

## Purpose

Determine whether the canonical hourly rain/snow threshold (`rst`) can explain
the low peak-SWE signal, and identify the process fluxes responsible for SWE
loss before the observed annual peak.

## Frozen Experiment

Run the four retained SNOTEL mountain fixtures through the real direct-production
snowbench consumer at `rst = 0.0, 0.5, 1.0, ..., 4.0 deg C`. The initial arm
retains the active `harder_pomeroy_hourly` phase model. A prospectively frozen
extension, added after that arm proved `rst`-invariant, repeats the matrix with
the existing diagnostic `legacy_rst` selector so the threshold actually owns
the branch. The `0..1 deg C`
subset is the currently documented input domain. Values above `1 deg C` are an
explicit diagnostic stress range requested to expose a best-case phase bound;
they are not an admissible calibration recommendation, production proposal, or
range-policy amendment.

For every site/threshold/water year, independently reconstruct the accumulation
window from water-year start through the observed SNOTEL peak and report:

- modeled and observed peak SWE and peak timing;
- snow accumulation admitted and rain retained in the pack;
- actual snowpack SWE loss and sublimation;
- CoE raw melt demand, routed melt, liquid release, rain release, and refreezing
  as overlapping process diagnostics rather than additive mass sinks;
- storage change and daily/window mass closure; and
- sensitivity relative to the `rst = 0 deg C` control.

## Authority And Claims

- SNOTEL observations are `CALIBRATION` evidence already used in this campaign.
- The run changes only copied diagnostic `snow.txt` sidecars under the package
  target tree; production defaults, parsers, contracts, fixtures, and kernel
  physics remain read-only.
- Thresholds above `1 deg C` are `ASSUMED_FOR_EXECUTION` stress values. They may
  bound attribution but cannot be promoted or described as calibrated.
- This package may diagnose phase or loss ownership. It cannot correct or tune
  production physics.

## Write Set

- This package tree, terminal active-arm evidence under
  `target/snowbird_rst_prepeak_flux_diagnostic/`, terminal extension evidence
  under `target/snowbird_rst_prepeak_flux_legacy_rst_extension/`, and the
  explicitly rejected pre-receipt schema-mismatch attempt under
  `target/snowbird_rst_prepeak_flux_diagnostic_rejected_schema_mismatch/`.
- `docs/work-packages/README.md` and
  `docs/planning/snow-surface-energy-balance-roadmap.md`.

## Exit Criteria

- Freeze source identity, fixtures, observations, binary, tool, thresholds,
  environment, operators, and claims before result-bearing execution.
- Execute both `4 sites x 9 thresholds = 36`-cell phase arms through the real
  consumer and prove which phase model consumes `rst`.
- Independently reconstruct annual phase/storage/flux ledgers with two-sided
  closure and distinguish additive mass terms from overlapping diagnostics.
- Quantify threshold elasticity, best-case response, cross-site consistency,
  and dominant pre-peak loss contributors.
- Run syntax, JSON, Markdown, protected-path, exact-diff, and overwrite gates.
- Complete dual independent science review, finding disposition, dual terminal
  verification, and truthful final disposition.

## Delegation

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only science/data-governance
reviewers and two read-only terminal verifiers. Expected outputs are compact
package-local review and verification evidence; write access is read-only.

## Progress

- [x] (2026-08-03) User authorized the `0..4 deg C` stress range and required
  prior work to be committed and pushed first.
- [x] (2026-08-03) Prior increment committed and pushed as `afd80fdb`.
- [x] (2026-08-03) Diagnostic scope, stress semantics, and operators frozen;
  the initial write set was later found incomplete and is not claimed as a
  valid prospective freeze.
- [x] (2026-08-03) Executed the 36-cell active Harder-Pomeroy arm; all cells
  were exactly invariant because that phase model does not consume `rst`.
- [x] (2026-08-03) Prospectively froze a `legacy_rst` extension to measure the
  requested threshold best-case response through the existing diagnostic selector.
- [x] (2026-08-03) Executed the 36-cell legacy-RST extension and reconciled
  both phase arms. Phase is insufficient; pre-peak liquid release dominates.
- [x] (2026-08-03) Accepted review finding and amended the terminal write set
  to name the extension and preserved rejected-run trees.
- [x] (2026-08-03) Completed dual fresh review, dual terminal verification,
  direct gates, and closeout.

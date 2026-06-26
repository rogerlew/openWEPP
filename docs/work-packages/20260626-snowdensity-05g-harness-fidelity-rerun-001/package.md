# SNOWDENSITY-05G Harness Fidelity Rerun

Status: complete.

Package type: contract-first diagnostic harness correction and adjudication
rerun.

Primary authority: `SC-SNOWFREEZE-001` v83, especially
`INV-SNOWFREEZE-050`, `INV-SNOWFREEZE-052`, `INV-SNOWFREEZE-053`,
`INV-SNOWFREEZE-055`, `INV-SNOWFREEZE-056`, and `INV-SNOWFREEZE-057`.

Closure target: COMPLETE-05G-HARNESS-FIDELITY-RERUN.

Objective: clear the SNOWDENSITY-05F harness-fidelity caveat before density
work by making the diagnostic CoE melt replay use configured coniferous-forest
canopy (`cancov` near `0.9`) and either native openWEPP shortwave or an
explicit like-for-like proof of the PySnobal-bridge shortwave inversion, then
rerun the five-site SNOTEL melt adjudication.

## Scope

- Amend `SC-SNOWFREEZE-001` to capture the SNOWDENSITY-05G harness-fidelity
  acceptance rule.
- Correct the diagnostic-only `openwepp-snowbench coe-melt` replay so
  `canopy_cover_fraction` is sourced from the parsed/generated openWEPP runtime
  surface, not a `0.0` constant.
- Preserve `legacy_coe` as the default production model and keep
  `coe_shortwave_albedo_v1` opt-in only.
- Prove the shortwave input used by `coe-melt` is either native
  `winter.hourly.rad_mj_m2_####` or an exact inversion of the PySnobal bridge
  `net_solar = native_shortwave * 0.8`.
- Rerun the existing five-site SNOTEL `coe_melt_adjudication.py` comparison and
  record the representative-regime verdict.

## Non-Scope

- No density/pack physics change.
- No default activation.
- No production parser, runfile, CLI, or output-schema selector.
- No melt coefficient, albedo constant, or shared-radiation retuning.
- No `dense_slow_melt_v1` promotion.
- No frost attribution or non-SNOTEL snow-control verdict change.

## Acceptance Gates

- Canonical contract v83 records the 05G correction and prohibits
  `cancov = 0.0` diagnostic melt adjudication for the configured coniferous
  validation fixtures.
- Focused tests prove the diagnostic replay consumes nonzero configured canopy
  (`~0.9` for the CSS Lab fixture) and reports the shortwave bridge proof.
- SNOTEL adjudication emits profiles for both `legacy_coe` and
  `coe_shortwave_albedo_v1` across all five sites.
- The disposition reads forcing-robust rubric cells as the verdict surface and
  does not claim default activation from the rerun.
- Source scans prove no default activation, production selector, output schema,
  coefficient, or density-physics change occurred.
- Required gates pass:
  `cargo fmt --check`,
  `cargo clippy -p openwepp-runner --bin openwepp-snowbench -- -D warnings`,
  focused SNOWDENSITY contract tests, and `git diff --check`.

## Phase Plan

1. Read 05F closeout, `SC-SNOWFREEZE-001`, strategy §7, and the 05E
   adjudication tooling.
2. Scaffold package prompts and artifacts.
3. Amend the contract and contract-derived tests.
4. Correct diagnostic `coe-melt` canopy sourcing and shortwave lineage
   reporting.
5. Rerun focused tests and five-site SNOTEL adjudication.
6. Record implementation evidence, reviews, verification, line-count
   governance, and final disposition.

## Subagent Authorization

Subagent authorization: this package does not explicitly authorize spawning or
delegating to subagents. Reviews and verification are local unless a later
operator request adds explicit delegation authorization.

## Completion Summary

Closure: `COMPLETE-05G-HARNESS-FIDELITY-RERUN`.

Amended `SC-SNOWFREEZE-001` to v83 with `INV-SNOWFREEZE-057`,
`OBL-SNOWFREEZE-P-032`, and the 05G addendum. Corrected diagnostic
`openwepp-snowbench coe-melt` replay to consume configured runtime-surface
canopy (`generated_openwepp_runtime_surface.cancov`) rather than the old
`cancov = 0.0` harness constant. The representative SNOTEL fixtures all replay
with `canopy_cover_fraction = 0.9`.

The replay still transports forcing through the PySnobal CSV, but now publishes
the bridge identity proving like-for-like shortwave inversion:
`net_solar_Wm-2 = hrrad_MJ_m-2_h-1 * 1000000 / 3600 * 0.8`, and replay
`hrrad = net_solar_Wm-2 * 3600 / 1000000 / 0.8`.

Five-site SNOTEL adjudication closed `NON-PROMOTION` for default activation:
`legacy_coe` and `coe_shortwave_albedo_v1` both have robust failure count `9`;
the opt-in ordinal score rises from `84` to `86`, but the promotion rule
requires a lower robust failure count. The 05E `PROMOTION-CANDIDATE` result is
therefore superseded as regime-limited context.

No default activation, production parser/runfile/CLI selector, output schema,
coefficient, shared-radiation, density-physics, or frost-attribution change was
made.

## Closeout Artifacts

- `artifacts/implementation-evidence.md`
- `artifacts/snotel-adjudication.{json,md}`
- `artifacts/gate-results.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/external-review.md`
- `artifacts/review-disposition.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
- `artifacts/line-count-governance-checklist.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/worker-handoff.md`
- `artifacts/disposition.md`

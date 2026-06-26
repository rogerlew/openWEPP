# SNOWDENSITY-05E Melt Rubric Adjudication

Status: complete.

Package type: diagnostic adjudication and evidence package.

Primary authority: `SC-SNOWFREEZE-001` v79, especially `INV-SNOWFREEZE-050`,
`INV-SNOWFREEZE-052`, `INV-SNOWFREEZE-053`, `INV-SNOWFREEZE-054`, and
`INV-SNOWFREEZE-055`.

Closure target: COMPLETE-05E-MELT-RUBRIC-ADJUDICATION.

Objective: compare `legacy_coe` and `coe_shortwave_albedo_v1` against the
snow/frost rubric without activating the opt-in melt path by default. The
comparison must use forcing-robust rubric signatures for disposition and report
forcing-limited magnitude cells separately.

## Scope

- Add diagnostic-only snowbench/profile tooling that can replay the typed CoE
  melt path for `legacy_coe` and `coe_shortwave_albedo_v1`.
- Generate five-site SNOTEL rubric profiles for both melt models.
- Generate the non-SNOTEL rubric baseline after 05D so frost-site evidence stays
  current.
- Compare forcing-robust SNOTEL cells against the existing H openWEPP/legacy
  comparators where available.
- Record a promotion, non-promotion, or hold disposition for the opt-in melt
  candidate.

## Non-Scope

- No default activation.
- No production runner parser, CLI, runfile, or output-schema selector.
- No coefficient fitting, SNOTEL-site tuning, or shared radiation forcing
  adjustment.
- No `dense_slow_melt_v1` promotion.
- No density/pack physics change.

## Acceptance Gates

- Diagnostic tooling is explicitly labeled non-production and cannot be confused
  with runtime activation.
- `legacy_coe` and `coe_shortwave_albedo_v1` profiles are emitted for all five
  SNOTEL fixtures or the package closes `HOLD` with the blocker.
- Non-SNOTEL rubric baseline is rerun or a blocker is recorded.
- The disposition reads forcing-robust rubric cells as the verdict surface and
  does not claim defects from observation disagreement alone.
- Source scans prove no default activation, parser surface, output schema, or
  degree-day benchmark promotion occurred.
- Required gates pass:
  `cargo fmt --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`,
  `cargo deny check`.

## Phase Plan

1. Read 05D handoff, H/I0 rubric evidence, and strategy sections.
2. Scaffold package evidence files and prompts.
3. Add diagnostic-only CoE melt snowbench replay and profile wrapper.
4. Add focused tests for diagnostic confinement and profile generation.
5. Generate SNOTEL and non-SNOTEL profile artifacts.
6. Compare cells, disposition findings, run gates, and close package.

## Subagent Authorization

Subagent authorization: this package does not explicitly authorize spawning or
delegating to subagents. Reviews and verification are local unless a later
operator request adds explicit delegation authorization.

## Completion Summary

Closure: `COMPLETE-05E-MELT-RUBRIC-ADJUDICATION`.

Implemented diagnostic-only `openwepp-snowbench coe-melt` replay for
`legacy_coe` and `coe_shortwave_albedo_v1`, plus a Python rubric wrapper for
five-site SNOTEL adjudication. The replay uses the typed CoE melt helper but is
not wired to runtime activation, parser surfaces, output schemas, or defaults.

SNOTEL rubric result: `coe_shortwave_albedo_v1` is a promotion candidate
relative to diagnostic `legacy_coe` replay (`robust_fail_count 13 -> 10`,
`robust_ordinal_score 61 -> 84`). H comparator context remains material:
H as-built openWEPP/legacy profiles were `robust_fail_count=9`,
`robust_ordinal_score=84`, so 05E does not authorize default activation.

Non-SNOTEL result: baseline rerun remains `openwepp_defective_cells=0`, with
three paired snow sites still `SNOW_CONTROL_FAILED` and two sites lacking paired
observed snow. Frost attribution remains blocked behind snow-depth structural
remediation.

## Closeout Artifacts

- `artifacts/implementation-evidence.md`
- `artifacts/gate-results.md`
- `artifacts/snotel-adjudication.{json,md}`
- `artifacts/non-snotel-baseline.{json,md}`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/review-disposition.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
- `artifacts/line-count-governance-checklist.md`
- `artifacts/kernel-profile-compliance-checklist.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/worker-handoff.md`
- `artifacts/disposition.md`

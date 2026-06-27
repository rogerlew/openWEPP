# SNOWDENSITY-10.3.6 - Winter-Thaw Melt Response Diagnosis

Status: complete (executed by Codex, 2026-06-27).

Package type: diagnostic defect-closure style adjudication package; event-window
snowmelt response evidence only; no production physics change.

Closure target: `COMPLETE-10-3-6-WINTER-THAW-MELT-RESPONSE-ADJUDICATED`.

## Objective

Decompose the SNOWDENSITY-10.3.4 rank-2 winter-thaw melt-response hypothesis by
comparing observed snow-depth ablation intervals against modeled CoE thaw-window
melt response for the same maritime Sleepers/Harvard paired surfaces, while
reporting HJ Andrews and Hubbard Brook as observation-blocked diagnostic-only
surfaces. This package decides whether winter-thaw melt response is
defect-eligible enough to justify a later opt-in correction package.

## Primary Authority

- `docs/planning/snow-frost-fidelity-strategy.md` section 10.3: after
  SNOWDENSITY-10.3.5c, the active route is 10.3.4 rank-2 winter-thaw melt
  response before sub-canopy longwave or rain heat.
- `docs/work-packages/20260627-snowdensity-10-3-4-maritime-overaccumulation-diagnosis-001/`:
  ranked winter-thaw melt response as defect-eligible with `167,815`
  positive-temperature snowpack hours over paired over-accumulating surfaces.
- `docs/work-packages/20260627-snowdensity-10-3-5c-phase-partition-snow-depth-impact-001/`:
  retired the opt-in phase partition as a snow-depth remediation candidate.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  `INV-SNOWFREEZE-047`, `INV-SNOWFREEZE-048`, and `INV-SNOWFREEZE-050`.
- ADR-0017: comparator/legacy agreement is a flag; observed snow-depth authority
  controls defect attribution.

## Correction Authority Envelope

In scope:

- A package-local diagnostic tool that runs or reuses `openwepp-snowbench
  coe-melt --model legacy_coe` for the 10.3.4 maritime surfaces.
- Event-window analysis over paired observed snow-depth intervals:
  - observed snow-depth loss;
  - modeled snow-depth loss;
  - modeled raw melt, routed melt, and SWE loss;
  - positive-temperature snowpack hours;
  - warm-rain heat equivalent as diagnostic context only.
- Surface-level and cohort-level disposition of observed thaw-ablation deficits.
- Observation-blocked diagnostic summaries for HJ Andrews and Hubbard Brook.
- Artifacts, package guard test, focused execution, reviews, verification, and
  closure disposition.

Protected boundaries:

- No production physics changes.
- No default activation.
- No parser/runfile/user CLI selector.
- No fixture input edits.
- No public output schema changes.
- No coefficient tuning, site constants, radiation tuning, canopy tuning, phase
  changes, density changes, frost changes, or compatibility-runtime changes.
- No defect verdict for observation-blocked surfaces.
- No sub-canopy longwave or rain-heat correction in this package.

## Intended Write Set

- `tools/snowfreeze_observed/winter_thaw_melt_response.py`
- `tests/integration/snowdensity10_3_6_winter_thaw_melt_response.rs`
- `Cargo.toml`
- `docs/planning/snow-frost-fidelity-strategy.md`
- `docs/work-packages/README.md`
- Package-local files under
  `docs/work-packages/20260627-snowdensity-10-3-6-winter-thaw-melt-response-001/**`

Any production Rust edit is out of scope.

## Required Evidence Artifacts

- `artifacts/required-reading-map.md`
- `artifacts/scaffold-evidence.md`
- `artifacts/winter-thaw-melt-response.json`
- `artifacts/winter-thaw-melt-response.md`
- `artifacts/no-scope-creep-scan.md`
- `artifacts/test-evidence.md`
- `artifacts/gate-results.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/review-disposition.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
- `artifacts/line-count-governance-checklist.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/worker-handoff.md`
- `artifacts/disposition.md`

Evidence must label `Static:` versus `Ran:`. Placeholder artifacts must be
updated during execution.

## Acceptance Gates

- The package tool remains diagnostic-only and uses `legacy_coe` CoE replay.
- The report includes event-window summaries for all four paired
  Sleepers/Harvard surfaces.
- The report includes observation-blocked HJ Andrews and Hubbard Brook surfaces
  as diagnostic-only evidence.
- The report separates winter-thaw melt-response evidence from rain heat and
  sub-canopy longwave; rain heat is reported but not corrected.
- The report disposition is one of:
  - `WINTER-THAW-MELT-RESPONSE-DEFECT-ELIGIBLE`;
  - `WINTER-THAW-MELT-RESPONSE-PARTIAL`;
  - `WINTER-THAW-MELT-RESPONSE-NOT-PRIMARY`;
  - `WINTER-THAW-MELT-RESPONSE-HOLD`.
- No production physics, default, fixture, schema, parser/CLI selector, tuning,
  phase, density, frost, longwave, or rain-heat change occurs.
- Focused gates:
  - `.venv/bin/python tools/snowfreeze_observed/winter_thaw_melt_response.py`
  - `cargo test --test snowdensity10_3_6_winter_thaw_melt_response`
- Final gates:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
  - `wctl doc-lint --path docs/work-packages`

Any failed, blocked, or unjustified not-run required gate prevents `COMPLETE`.

## Phase Plan

1. Scaffold package and record required reading.
2. Implement the winter-thaw event-window diagnostic tool and Rust package guard.
3. Execute diagnostic evidence across the maritime surface set.
4. Update strategy/index documentation with the observed disposition and next
   route.
5. Run focused/full gates and complete review, verification, line-count,
   handoff, and disposition artifacts.

## Subagent Authorization

Subagent authorization: none. This package does not explicitly authorize
spawning/delegating to subagents; required review and verification are performed
locally with evidence labels.

## Downstream

If winter-thaw melt response is defect-eligible, the next package should author
an opt-in correction package with contract-first operand authority and
independent melt/ablation reconstruction. If it is not primary, proceed to
10.3.4 rank-3 sub-canopy longwave / forest energy.

## Execution Disposition

Completed as `COMPLETE-10-3-6-WINTER-THAW-MELT-RESPONSE-ADJUDICATED`.

The diagnostic found `WINTER-THAW-MELT-RESPONSE-DEFECT-ELIGIBLE` across the four
paired Sleepers/Harvard surfaces: `219` observed thaw-ablation windows, `132`
under-ablation windows (`0.603`), `24.105 m` aggregate snow-depth loss deficit,
`19,166` positive-temperature snowpack hours, `8.685 m` raw CoE melt, `4.628 m`
modeled SWE loss, and only `0.190 m` warm-rain heat equivalent. HJ Andrews and
Hubbard Brook remain observation-blocked diagnostic-only surfaces.

No production physics, default activation, fixture input, public output schema,
parser/runfile/user selector, coefficient, radiation, canopy, phase, density,
frost, longwave, or rain-heat correction changed. The next route is a
contract-first opt-in winter-thaw melt-response correction package; rain heat and
sub-canopy longwave remain separate later levers.

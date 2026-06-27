# SNOWDENSITY-10.3.3 Gradient Melt Adjudication

Status: complete.

Package type: diagnostic evidence and rubric adjudication.

Primary authority: `docs/planning/snow-frost-fidelity-strategy.md` §10.3,
item 3 "Gradient Melt Adjudication"; `SC-SNOWFREEZE-001`
`INV-SNOWFREEZE-050`, `INV-SNOWFREEZE-057`, and `INV-SNOWFREEZE-063`.

Closure target: `COMPLETE-10-3-3-GRADIENT-MELT-ADJUDICATED`.

Objective: rerun the 05G-style CoE melt comparison across the confirmed
canopy-gradient fixtures using `legacy_coe` and `coe_shortwave_albedo_v1`, then
answer only whether the shortwave/albedo modernization earns value outside the
high-evergreen regime.

## Scope

- Use the current `tests/fixtures/cancov_forest/` canopy-gradient fixtures.
- Use installed stratified observations under
  `tests/fixtures/cancov_forest/observations/`.
- Reuse the v74+ snow/frost rubric profile implementation from
  `tools/snowfreeze_observed/snotel_density_three_way.py`.
- Run `openwepp-snowbench coe-melt` for both `legacy_coe` and
  `coe_shortwave_albedo_v1`.
- Produce regime profiles for conifer, mixed, deciduous, and open/pasture
  regimes.
- Treat exact observation-to-model bindings as verdict-bearing:
  Marcell conifer, Marcell deciduous, Marcell open, Harvard hardwood, and
  Harvard open.
- Report Harvard hemlock as unbound, not verdict-bearing.
- Report mixed-hillslope aggregate comparisons as diagnostic-only unless an
  explicit aggregate observation rule is recorded in this package.

## Non-Scope

- No production physics changes.
- No default activation.
- No parser, runfile, user CLI, or publication-schema selector changes.
- No melt coefficient, albedo constant, shared-radiation, canopy, density,
  frost, or snow/rain partition tuning.
- No site-specific parameter fitting.
- No promotion of `coe_shortwave_albedo_v1`; this package can only classify
  diagnostic value and route follow-up work.

## Acceptance Gates

- A package-local adjudication tool exists and runs both CoE melt models against
  the canopy-gradient comparison set.
- The tool uses per-day direct-runtime `cancov` from the snowbench sidecar, not
  a scalar replay constant.
- The report includes regime-level rubric summaries for conifer, mixed,
  deciduous, and open/pasture regimes.
- Verdict-bearing and diagnostic-only comparisons are separated in the report.
- Harvard hemlock is explicitly reported as unbound/non-verdict.
- The conclusion says whether `coe_shortwave_albedo_v1` earns value outside the
  high-evergreen regime, without retuning coefficients.
- Focused checks pass:
  - `.venv/bin/python -m py_compile tools/snowfreeze_observed/cancov_gradient_melt_adjudication.py`
  - `cargo build -q -p openwepp-runner --bin openwepp-snowbench`
  - `.venv/bin/python tools/snowfreeze_observed/cancov_gradient_melt_adjudication.py --output-dir target/snowdensity10_3_3_gradient_melt_adjudication --snowbench-binary target/debug/openwepp-snowbench`
  - `cargo test --test snowdensity10_3_3_gradient_melt_adjudication`
  - `cargo clippy --test snowdensity10_3_3_gradient_melt_adjudication -- -D warnings`
  - `cargo fmt --check`
  - `git diff --check`

## Phase Plan

1. Scaffold package and kickoff prompt.
2. Read §10.3, 10.3.1a, 10.3.2, observation manifests, and the 05G
   adjudication tool.
3. Add a package-local canopy-gradient adjudication tool.
4. Add a focused guard test for the package/tool/report contract.
5. Build snowbench, run the adjudication, and copy report artifacts into this
   package.
6. Record gate results, dual reviews, review disposition, dual verification,
   owned files, handoff, and disposition.

## Subagent Authorization

Subagent authorization: this package does not explicitly authorize spawning or
delegating to subagents. Reviews and verification are local unless a later
operator request adds explicit delegation authorization.

## Completion Summary

Closure: `COMPLETE-10-3-3-GRADIENT-MELT-ADJUDICATED`.

The diagnostic `openwepp-snowbench coe-melt` replay ran `legacy_coe` and
`coe_shortwave_albedo_v1` across seven comparison surfaces: five exact
verdict-bearing strata and two diagnostic mixed aggregates.

Result: `LOW-CANOPY-NON-PROMOTION`. The opt-in shortwave/albedo CoE melt path
does **not** earn value outside the high-evergreen regime on current
verdict-bearing evidence. Low-canopy exact-bound robust failures worsened from
`6` to `7`, while the robust ordinal score stayed flat at `70`. Whole
verdict-bearing evidence likewise worsened from `7` to `8` robust failures with
no score gain (`92 -> 92`).

Regime evidence:

- Conifer exact binding is neutral (`fail 1 -> 1`, score `22 -> 22`).
- Deciduous exact bindings worsen (`fail 3 -> 4`, score `34 -> 34`), driven by
  Harvard hardwood.
- Open/pasture exact bindings are neutral (`fail 3 -> 3`, score `36 -> 36`).
- Mixed aggregate evidence is diagnostic-only and worsens (`fail 5 -> 6`, score
  `33 -> 32`); it does not carry canopy-stratum verdict authority.

Harvard hemlock remains explicitly unbound to a pure conifer/hemlock model
hillslope and is excluded from verdict-bearing evidence.

No production activation, default, parser/runfile/user CLI selector, output
schema, coefficient, radiation, canopy, albedo, density, partition, frost, or
fixture-input change was made.

## Closeout Artifacts

- `artifacts/gradient_melt_adjudication.json`
- `artifacts/gradient_melt_adjudication.md`
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

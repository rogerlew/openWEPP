# SNOWDENSITY-10.3.4 Maritime Over-Accumulation Diagnosis

Status: complete.

Package type: diagnostic evidence and blocker ranking.

Primary authority: `docs/planning/snow-frost-fidelity-strategy.md` §10.3,
item 4 "Maritime Over-Accumulation Diagnosis"; `SC-SNOWFREEZE-001`
`INV-SNOWFREEZE-047`, `INV-SNOWFREEZE-048`, `INV-SNOWFREEZE-050`, and
`INV-SNOWFREEZE-063`.

Closure target: `COMPLETE-10-3-4-MARITIME-OVERACCUMULATION-DIAGNOSED`.

Objective: decompose HJ Andrews, Sleepers, Harvard, and Hubbard Brook snow
over-accumulation evidence into candidate causes and rank which mechanisms are
defect-eligible versus forcing-limited or observation-blocked.

## Scope

- Use existing in-repo fixtures:
  - HJ Andrews: `tests/fixtures/cancov_forest/hjandrews_conifer_or`.
  - Sleepers: `tests/fixtures/snowfreeze_observed/site1_sleepers_south_field_vt`
    and `tests/fixtures/snowfreeze_observed/site2_sleepers_w9_hardwood_vt`.
  - Harvard: `tests/fixtures/cancov_forest/harvard_deciduous_ma` and
    `tests/fixtures/cancov_forest/harvard_open_ma`.
  - Hubbard Brook: `tests/fixtures/cancov_forest/hubbardbrook_deciduous_nh` and
    `tests/fixtures/cancov_forest/hubbardbrook_mixed_nh`.
- Use installed observation tables where present:
  - Sleepers snow/frost observations under
    `tests/fixtures/snowfreeze_observed/observations/sites/`.
  - Harvard HF237 stratified observations under
    `tests/fixtures/cancov_forest/observations/sites/`.
- Run or read `openwepp-snowbench coe-melt` legacy-CoE diagnostic surfaces for
  the maritime fixtures.
- Decompose evidence for these candidate mechanisms:
  - snow/rain partition near 0 degC;
  - rain-on-snow heat / warm-rain energy;
  - winter-thaw melt response;
  - precipitation bias;
  - wind/undercatch;
  - representativeness;
  - possible sub-canopy longwave.
- Produce a ranked blocker disposition with evidence class per mechanism:
  `DEFECT-ELIGIBLE`, `FORCING-LIMITED`, `OBSERVATION-BLOCKED`,
  `LOW-PRIORITY`, or `NOT-SUPPORTED`.

## Non-Scope

- No production physics changes.
- No new observation acquisition.
- No fixture input edits.
- No default activation.
- No parser, runfile, user CLI, or publication-schema selector changes.
- No coefficient, radiation, canopy, albedo, density, snow/rain partition,
  rain-heat, longwave, frost, or precipitation tuning.
- No promotion or retirement decision for `coe_shortwave_albedo_v1`.

## Acceptance Gates

- A package-local diagnostic tool exists and generates JSON/Markdown reports.
- The report covers HJ Andrews, Sleepers, Harvard, and Hubbard Brook.
- Sites without installed paired snow observations are explicitly marked
  observation-blocked, not silently treated as pass/fail.
- Sleepers and Harvard use paired observed snow-depth rows where available.
- Mechanism dispositions are ranked and identify which follow-up mechanism, if
  any, is eligible for an opt-in physics candidate package.
- Focused checks pass:
  - `.venv/bin/python -m py_compile tools/snowfreeze_observed/maritime_overaccumulation_diagnosis.py`
  - `cargo build -q -p openwepp-runner --bin openwepp-snowbench`
  - `.venv/bin/python tools/snowfreeze_observed/maritime_overaccumulation_diagnosis.py --output-dir target/snowdensity10_3_4_maritime_overaccumulation_diagnosis --snowbench-binary target/debug/openwepp-snowbench`
  - `cargo test --test snowdensity10_3_4_maritime_overaccumulation_diagnosis`
  - `cargo clippy --test snowdensity10_3_4_maritime_overaccumulation_diagnosis -- -D warnings`
  - `cargo fmt --check`
  - `git diff --check`

## Phase Plan

1. Scaffold package and kickoff prompt.
2. Read §10.3.4, 10.3.3 closeout, fixture manifests, observation manifests,
   and relevant existing harness code.
3. Add a package-local maritime diagnostic tool.
4. Add a focused guard test for the package/tool/report contract.
5. Run snowbench diagnosis and copy report artifacts into this package.
6. Record gate results, dual reviews, review disposition, dual verification,
   owned files, handoff, and disposition.

## Subagent Authorization

Subagent authorization: this package does not explicitly authorize spawning or
delegating to subagents. Reviews and verification are local unless a later
operator request adds explicit delegation authorization.

## Completion Summary

Closed as `COMPLETE-10-3-4-MARITIME-OVERACCUMULATION-DIAGNOSED`.

The package added a diagnostic-only maritime over-accumulation tool, ran
legacy-CoE snowbench replay across seven HJ Andrews, Sleepers, Harvard, and
Hubbard Brook surfaces, and generated ranked JSON/Markdown evidence artifacts.
Four paired Sleepers/Harvard surfaces over-accumulate snow depth; HJ Andrews
and Hubbard Brook remain observation-blocked because paired snow-depth tables
are not installed.

Disposition: `PARTITION-THAW-FIRST`. The ranked defect-eligible mechanisms are
near-zero snow/rain partition, winter-thaw melt response, sub-canopy longwave or
forest energy, and then rain-on-snow heat. Rain heat is not first because the
current CoE path already contains `dmelt`, and event-window reconstruction must
prove that term is numerically inactive before a rain-heat production change.
Precipitation bias and representativeness are forcing-limited; wind undercatch
is not supported as a correction lever for modeled-over-observed snow depth.

No production physics, defaults, output schema, fixtures, coefficients,
selectors, radiation, canopy, albedo, density, partition, precipitation, frost,
or promotion decisions changed.

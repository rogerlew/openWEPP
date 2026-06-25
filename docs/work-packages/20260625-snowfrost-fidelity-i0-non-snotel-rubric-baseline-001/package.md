# SNOWFROST-FIDELITY-I0 Non-SNOTEL Rubric Baseline

Status: complete.

Package type: characterization / baseline.

Primary gap: `GAP-SNOWFREEZE-002`.

Objective: before snow-depth remediation, rerun the five
`tests/fixtures/snowfreeze_observed/` non-SNOTEL frost sites and score the
current openWEPP output with the `SC-SNOWFREEZE-001` v74 snow/frost fidelity
rubric (`INV-SNOWFREEZE-050`, `TOL-SNOWFREEZE-011`). The output is a
per-site/per-cell baseline profile that records which snow and frost signatures
are scored, unavailable, or blocked by snow-control, without changing production
physics.

This package follows `AGENTS.md`, `docs/work-packages/AGENTS.md`,
`docs/specifications/science-contracts/AGENTS.md`,
`docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`,
`tools/snowfreeze_observed/README.md`,
`tests/fixtures/snowfreeze_observed/README.md`, and SNOWFROST-FIDELITY-A through
H.

Subagent authorization: none. Execute locally and record review/disposition in
package artifacts.

## Scope

In scope:

- Add a diagnostic aggregation tool under `tools/snowfreeze_observed/` that
  consumes `observed_harness.py compare` reports and emits a v74 snow/frost
  rubric profile for the non-SNOTEL frost sites.
- Run current openWEPP `openwepp-cli-hill` against all five non-SNOTEL frost
  fixtures using the existing observed harness.
- Generate JSON and Markdown baseline reports under `target/` and copy the
  executed evidence into this package's `artifacts/`.
- Update `docs/work-packages/README.md` and `tools/snowfreeze_observed/README.md`
  so the baseline is discoverable.

Out of scope:

- No production snow/frost physics, constants, `Qwet`, `frzftp`, runtime
  activation defaults, compatibility deletion, or direct default activation.
- No claim that observation disagreement alone is `OPENWEPP-DEFECTIVE`.
- No SNOTEL acquisition or SSD-arm rerun; H already established the density fork.
- No remediation. The next remediation package consumes this baseline.

## Acceptance Criteria

- The normalized non-SNOTEL observation corpus validates.
- All five non-SNOTEL frost fixtures run through `observed_harness.py compare`
  and emit current comparison reports.
- A v74 profile report is emitted with:
  - snow cells using physical modeled `Snow-Depth`, not `Snow-Water`;
  - frost-tube magnitude cells only where frost-tube observations exist;
  - isotherm upper-bound/timing cells only where soil-temperature isotherm
    observations exist;
  - unavailable SWE, density, event, and conservation cells clearly marked;
  - ADR-0017 per-cell verdict posture and `openwepp_defective_cells = 0`.
- The report records snow-control blockers before frost attribution.
- `py_compile`, observation validation, the baseline command, and `git diff
  --check` pass.

## HOLD Boundaries

Close as `HOLD` only if the observation corpus no longer validates, one of the
five model runs fails, the rubric profile cannot be emitted non-tautologically,
or current output no longer includes WAT `Snow-Depth`/`frdp` needed by the
existing harness.

## Execution Plan

1. Add `tools/snowfreeze_observed/non_snotel_rubric_baseline.py`.
2. Validate observations and build/run `openwepp-cli-hill`.
3. Execute the baseline command across all five sites.
4. Copy generated JSON/Markdown to artifacts.
5. Record review, verification, line-count governance, and handoff.

# Verification - Codex

Evidence mode: Static + Ran.
Date: 2026-07-08.

## Commands Run

- Read governance and package context with `sed -n`: `AGENTS.md`,
  `docs/work-packages/AGENTS.md`,
  `docs/specifications/science-contracts/AGENTS.md`,
  `docs/standards/AGENTS.md`,
  `docs/standards/prompt-wording-guidance.md`, package `package.md`,
  analyzer, replay artifacts, review artifact, final disposition, worker
  handoff, predecessor attribution artifacts, and the current
  `SC-OFEROUTE-001` diff/sections.
- `PYTHONPYCACHEPREFIX=/tmp/openwepp-pycache .venv/bin/python -m py_compile docs/work-packages/20260708-laned-router-annual-sediment-adequacy-metric-authority-001/artifacts/analyze_annual_sediment_metric.py`
  - Result: exit 0.
- Temp replay, to avoid rewriting package artifacts:
  - Copied `analyze_annual_sediment_metric.py` into
    `/tmp/openwepp-replay.*/docs/work-packages/20260708-laned-router-annual-sediment-adequacy-metric-authority-001/artifacts/`.
  - Symlinked the source `coupled-spacetime-summary.json`.
  - Ran `.venv/bin/python /tmp/openwepp-replay.*/docs/work-packages/20260708-laned-router-annual-sediment-adequacy-metric-authority-001/artifacts/analyze_annual_sediment_metric.py`.
  - Result: generated 21 comparisons, 1 strict blocker, 0 rev-44 blockers;
    generated JSON matched the package JSON after removing only
    `created_utc`; generated Markdown matched exactly.
- Standalone `.venv/bin/python - <<'PY'` recomputation, independent of the
  analyzer module, reading
  `docs/work-packages/20260708-laned-router-tier2-dx5-coupled-spacetime-ratification-001/artifacts/coupled-spacetime-summary.json`,
  pass parquets, and the package replay JSON.
  - Result: source comparisons 21; replay comparisons 21; no missing or extra
    comparison keys; independent strict blockers 1; independent rev-44
    blockers 0; max absolute delta versus replay metrics 0.
- `git diff --check -- docs/ROADMAP.md docs/work-packages/README.md docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
  - Result: exit 0.
- `.venv/bin/python tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
  - Result: `PASS-DEFERRED ... 8 binding exposure row(s), 7 science-review-follow-on row(s) not yet consolidated`; exit 0.

Full Rust gates were not run. I saw no Rust source changes and this package is
docs/contract/analyzer scoped. I did not identify or run a repo-local Markdown
lint command for the touched docs.

## PASS/FAIL Table

| Claim / Gate | Status | Evidence |
|---|---|---|
| Required local instructions and package context read | PASS | Root, work-package, science-contract, standards, package, analyzer, replay, contract diff, and predecessor attribution context were read. |
| Analyzer compiles | PASS | `py_compile` exit 0. |
| Replay script reruns | PASS | Temp replay exit 0; generated replay matched checked-in replay except JSON `created_utc`. |
| All selected-cohort annual pass-sediment comparisons are covered | PASS | Source comparison set count 21; replay comparison set count 21; missing 0; extra 0. |
| Replay blocker counts are reproducible | PASS | Independent parquet recomputation matched replay exactly: strict blockers 1, rev-44 blockers 0, max metric delta 0. |
| `SC-OFEROUTE-001` rev-44 metric scope is contract-recorded | PASS | Current diff adds material-year plus annual-vector authority to active mesh-policy surfaces, `INV-OFEROUTE-013`, tolerance notes, BEI row, and change log. |
| No production default flip in this package | PASS | Contract diff keeps active production default fixed at `10 cells/OFE` and target-`dx` diagnostic/non-promotional. Package handoff names default promotion as follow-on. |
| `git diff --check` for touched tracked docs | PASS | Exit 0 for `docs/ROADMAP.md`, `docs/work-packages/README.md`, and `SC-OFEROUTE-001.md`. |
| BEI checker for touched contract | PASS | Checker exits 0 with standing `PASS-DEFERRED` posture: 8 rows, 7 existing `science-review-follow-on` rows. |
| Required package `gate-results.md` artifact exists and records gates | FAIL | `artifacts/gate-results.md` is absent. |
| Review/verification findings are dispositioned before final handoff | FAIL | `final-disposition.md` still says review/verification artifacts are pending and does not disposition `review-codex.md` findings. |
| Markdown/doc lint gate is evidenced | FAIL | Package requires Markdown/doc lint, but no package gate artifact records it and I did not find/run a repo-local Markdown lint command. |
| Full Rust closure gates required here | PASS | Not required for this docs/contract/analyzer-only write set; no Rust code changed. |

## Exact Counts

- Source summary SHA-256 recorded by replay:
  `a1e5d1d8886e629cf841aefa191cc26453d82d34bcf096b65e7ca69c8547b870`.
- Source selected-cohort comparisons: 21.
- Replay JSON comparisons: 21.
- Missing source comparisons from replay: 0.
- Extra replay comparisons: 0.
- Members covered: 3.
  - `mn_corn_h4`: 7 comparisons.
  - `n_idaho_forest_h1`: 7 comparisons.
  - `wa_cascades_forest_h1`: 7 comparisons.
- Roles covered: 7 roles, 3 comparisons per role:
  `candidate_vs_reference_dt300`, `candidate_vs_reference_dt75`,
  `fine_reference_adequacy_dt300`, `fine_reference_adequacy_dt75`,
  `timestep_control_dx1p25`, `timestep_control_dx2p5`,
  `timestep_control_dx5`.
- Pre-rev44 strict-relative annual sediment blockers: 1.
  - `wa_cascades_forest_h1`, `fine_reference_adequacy_dt75`,
    `dx2p5_dt75` vs `dx1p25_dt75`, `tdep:4`:
    `0.022131683796129127 > 0.006666666666666667`.
- Rev-44 annual sediment blockers: 0.

## Verification Findings

1. Closure/gate evidence is incomplete. The package claims
   `EXECUTED-COMPLETE-METRIC-AUTHORITY`, but its required `gate-results.md`
   artifact is absent. The narrow commands above pass, but they are not a
   replacement for the package-local gate artifact required by `package.md` and
   `docs/work-packages/AGENTS.md`.

2. Final disposition is not current. `final-disposition.md` still says
   `review-codex.md` and `verification-codex.md` are pending. The current
   `docs/ROADMAP.md` and `docs/work-packages/README.md` diffs appear to address
   the review's stale-routing finding, but that finding is not dispositioned in
   the final artifact, and the gate-results finding remains open.

3. No replay adequacy defect found. The annual sediment replay covers all 21
   selected-cohort comparisons. The sole strict-relative blocker is the known
   WA refined-75 low-contribution `tdep:4` surface, and the rev-44
   material-year plus annual-vector rule produces zero annual sediment
   blockers in the selected cohort.

4. No contract-scope defect found in the rev-44 diff. The contract amendment is
   limited to mesh-policy annual pass-sediment metric authority and preserves
   the explicit non-authorization boundary for routed water, routed shape,
   storage, tail-fold, closure, active selector, shadow mesh, production
   default, and sediment process physics.

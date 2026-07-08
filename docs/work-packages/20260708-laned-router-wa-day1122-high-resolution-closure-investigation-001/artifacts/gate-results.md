# Gate Results

Status: EXECUTED-HOLD-ACTIVE-ROUTER-CLAMP-NUMERICS
Evidence mode: Ran.

## Required Gates

| Gate | Status | Evidence |
|---|---|---|
| `git diff --check` | PASS | Ran from repo root after package edits; no output. |
| Markdown/doc lint for touched docs | PASS | `markdown-doc lint --path docs/work-packages/20260708-laned-router-wa-day1122-high-resolution-closure-investigation-001` -> `13 files validated, 0 errors, 0 warnings` before closure artifacts; final rerun after closure artifacts recorded below. |
| Package-local WA analysis script | PASS | `.venv/bin/python .../artifacts/analyze_wa_day1122.py` -> `{"status": "PASS"}`. |
| Current WA active-plain rerun | PASS | `.venv/bin/python .../artifacts/run_mesh_ladder.py --members wa_cascades_forest_h1` built release binary and ran six rungs. Command exited `1` because expected fine rungs failed; evidence captured in `mesh-ladder-summary.md/json`. |
| Active-mode closure evidence for `dx2p5` and `dx1p25` | PASS | Both fail first at `laned_active_day_cascade_residual` on day 1122 with operands recorded in `day1122-reproduction.md`. |
| Completed-rung trace evidence for `baseline_fixed10`, `dx20`, `dx10`, `dx5` | PASS | Completed rungs produced `laned_active_trace.jsonl`; day-1122 books and day-1418 magnitude attribution generated from traces. |
| Static production default remains fixed `10 cells/OFE` | PASS | `rg` confirmed `LANED_ACTIVE_DEFAULT_CELLS: usize = 10`, runner target-`dx` selector remains env-gated, and `SC-OFEROUTE-001` records selector absent means fixed `10 cells/OFE`. |
| Python diagnostic scripts syntax | PASS | `.venv/bin/python -m py_compile .../analyze_wa_day1122.py .../run_mesh_ladder.py`. |
| Independent review | PASS | `review-agent-a.md`; findings accepted/dispositioned. |
| Independent verification | PASS | `verification-agent-a.md`; no numeric discrepancies. |

## Conditional Gates

| Gate | Status | Evidence |
|---|---|---|
| `cargo fmt --check` | NOT RUN | No Rust source was edited. |
| `cargo clippy --workspace --all-targets -- -D warnings` | NOT RUN | No Rust source was edited. |
| `cargo nextest run --workspace --profile full` | NOT RUN | No Rust source was edited; package-local rerun is the relevant runtime evidence. |
| `cargo deny check` | NOT RUN | No dependency or Rust source changes. |
| Contract/profile/BEI checks | NOT RUN | No `SC-*` contract, BEI, or profile file was edited. |
| Authority anti-evasion guards | NOT RUN | No required-case binding, cohort fixture posture, or external-authority suite posture was edited. |

## Final Closure Reruns

These were rerun after adding review, verification, disposition, and handoff
artifacts:

| Gate | Status | Evidence |
|---|---|---|
| Markdown/doc lint final | PASS | `markdown-doc lint --path docs/work-packages/20260708-laned-router-wa-day1122-high-resolution-closure-investigation-001` -> final clean result in `markdown-doc-lint.log`. |
| `git diff --check` final | PASS | No output. |

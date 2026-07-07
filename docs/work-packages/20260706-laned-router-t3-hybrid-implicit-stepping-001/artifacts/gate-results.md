# Gate Results (LANED-T3, through I2-experimental + review fixes)

Status: **EXECUTED for the experimental increment; CLOSURE gates for rev-28/31
ratification remain HELD** (gate non-deferral: they hold the package, not
the earlier increments).

Evidence mode: **Ran** for every PASS row (2026-07-06/07 sessions).

| Gate | Status | Evidence |
|---|---|---|
| `cargo fmt --check` | PASS | final tree |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | final tree |
| Focused suites (implicit_recession / hybrid / cascade / kinematic_wave / laned_active) | PASS | incl. the review-required direct LOW-jump→HIGH-root vector and the dust-accumulation vector (post-fix) |
| `cargo nextest run --workspace --profile full` | PASS | 1417/1417 at `bd64d2c8`; re-run post-review-fixes recorded in `review-disposition.md` |
| `cargo deny check` | PASS | no dependency changes in T3; re-run post-fix |
| Contract/BEI lint | PASS | `check_sc_binding_exposure.py` PASS-DEFERRED posture unchanged |
| H2637 hybrid endpoint (strict rule) | PASS | 37.0-37.2 s ×3; all rev-27 day-closure hard-fails green (seam 1.7e-14 / cascade 6.4e-14 / identity 2.1e-13) |
| H2637 plain-active no-perturbation | PASS | parquet hash unchanged vs pre-T3 (`21c54bf2…`) |
| I1 acceptance ladder | PASS | dt/mesh convergence + exact ledgers (`i1-implicit-stepper-evidence.md`) |
| **I2 closure: full hybrid Case-4 oracle ladder** | **FAIL — HELD** | rev-31 rerun package `20260707-laned-router-t3-ratification-solve-cost-001`: retained vector failed peak errors `22.8% / 15.5% / 10.2%` vs ratified `5%` tolerance |
| **I2 closure: fidelity-tolerance ratification** | **BLOCKED — HELD** | blocked by Case-4 hybrid ladder failure; rev 31 records selector remains experimental/unpromoted |
| Dual review | EXECUTED | Codex code lane NO-GO-for-settling-rev-28 (2 High, fixed — `review-disposition.md`); QA lane GO-WITH-AMENDMENTS (3 Medium, fixed) |

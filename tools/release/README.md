# Release Gate Automation

This directory hosts repository-local automation for openWEPP release gates.

## Scripts

- `run_release_candidate_gates.sh`
  - Runs workspace gates (`fmt`, `clippy`, `test`, `deny`), builds release
    binaries, stages release artifacts, emits sidecars, and runs
    `open_wepp_runner release lint`.
  - Optionally runs stability cohort gate unless `--skip-stability` is passed.
- `run_hillstab_gate.sh`
  - Executes the HILLSTAB01 cohort harness and applies pass/fail assertions.
- `assert_hillstab_success.py`
  - Validates HILLSTAB01 JSON suite summaries and exits non-zero on failures.

## Typical Usage

```bash
bash tools/release/run_release_candidate_gates.sh --skip-stability
```

```bash
bash tools/release/run_release_candidate_gates.sh \
  --cohort-seeds-csv /workdir/wepp-forest/docs/work-packages/20260503-wb05b-forest-hillslope-closure-sweep/artifacts/audits/_meta/defect_seeds.csv \
  --watchlist-csv /workdir/wepp-forest/docs/ablation/hillslope_watchlist.csv \
  --expect-suite wb05b_1166=1166 \
  --expect-suite release_gate_watchlist=19
```

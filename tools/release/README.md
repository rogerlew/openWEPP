# Release Gate Automation

This directory hosts repository-local automation for openWEPP release gates.

Required authority lane runs by default.

## Scripts

- `run_release_candidate_gates.sh`
  - Runs workspace gates (`fmt`, `clippy`, `test`, `deny`), builds release
    binaries, stages release artifacts, emits sidecars, and runs
    `open_wepp_runner release lint`.
  - Evaluates external-authority suite lanes from
    `docs/specifications/external-authority/registry.yaml`:
    - verifies fixture integrity for all active suites before lane execution:
      - `fixtures.sha256` (`sha256sum --check --strict`)
      - `fixtures.provenance.yaml` (required per-fixture provenance keys)
    - `required` lane runs by default (blocking on `hard-fail`).
    - `periodic` lane runs when `--run-authority-periodic` is set.
    - `manual` lane runs when `--run-authority-manual` is set.
    - `investigation` failures are recorded and surfaced as non-blocking.
  - Optionally runs stability cohort gate unless `--skip-stability` is passed.
- `run_hillstab_gate.sh`
  - Executes the HILLSTAB01 cohort harness and applies pass/fail assertions.
- `assert_hillstab_success.py`
  - Validates HILLSTAB01 JSON suite summaries and exits non-zero on failures.
- `check_authority_suite_antievasion.sh`
  - Runs diff-based anti-evasion checks for suites declared in
    `docs/specifications/external-authority/required-suite-obligations.json`.
  - Guards against:
    - required-case anchor removal,
    - cohort cardinality shrinkage,
    - threshold loosening,
    - lane/failure posture changes without control-path updates,
    - non-blocking Level-4 suites without queued/in-progress closure package
      linkage in `docs/work-packages/README.md`.

## Typical Usage

```bash
bash tools/release/run_release_candidate_gates.sh --skip-stability
```

```bash
bash tools/release/check_authority_suite_antievasion.sh --base-ref HEAD~1 --head-ref HEAD
```

```bash
bash tools/release/run_release_candidate_gates.sh \
  --run-authority-periodic \
  --cohort-seeds-csv /workdir/wepp-forest/docs/work-packages/20260503-wb05b-forest-hillslope-closure-sweep/artifacts/audits/_meta/defect_seeds.csv \
  --watchlist-csv /workdir/wepp-forest/docs/ablation/hillslope_watchlist.csv \
  --expect-suite wb05b_1166=1166 \
  --expect-suite release_gate_watchlist=19
```

## Authority Lane Flags

- `--skip-authority-required`
  - Skip the default required authority lane.
- `--run-authority-periodic`
  - Execute `gate_lane=periodic` suites from the authority registry.
- `--run-authority-manual`
  - Execute `gate_lane=manual` suites from the authority registry.
- `--authority-registry <path>`
  - Override registry location (default:
    `docs/specifications/external-authority/registry.yaml`).
- `--authority-report <path>`
  - Override report destination (default:
    `<release_dir>/authority_suite_results.md`).

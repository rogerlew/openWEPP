# Empirical Concurrency Sweep

Machine: `forest` development environment.

Static machine facts:

- CPU count: 48 logical CPUs (`nproc`)
- Memory: 125 GiB total, 107 GiB available at sweep start (`free -h`)

Method:

- Sweeps used `tools/local_ci/nextest_timing.py sweep`.
- Each run generated a temporary nextest config with one group cap patched.
- The committed `.config/nextest.toml` was not edited during measurement.
- Each row ran the same filterset for the compared caps.
- Each cap was run once. These are local-CI scheduling measurements for
  `forest`, not a statistical benchmark suite.
- For groups with `threads-required = 2`, a group cap of `4` permits two
  matching fixture tests to run at once.

## Commands

```bash
python3 tools/local_ci/nextest_timing.py sweep \
  --group runner-fixture \
  --caps 2,4 \
  --profile full \
  --filterset 'package(openwepp-runner) & test(/(r6j_|r7c_direct_production|r7d_direct|r7e_default_candidate|simimpl14_contract_gate)/)' \
  --top 8
```

```bash
python3 tools/local_ci/nextest_timing.py sweep \
  --group cli-fixture \
  --caps 2,4 \
  --profile full \
  --filterset 'binary(/^(cli01_runner_hillslope_integration|cli03_runner_contract_derived_tests|cli04_runner_wat_parquet_contract_derived_tests|watershed_cli_behavior_contract)$/)' \
  --top 8
```

```bash
python3 tools/local_ci/nextest_timing.py sweep \
  --group snowbench \
  --caps 1,2 \
  --profile full \
  --filterset 'binary(/^(snowdensity03_physics_bulk_offline_contract|snowdensity05e_melt_adjudication|snowdensity06_density_compaction)$/)' \
  --top 8
```

```bash
python3 tools/local_ci/nextest_timing.py sweep \
  --group frost-fixture \
  --caps 2,4 \
  --profile full \
  --filterset '(binary(/^(paradigm2.*|cancov_stratified_observations)$/) | test(/(frost|winter|stmtim)/)) - (binary(/^(snowdensity03_physics_bulk_offline_contract|snowdensity05e_melt_adjudication|snowdensity05g_harness_fidelity_rerun|snowdensity06_density_compaction|snowdensity06b_coe_bound_density_replay|snowdensity10_3_1a_per_day_cancov|snowfrost_fidelity_g0_pysnobal_bridge_contract)$/) | test(/(snowbench|pysnobal|snotel|coe_melt_snowbench|physics_bulk_snowbench|density_compaction_snowbench)/))' \
  --top 8
```

## Results

| Group | Filterset summary | Cap | Result | Nextest wall time | Decision |
|---|---|---:|---|---:|---|
| `runner-fixture` | runner R6J/R7C/R7D/R7E/SIMIMPL fixture tests | 2 | 8 passed | 71.948 s | raise |
| `runner-fixture` | same | 4 | 8 passed | 31.159 s | selected |
| `cli-fixture` | CLI01/CLI03/CLI04/watershed CLI binaries | 2 | 54 passed | 245.884 s | raise |
| `cli-fixture` | same | 4 | 54 passed | 120.590 s | selected |
| `snowbench` | representative snowdensity03/snowdensity05e/snowdensity06 subset | 1 | 7 passed | 262.323 s | selected |
| `snowbench` | same | 2 | 7 passed | 263.345 s | keep serial |
| `frost-fixture` | non-snowbench frost/winter/stmtim subset | 2 | 83 passed | 1.878 s | raise, low-confidence small subset |
| `frost-fixture` | same | 4 | 83 passed | 0.929 s | selected, low-confidence small subset |

## Decision

- Raise `runner-fixture.max-threads` from `2` to `4`.
- Raise `cli-fixture.max-threads` from `2` to `4`.
- Raise `frost-fixture.max-threads` from `2` to `4` for the non-snowbench
  frost diagnostics. This is a low-confidence but low-risk local scheduling
  change because the measured subset was small and snowbench remains isolated
  in its serial group.
- Keep `snowbench.max-threads = 1`; the representative cap-2 sweep was not
  faster on `forest`.
- Keep full-suite closure intact; these changes only improve scheduling.

## Assignment Verification

`cargo nextest show-config test-groups --profile full` confirmed the heavy
snowbench binaries remain assigned to `snowbench (max threads = 1)`, while
non-snowbench frost diagnostics are assigned to `frost-fixture (max threads =
4)`.

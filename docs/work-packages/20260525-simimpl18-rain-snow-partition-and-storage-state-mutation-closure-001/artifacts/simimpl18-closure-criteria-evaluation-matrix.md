# simimpl18-closure-criteria-evaluation-matrix

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- Matrix evaluates SIMIMPL18 exit criteria from `package.md` against final
  rerun/gate evidence.

## Ran
| Exit Criterion | Status | Evidence | Notes |
| --- | --- | --- | --- |
| Day-1 parity on targeted columns | fail | `replay-run-20260525T132822Z/candidate/H5.wat.dat`; `replay-run-20260525T132822Z/suite_parquet/investigation/baseline_wat_year_policy.dat` | First key mismatch persists (`RM`, `Snow-Water`, `Total-Soil`, `frozwt`, `SoilWaterTotal`). |
| Candidate storage surfaces not invariant across span | fail | `replay-run-20260525T132822Z/candidate/H5.hbp`; failing test `simimpl18_contract_requires_multi_day_storage_state_mutation` | Storage tuple remains constant over all rows. |
| No static-parameter publication leak | fail | `replay-run-20260525T132822Z/candidate/openwepp_hillslope_run_manifest.json` | `winter.ssd=250` and `hydout_equivalent.snow_water=250` with `runtime_swe=0`. |
| Baseline/candidate replay span closure over 1095 keys | pass-with-policy | `suite_parquet/investigation/pl14s_provenance_manifest.json`; `suite_dat/investigation/pl14s_provenance_manifest.json` | Achieved via explicit baseline-year policy materialization (`365 -> 1095`). |
| Full-span precipitation (`P`) parity over 1095 keys | fail | `suite_parquet/investigation/h5_wat_semantic_comparator.json`; `suite_dat/investigation/h5_wat_semantic_comparator.json` | `P.fail_count=447` (parquet) and `446` (dat). |
| Contract-derived tests fail pre-fix and pass final | fail | `artifacts/simimpl18-preimplementation-contract-gate.md`; `replay-run-20260525T132822Z/gates/test.stdout.log` | Pre-fix failure captured, but final pass not achieved. |
| Tier-A rerun bundle reproducible with logs/manifests/hashes | pass | `replay-run-20260525T132822Z/`; `replay-run-20260525T132822Z/evidence_sha256sums.txt` | Evidence bundle complete and checksummed. |
| Required gates pass (`fmt`,`clippy`,`test`,`deny`) | fail | `replay-run-20260525T132822Z/gates/gate_exit_codes.log` | `cargo test --workspace` fails (`rc=101`). |
| Contract-first sequence evidence complete | pass | contract evidence + pre-impl gate + implementation evidence artifacts | Sequence preserved. |
| Dual review and dual verification complete | partial | `review_agent_a.md`, `review_agent_b.md`, `verification_agent_a.md`, `verification_agent_b.md` | Artifacts present; independent-agent requirement remains governance gap. |

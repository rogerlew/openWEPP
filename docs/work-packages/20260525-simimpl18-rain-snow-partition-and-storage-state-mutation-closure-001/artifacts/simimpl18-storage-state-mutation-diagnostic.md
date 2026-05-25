# simimpl18-storage-state-mutation-diagnostic

Status: complete
Evidence mode: ran
Date: 2026-05-25

## Static
- Diagnostic target: confirm whether candidate published storage tuple mutates
  across executed span under varying forcing.

## Ran
- Evidence input:
  - `artifacts/replay-run-20260525T132822Z/candidate/H5.hbp`
- Candidate storage tuple (`Total-Soil`, `frozwt`, `Snow-Water`,
  `SoilWaterTotal`) unique-count over 1095 rows:
  - `1` unique tuple.
- Observed invariant tuple across span:
  - `Total-Soil=76.00`, `frozwt=0.00`, `Snow-Water=250.00`,
    `SoilWaterTotal=76.00`.
- Contract-test corroboration:
  - `simimpl18_contract_requires_multi_day_storage_state_mutation` fails in
    `tests/integration/pl14s_tier_a_candidate_emission_and_replay_contract.rs`.

## Interpretation
- Multi-day storage-state mutation closure remains open.

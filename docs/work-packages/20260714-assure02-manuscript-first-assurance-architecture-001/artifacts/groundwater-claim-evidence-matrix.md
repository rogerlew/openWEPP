# Groundwater Claim-Evidence Matrix

Status: complete for the ASSURE-02 prototype

Frozen assessed commit: `de520f1ff867ca5c65b1f82dfe32a19c213ae18c`

Current documentation checkout at intake:
`773eb3c56f0afcbc7f605d49c9a09d391e8113a5`

## Evidence Matrix

| Claim ID | Claim | Evidence type | Retained identity | Scope limit |
| --- | --- | --- | --- | --- |
| `GW-P01` | The authorized daily recurrence is `S_i = S_(i-1) + D_i - Qb_(i-1) - Qs_(i-1)`, followed by `Qb_i = kb S_i Δt` and `Qs_i = ks S_i Δt`, with `Δt = 1 d`; the contract and code use the equivalent one-day shorthand. | Formulation authority | `docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md`, SHA-256 `97ee00e87df4a87221aa34fc1f44c77176f43922bcfac96c69d4b6de8e230d60`; Srivastava et al. (2013), DOI `10.13031/2013.42691` | Linear daily routine only; later nonlinear formulations excluded |
| `GW-P02` | The openWEPP implementation applies prior-day exports before computing current-day exports. | Code verification | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/groundwater.rs`, SHA-256 `92674b52f7edd4ec67acd07a9069be9de4cfec6512f7c47e19d03f228746cb14`; `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs:1324` | Exact tested recurrence and admitted coefficient domain |
| `GW-P03` | A two-day vector yields storage `12.0` and `14.2 m3`, baseflow `1.2` and `1.42 m3`, and deep seepage `0.6` and `0.71 m3`, with a maximum observed absolute residual of `1.7763568394002505e-15 m3` against an absolute `1.0e-12 m3` implementation-test tolerance. | Analytical test vector | `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs:1324`; current execution and independent arithmetic record `docs/work-packages/20260714-assure02-manuscript-first-assurance-architecture-001/artifacts/groundwater-current-tree-confirmation.md`, SHA-256 `2e202908260751f1983ff7129623e485f05aa6510880266a596db22d532dacab` | Synthetic 1,000 m2 case; not field performance or solver-convergence evidence |
| `GW-P04` | Current authority admits finite `kb >= 0` and `ks >= 0` and fails closed if the combined daily exports exceed accepted storage. Negative `ks` or upward lower-aquifer recharge is outside the current model authority. | Domain-authority and guard verification | `docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md:139-148,283-294`; test `gwbaseflow_exports_over_accepted_storage_fail_closed` in `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs:1392` | No scientific upper bound is assigned to either individual coefficient; admissibility is recurrence- and timestep-specific |
| `GW-P05` | The retained H2637 run closes both published recurrence timing identities within `4.26e-11 m3`, against storage-scaled allowances of `1.2601452784040276e-7 m3` and `1.2097394672678664e-7 m3`. | Integrated recurrence and publication-ledger reconstruction | `docs/work-packages/20260713-integrated-validation-campaign-001/artifacts/final-conservation-and-consumer-evidence.md`, SHA-256 `306b96a1d45fca85d5604b16fe8ce4b814df48d2fc15ecb910e198085ee81f18`; `docs/work-packages/20260713-integrated-validation-campaign-001/artifacts/logs/final-reconstruction-arithmetic.log`, SHA-256 `4f2675a0fe57f4153f002a7051ae99971811af84c430ead68395d4205c52468e`; acceptance assertions in `tests/integration/laned_shadow_h2637.rs:655-672` | One 731-day, 19-OFE retained case; ledger closure is neither numerical convergence nor accuracy against observations |
| `GW-P06` | Generated baseflow and deep seepage traverse direct publication, hillslope binary pass serialization/parsing, watershed contribution, and the generated-baseflow channel branch; `cbase` is not substituted when the linear reservoir is enabled. | Real consumer-path verification | `docs/work-packages/20260709-laned-active-baseflow-export-closure-001/artifacts/consumer-path-proof.md`, SHA-256 `708a5d6fba629a3ba6945781b6aa625eaf89ab9b0583517fbfc37622b503a3fa`; focused test paths named there and in the current execution record | Tested consumers and branches only |
| `GW-P07` | Generated groundwater is excluded from the active surface-runoff source builder and therefore does not reenter the tested surface router. | Active-router negative proof | `docs/work-packages/20260708-groundwater-baseflow-laned-single-ofe-mofe-implementation-001/artifacts/consumer-path-proof.md`, SHA-256 `fed22fa4c031ef102c214a99fbbda9efbfcaaaa6e2b482f2883be4aa082764b4`, step 7 | Static source exclusion for the named active source builder; not a universal proof for future routers |
| `GW-P08` | Twelve declared groundwater implementation, publication, consumer, and test paths are unchanged between the frozen commit and the ASSURE-02 intake commit. | Static currency check | Exact ordered path list, command, empty output, and exit `0` in `docs/work-packages/20260714-assure02-manuscript-first-assurance-architecture-001/artifacts/groundwater-current-tree-confirmation.md`, SHA-256 `2e202908260751f1983ff7129623e485f05aa6510880266a596db22d532dacab` | Applies only to the twelve named paths; does not substitute for fresh integrated release transfer |
| `GW-P09` | Focused current-tree recurrence, guard, threshold, authority, and HBP-consumer tests pass. | Fresh executable confirmation | Exact nextest command, seven selected test names, run ID, output summary, and exit `0` in `docs/work-packages/20260714-assure02-manuscript-first-assurance-architecture-001/artifacts/groundwater-current-tree-confirmation.md`, SHA-256 `2e202908260751f1983ff7129623e485f05aa6510880266a596db22d532dacab` | Focused confirmation; not a full workspace or fresh H2637 rerun |

## Key-Finding Resolution

| Prototype key finding | Claim IDs |
| --- | --- |
| Daily recurrence and prior-day debit timing | `GW-P01` through `GW-P04`, `GW-P09` |
| 731-day production ledger reconstruction | `GW-P05` |
| Hillslope-pass transfer, watershed consumption, `cbase` separation, and active-router exclusion | `GW-P06`, `GW-P07`, `GW-P09` |

## Retained Run Values

| Operand | Value (`m3`) |
| --- | ---: |
| Initial storage, `S0` | 0.0 |
| Cumulative recharge, `sum(D)` | 3668.610172576748 |
| Cumulative baseflow, `sum(Qb)` | 3547.636225849919 |
| Cumulative deep seepage, `sum(Qs)` | 0.0 |
| Terminal pre-export storage, `SN` | 126.01452784040274 |
| Terminal-day baseflow, `QbN` | 5.04058111361611 |
| Terminal-day deep seepage, `QsN` | 0.0 |
| Recurrence residual | `-4.249045559845399e-11` |
| Complete post-export ledger residual | `-4.250466645316919e-11` |

The latest runoff-event HBP baseflow (`5.032033091000001 m3`) is explicitly not
used as terminal-day `QbN`; the latest runoff event need not be the last
simulation day.

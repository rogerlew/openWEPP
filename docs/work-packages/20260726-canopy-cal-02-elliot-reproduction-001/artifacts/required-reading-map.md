# Required Reading Map

Evidence class: `Static scaffold`

| Tier | Path | Purpose |
| --- | --- | --- |
| Core | `AGENTS.md` | Repository governance. |
| Core | `docs/work-packages/AGENTS.md` | Work-package execution and evidence rules. |
| Core | `tests/AGENTS.md` | Test and harness governance. |
| Core | `tests/fixtures/AGENTS.md` | Durable fixture provenance and checksum rules. |
| Core | `docs/work-packages/20260726-canopy-cal-02-elliot-reproduction-001/package.md` | Active package scope and acceptance. |
| Core | `docs/planning/canopy-phenology-assurance-roadmap.md` | Campaign Order 2 and final evidence purpose. |
| Core | `docs/work-packages/20260726-canopy-cal-01-source-target-ledger-001/package.md` | Dependency contract. |
| Core | `docs/work-packages/20260726-canopy-cal-01-source-target-ledger-001/artifacts/cal02-admission.json` | Exact execution admission; absent until CAL-01 completes. |
| Core | `docs/work-packages/20260726-canopy-cal-01-source-target-ledger-001/artifacts/target-ledger.csv` | Accepted targets and roles; absent until CAL-01 completes. |
| Core | `references/canopy_phenology/elliot_2026/README.md` | Installed source provenance; absent until CAL-01 completes. |
| Core | `references/canopy_phenology/elliot_2026/bill_elliot_2026_modeling_hardwood_and_mixed_forests_in_wepp.pdf` | Report figures/tables and method; absent until CAL-01 completes. |
| Conditional | `docs/standards/testing-and-gate-strategy.md` | Intent/terminal plan and campaign gate selection. |
| Conditional | `docs/standards/prompt-wording-guidance.md` | Prompt changes and required delegation wording. |
| Conditional | `tools/owcmp/AGENTS.md` | Read if the admitted harness uses or extends `owcmp`. |
| On-demand | `docs/work-packages/20260719-canopy-phenology-native-integration-001/artifacts/operand-lineage.md` | Native comparison definitions only; native calibration is excluded. |
| On-demand | `docs/specifications/science-contracts/contracts/SC-PLANT-001.md` | Canonical plant units/definitions if output aliases are disputed. |
| On-demand | `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md` | Canonical residue units/definitions if output aliases are disputed. |

`tools/agents/find-agents --for` reports that package/catalog paths inherit
`AGENTS.md` plus `docs/work-packages/AGENTS.md`; future tool paths inherit root
governance; future fixture paths inherit `AGENTS.md`, `tests/AGENTS.md`, and
`tests/fixtures/AGENTS.md`.

Required-reading budget: `7225123` current local bytes,
`REQUIRES-JUSTIFICATION`. This scaffold-time total uses the exact sibling
WEPPcloud PDF as the byte-equivalent source because the CAL-01 installed copy
and CAL-01 result artifacts do not yet exist. The commissioned PDF and complete
CAL-01 evidence are load-bearing reproduction inputs. Recompute this total
after CAL-01 closes and before CAL-02 intent admission.

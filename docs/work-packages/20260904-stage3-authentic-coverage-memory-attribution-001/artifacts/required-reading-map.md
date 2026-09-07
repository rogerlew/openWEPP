# Required-reading map

Status: `PHASE 0 COMPLETE — NO RUST WRITE AUTHORIZED OR NEEDED`

Evidence mode: `Static + Ran`

Core reading was completed before package edits. The pre-edit required set was
329,816 bytes (under the 400,000-byte guidance budget):

| Set | Paths and ranges | Bytes |
|---|---|---:|
| Core governance | `AGENTS.md` (1-160), `docs/work-packages/AGENTS.md` (1-388), `docs/codex_exec_plans.md` (1-244), `docs/standards/prompt-wording-guidance.md` (1-180), `docs/standards/testing-and-gate-strategy.md` (1-459), `docs/specifications/science-contracts/AGENTS.md` (1-77), `docs/standards/kernel-work-package-preparation.md` (1-255), `docs/standards/numerical-solver-architecture.md` (1-152), `crates/AGENTS.md` (1-75), `tests/AGENTS.md` (1-63) | 131,161 |
| Mandatory predecessor | `package.md` objective/authority/protected/progress/disposition sections (1-1318), `artifacts/final-disposition.md` (1-65), `artifacts/hold-legitimacy-audit.md` (1-91), `artifacts/gate-results.md` (1-75), component replay `contract_ref.md` (1-245) | 115,443 |
| Named raw evidence | `revision61_feed_forward_release_3run.log` (1-53), `component_dependency_replay_baseline_3run.log` (1-101), `component_dependency_replay_candidate_3run.log` (1-59), `carrier_static_attribution_one_ofe_release.log` (1-8) | 83,212 |

On-demand mechanism reads were deliberately separate from the pre-edit budget:
`performance-budget.md` (48,583 bytes), `workload-and-benchmark-matrix.md` (11,661 bytes),
`tolerance-authority.md` (11,902 bytes), `implementation-and-focused-validation.md`
(130,772 bytes), and the two candidate implementation reviews (19,686 bytes).
They were read only for the historical predicate, ceiling, and source-recovery
questions; they are not authority replacement. Conditional science-contract
text was consulted only for the existing dependency-replay binding. No source
edit was made, so no conditional kernel write path was opened.

The comparator role was attempted before heavy execution but failed with the
service usage-limit error recorded in `artifacts/raw/README.md`. The package
authorized a parent fallback for the bounded retained-source probes; those runs
are labeled parent fallback, not comparator output.

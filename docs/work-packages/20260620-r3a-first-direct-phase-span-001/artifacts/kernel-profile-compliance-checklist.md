# R3A Kernel Profile Compliance Checklist

Status: complete.
Evidence mode: Static + Ran.

| Requirement | Status | Evidence |
|---|---|---|
| Selected span has canonical compute authority | PASS | Arithmetic accounting over typed direct inputs only; no process physics. |
| Selected span includes inputs/compute/mutation/downstream/shadow | PASS | `contract-implementation-evidence.md` and focused tests. |
| Phase-span identity passes | PASS | `phase-span-identity-evidence.md`; exact binary-fraction fixture. |
| No output schema/unit/metadata meaning changes | PASS | No output writer/schema files touched; protected H2637 identity passes. |
| No publication cutover | PASS | Runner still publishes through compatibility scheduler path. |
| Direct mode inactive by default | PASS | Runner default-disabled fixture counters all zero; H2637 compat command used. |
| Direct span excludes compatibility types and calls | PASS | Forbidden-token scan no matches; scheduler no diff. |
| Runtime counters are non-tautological | PASS | Production opt-in compatibility-edge handoff plus zero-span edge evidence. |
| Default-disabled H2637 regression gate passes | PASS | Median `632.08 s <= 676.67 s`. |
| Gate Evidence Non-Deferral checked | PASS | Focused/full gates, benchmark, identity, review, verification, markdown lint, and diff check recorded. |

R3A remains pre-publication and pre-hydrology-path migration. It does not
authorize default activation.

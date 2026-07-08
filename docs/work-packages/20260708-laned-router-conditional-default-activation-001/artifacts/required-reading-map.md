# Required Reading Map

Status: `COMPLETE`
Evidence mode: Static.

## Core Reads

| Path | Disposition |
|---|---|
| `AGENTS.md` | Root package, contract, validation, and no-branch-switch governance. |
| `docs/work-packages/AGENTS.md` | Package lifecycle, gate non-deferral, consumer proof, review/verification, and subagent wording. |
| `docs/specifications/science-contracts/AGENTS.md` | Contract-first sequencing and SC amendment requirements. |
| `docs/standards/AGENTS.md` | Standards routing. |
| `docs/standards/prompt-wording-guidance.md` | Package prompt and subagent authorization wording. |
| `crates/AGENTS.md` | Rust implementation and closure-gate requirements. |
| `tests/AGENTS.md` | Test posture and focused/full gate selection. |
| `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` | Current active owner, dx5 mesh default, and default-activation authority surface. |
| `docs/work-packages/20260708-laned-router-dx5-production-mesh-policy-ratification-001/artifacts/final-disposition.md` | Rev 45 production active mesh default evidence. |
| `docs/work-packages/20260708-laned-router-dx5-production-mesh-policy-ratification-001/artifacts/worker-handoff.md` | Names default activation follow-ons and non-silent gates. |
| `docs/work-packages/20260707-laned-router-d16-hybrid-disturbed-route-coeff-source-acquisition-001/artifacts/openwepp-authority-lift.md` | Disturbed/native routing-coefficient parser/projection authority. |
| `docs/work-packages/20260707-laned-router-d16-hybrid-disturbed-route-coeff-source-acquisition-001/artifacts/active-consumer-proof.md` | Proof that active routing consumes projected `routing_coefficients`. |

## Implementation Reads

| Path | Disposition |
|---|---|
| `crates/openwepp-runner/src/hillslope/laned_active.rs` | Current explicit active selector and diagnostic selector guards. |
| `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs` | Runtime selection and active summary/manifest handoff. |
| `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs` | Lane authority and per-lane `ofe_routing` coefficient projection. |
| `tests/integration/laned_shadow_h2637.rs` | Existing active/shadow H2637 selector tests and coefficient fixture patch helper. |

## Operator Direction

The binding operator policy for this package is:

- all scheduled lanes have extended coefficients: Lane D active routes by
  default;
- no scheduled lanes have extended coefficients: legacy/default path runs;
- mixed extended and non-extended lanes: fail closed before streaming.

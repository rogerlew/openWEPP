# Required Reading Map

Status: `SCAFFOLD-RECORDED`

Scaffold-time byte count recorded on `2026-07-08` with `wc -c`.

## Threshold Disposition

- Core package reading: `510288` bytes - `WARN` (`>400000`, `<=800000`).
- Core + implementation-local reading: `800373` bytes -
  `REQUIRES-JUSTIFICATION` (`>800000` by `373` bytes).
- Full listed set, including contract-authoring and timing context:
  `844183` bytes - `REQUIRES-JUSTIFICATION`.

Justification: Tier 1 intentionally changes numerical method in the active
router, amends `SC-OFEROUTE-001`, touches D10B/Iwagaki oracle acceptance, and
requires active H2637 timing/fidelity evidence. The large total is driven by
the current size of `docs/work-packages/README.md`, `SC-OFEROUTE-001`, and the
hot solver file. Trimming the set below threshold would remove either governing
authority or directly targeted implementation/test context. An executor may
defer timing-context files until Phase F, but must record that deferral before
implementation.

## Core

| Bytes | File |
| ---: | --- |
| 10269 | `AGENTS.md` |
| 20708 | `docs/codex_exec_plans.md` |
| 16364 | `docs/work-packages/AGENTS.md` |
| 252269 | `docs/work-packages/README.md` |
| 5599 | `docs/specifications/science-contracts/AGENTS.md` |
| 3328 | `docs/standards/AGENTS.md` |
| 9780 | `docs/standards/prompt-wording-guidance.md` |
| 13488 | `docs/standards/kernel-work-package-preparation.md` |
| 5900 | `docs/backlog/20260706-laned-router-numerics-performance-tiers.md` |
| 157159 | `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` |
| 15424 | `docs/work-packages/20260708-laned-router-tier1-local-numerics-001/package.md` |

Core subtotal: `510288`.

## Implementation-Local

| Bytes | File |
| ---: | --- |
| 5171 | `crates/AGENTS.md` |
| 4534 | `tests/AGENTS.md` |
| 3501 | `docs/standards/local-ci-gate-selection.md` |
| 83938 | `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs` |
| 15630 | `crates/openwepp-hillslope-orchestrator/src/ofe_routing/friction.rs` |
| 19571 | `crates/openwepp-hillslope-orchestrator/src/ofe_routing/d10b_reconciliation_tests.rs` |
| 26399 | `crates/openwepp-hillslope-orchestrator/src/ofe_routing/iwagaki_oracle.rs` |
| 20755 | `crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs` |
| 6711 | `crates/openwepp-hillslope-orchestrator/src/ofe_routing/profile.rs` |
| 61324 | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs` |
| 15487 | `crates/openwepp-runner/src/hillslope/laned_active.rs` |
| 27064 | `tests/integration/laned_shadow_h2637.rs` |

Core + implementation-local subtotal: `800373`.

## Contract-Authoring Conditional

Read before the first `SC-*` edit:

| Bytes | File |
| ---: | --- |
| 12423 | `docs/specifications/science-contract-authoring-procedure.md` |
| 5044 | `docs/specifications/science-contracts/kernel-process-contract-profile.md` |
| 8690 | `docs/specifications/science-contracts/index.md` |

## Timing Context

Read before Phase F timing adjudication, or earlier if using the D15A profile
as implementation evidence:

| Bytes | File |
| ---: | --- |
| 7396 | `docs/work-packages/20260706-mofefid-d15-active-owner-optimization-001/artifacts/baseline-profile.md` |
| 4835 | `docs/work-packages/20260706-mofefid-d15-active-owner-optimization-001/artifacts/slot-profile.md` |
| 5422 | `docs/work-packages/20260706-mofefid-d15-active-owner-optimization-001/artifacts/optimization-plan.md` |

Full listed total: `844183`.

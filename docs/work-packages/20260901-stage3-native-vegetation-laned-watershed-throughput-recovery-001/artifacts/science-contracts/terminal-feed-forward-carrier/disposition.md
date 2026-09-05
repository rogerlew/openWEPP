# Typed feed-forward terminal-carrier finding disposition

Canonical contract:
`docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md`

Base commit under review:
`a28c55c2d0f06e0c4aab58642f1009f70f82b3d3`

First-pass ordered manifest:
`8bc5f02fe1f3f777fc01f7f488c7f14ad068139314acff3fe5e83ac389967be3`

Corrected ordered four-file contract/test manifest after the final A-004
taxonomy amendment:
`a8a6677672c7f8f34acdaca30f3b94388902d022fe5886f28e4532762ffce804`

Evidence mode: `Static + Ran + Expected-red`

| finding_id | source(agent_a/agent_b) | severity | decision | action_taken | artifact_ref | rationale |
|---|---|---|---|---|---|---|
| `FFTC-A-001` | `agent_a` | high | accepted | Replaced the underspecified global logical-group key with an invocation-local exactly-one rule; explicitly separated discovery/exact outer invocations, single/batch paths, and same-map `FinalAccepted` completion. | `SC-SNOWENERGY-001#revision-61-typed-feed-forward-terminal-carrier-evaluation` | Invocation locality prevents legitimate equal-payload outer work from aliasing and requires no global duplicate registry. |
| `FFTC-A-002` | `agent_a` | high | accepted | Demoted the source assertion to a structural pre-red only, aligned it with the package-owned CQR test file, and made independently runnable in-crate behavior tests plus compile-time negative-capability evidence the implementation acceptance authority. | `contract_ref.md`; `readiness-matrix.md`; `tests/integration/snow_terminal_enthalpy_event_numerics_contract.rs` | Symbol presence is not accepted as behavioral evidence; the pre-red records absence only, while postimplementation behavior must execute at the authentic consumer seam. |
| `FFTC-A-003` | `agent_a` | high | accepted | Reconciled every current-contract revision pin in the integration target from 58 to 61 and required the complete target to have exactly the named structural expected red before production. | `tests/integration/snow_terminal_enthalpy_event_numerics_contract.rs`; `contract_ref.md` | Historical process-version narrative remains unchanged; all 26 assertions of the current contract revision now target 61. |
| `FFTC-A-004` | `agent_a` | medium | accepted | Added a concrete core Guard Map row and source-real ordered validation/error chain using existing `Kernel`, `TurbulentTransfer`, `TerminalNumerics`, and `TerminalCustody` variants; corrected evaluator boundary/result mismatch to `Kernel`; inserted the post-join `stage3_hourly_surface_energy` and required-diagnostics boundary; removed invented generic pre-carrier and duplicate errors. | `SC-SNOWENERGY-001#guard-map` | The scheduling correction neither hoists validators nor invents a solver fallback/error taxonomy. Adjacent competing-poison tests bind provider custody, result join, surface-energy/diagnostics, transition, exact, and final boundaries. |
| `FF-B-001` | `agent_b` | high | accepted | Downgraded the exact 200-group pre-change statement to an explicit static/focused-capture inference. Retained 400 as the only direct exact-release pre-change count and made a post-change exact 200-invocation role/path multiset mandatory. | `SC-SNOWENERGY-001#OBL-SNOWENERGY-C-056`; `contract_ref.md` | The retained 800 joins are intra-map joins, not group evidence. No direct 200-group release claim remains. |
| `FF-B-002` | `agent_b` | high | accepted | Pointed the structural expected-red gate to `cqr_row5_tests.rs`, matched package ownership, and separated the non-authoritative source pre-red from required executable behavior commands. | `tests/integration/snow_terminal_enthalpy_event_numerics_contract.rs`; `readiness-matrix.md` | A correct implementation at the declared write set can now satisfy the structural gate, but only executed behavior tests can satisfy C-056. |
| `FF-B-003` | `agent_b` | high | accepted | Bound the exact pre-change source/binary record and a three-run median keep/revert protocol: at least 750,000 us improvement in both provider-carrier subtotal and total wall time, exact science/count/multiset identity, and 65,536 KiB RSS ceiling. | `SC-SNOWENERGY-001#OBL-SNOWENERGY-C-056`; `package.md`; `readiness-matrix.md` | The prospective absolute ceilings prevent a call-count-only change from being retained when adapter/audit overhead erases material throughput recovery. |

All review findings are dispositioned and amended. Both reviewers accepted the
corrected manifest and both independent verifiers issued implementation `GO`.
The contract-first gate completed and authorized a bounded experiment. The
subsequent three-run keep/revert gate failed its per-run RSS conjunction even
though timings, science, counts, and the exact 200-invocation multiset passed.
The revision-61 production and diagnostic telemetry increment was therefore
rejected and fully reverted; the canonical contract and this review record are
retained as historical design/evidence, not as an implemented runtime claim.

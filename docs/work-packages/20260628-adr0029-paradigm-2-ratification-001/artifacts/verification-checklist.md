# Verification Checklist

Evidence class: Static, with prior Ran package evidence consumed.

## ADR-0028 Admission Authority

| Claim | Result | Evidence |
|---|---|---|
| ADR-0028 defines a bounded observed-data admission tier below derivable science contracts and above comparator/reference flags. | Pass | `docs/decisions/0028-observed-data-admission-authority.md:44-87` defines admissibility conditions and the authority tier; `docs/decisions/0028-observed-data-admission-authority.md:91-99` bounds scope. |
| The snow/frost rubric is the first operational instance. | Pass | ADR-0028 names `GAP-SNOWFREEZE-002` / `INV-SNOWFREEZE-050` and the SNOTEL corpus as the first application at `docs/decisions/0028-observed-data-admission-authority.md:127-134`. |
| `INV-SNOWFREEZE-050` is a forcing-robust rubric and keeps legacy/PySnobal as flags. | Pass | `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md:260` defines the signature/KGE/forcing-robust rubric and comparator flag posture; the rubric addendum defines `R` versus `L` cells at `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md:2675-2725`. |
| 10.3.x snow candidates consume the same observed-data rubric and candidate gates. | Pass | `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md:994-996` gates Harder-Pomeroy, sublimation, and climate-class density on cross-SNOTEL forcing-robust evidence; 10.3.18 consumed `INV-SNOWFREEZE-050` at `docs/work-packages/20260627-snowdensity-10-3-18-cross-snotel-mechanism-rubric-001/package.md:5-25`. |

Conclusion: ADR-0028 is sound and already operationalized. It is ratified before
ADR-0029 because ADR-0029 cites it as admission basis.

## ADR-0029 Load-Bearing Claims

| Claim | Result | Evidence |
|---|---|---|
| DirectFrostLaneState already carries variable-length `Vec` state under the ADR-0026 winter-column exception. | Pass | `crates/openwepp-hillslope-orchestrator/src/winter_column.rs:13-16` owns snow and frost sub-states; `crates/openwepp-hillslope-orchestrator/src/winter_column.rs:162-193` defines `DirectFrostLaneState` with `layer_shadows: Vec<_>` and `fine_layers: Vec<_>`; `docs/decisions/0026-stateful-winter-column-sub-solver.md:35-59` authorizes the stateful winter-column exception. |
| ADR-0025 remains the hot-path constraint, and ADR-0026 is the accepted winter-column home. | Pass | ADR-0025 is Accepted at `docs/decisions/0025-array-native-hillslope-day-frame.md:1-4` and carries workspace/perf gates at `docs/decisions/0025-array-native-hillslope-day-frame.md:61-80`; ADR-0026 is Accepted at `docs/decisions/0026-stateful-winter-column-sub-solver.md:1-5` and localizes snow/frost state authority at `docs/decisions/0026-stateful-winter-column-sub-solver.md:32-41`. |
| Current no-env bulk floor is `15` / `179` and beats legacy `16` / `176`. | Pass | 10.3.18 model summary records legacy `16` / `176` and Harder-Pomeroy partition `15` / `179` at `docs/work-packages/20260627-snowdensity-10-3-18-cross-snotel-mechanism-rubric-001/artifacts/cross-snotel-mechanism-rubric.md:16-25`; 10.3.21 repeats that the current no-env default beats legacy at `docs/work-packages/20260628-snowdensity-10-3-21-post-partition-residual-decomposition-001/artifacts/post-partition-residual-decomposition.md:47-58`. |
| Paradigm 1 was refuted by SNOWDENSITY-10.3.22 gate failure. | Pass | The 10.3.22 package status is `HOLD-GATE-FAILURE-NON-PROMOTION` at `docs/work-packages/20260628-snowdensity-10-3-22-climate-class-density-specialization-001/package.md:1-7`; gate results show candidate `16` / `168` versus default `15` / `179`, failed bidirectional densification flip, and failed persistence guardrail at `docs/work-packages/20260628-snowdensity-10-3-22-climate-class-density-specialization-001/package.md:102-142`. |
| The corpus/high-`rho_max` cluster claim is supported and does not over-claim validation of absent classes. | Pass, static inference | Sturm 2010 parameters put alpine, maritime, and prairie near `rho_max ~= 0.594-0.598`, while tundra/taiga diverge lower, at `docs/work-packages/20260628-snowdensity-10-3-22-climate-class-density-specialization-001/artifacts/authority-gap-and-disposition.md:11-23`. The 10.3.22 package states absent classes remain reference-covered, not rubric-validated, at `docs/work-packages/20260628-snowdensity-10-3-22-climate-class-density-specialization-001/package.md:130-132`. The load-bearing conclusion is the real gate failure, not a per-site class-fit claim. |
| Stage 0 is complete, pure, and unwired. | Pass | Stage 0 package status and disposition are complete at `docs/work-packages/20260628-paradigm-2-stage-0-surface-energy-balance-001/package.md:1-28`; no-production-wiring scan reports no runtime references at `docs/work-packages/20260628-paradigm-2-stage-0-surface-energy-balance-001/artifacts/no-production-wiring-scan.md:1-25`; verification gates passed at `docs/work-packages/20260628-paradigm-2-stage-0-surface-energy-balance-001/artifacts/verification.md:1-34`. |
| ADR-0029's staged Paradigm 2 scope is consistent with the specification. | Pass | The specification frames multilayer snow as shared foundation for frost, winter water temperature, and runoff at `docs/planning/paradigm2-multilayer-snow-specification.md:14-32`; de-risks variable-layer state at `docs/planning/paradigm2-multilayer-snow-specification.md:43-68`; defines Stage 0-3 at `docs/planning/paradigm2-multilayer-snow-specification.md:120-134`. |

Conclusion: ADR-0029's load-bearing claims are verified. No HOLD condition was
found.

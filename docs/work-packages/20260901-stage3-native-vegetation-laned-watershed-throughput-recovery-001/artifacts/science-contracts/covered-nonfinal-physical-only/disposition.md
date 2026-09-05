# Covered nonfinal physical-only review disposition

Status: `DUAL VERIFIED — IMPLEMENTATION AUTHORIZED`

Evidence mode: `Static`

| finding_id | source | severity | decision | action_taken | artifact_ref | rationale |
|---|---|---|---|---|---|---|
| `CPH-A-001` | agent_a | high | accepted | Separate private final-envelope construction from zero map-level publication and one accepted-parent publication in every affected contract and counter matrix. | canonical contracts + readiness matrix | Existing parent-atomic publication authority controls; candidate maps cannot publish. |
| `CPH-A-002` | agent_a | high | accepted | Add explicit success, pre-final failure, final-physical failure, and final-constructor failure accounting. | canonical contracts + behavioral tests | Validated endpoints cannot be inferred from charged attempts. |
| `CPH-A-003` | agent_a | high | accepted | Add missing `P-005`, executable counters/differential/privacy/poison/rollback tests, and retain source check only as anti-evasion. | contract-derived tests | Token presence cannot prove consumer behavior. |
| `CPH-A-004` | agent_a | medium | accepted | Make ordinary/native regime dispatch orthogonal to nonfinal/final role dispatch. | LSE/Snow/Surface contracts | Native represented snow must execute zero litter/WB14 physics. |
| `CPH-A-005` | agent_a | medium | accepted | Map new triggers to existing semantic typed errors and deterministic existing precedence; remove `ERR-CT-021`. | contract guard tables | Event-boundary error authority is inapplicable to role/custody faults. |
| `CRB-CNFP-001` | agent_b | high | accepted | Same action as `CPH-A-003`; add behavioral expected-red gates before production edits. | contract-derived tests | Independent review confirms evasion risk. |
| `CRB-CNFP-002` | agent_b | high | accepted | Specify exact zero-based role/ordinal bijection and contiguous trial sequence for both regimes. | Snow/CoupledTime contracts | Reproducible accounting must not be producer-defined. |
| `CRB-CNFP-003` | agent_b | high | accepted | Remove `INV-SURFACELIQUID-033` from `INV-034` authority and explicitly prohibit reuse in canonical covered solves. | SurfaceLiquid contract | Snow-free final-receipt reseal and covered final-map physics have different authority. |
| `CRB-CNFP-004` | agent_b | high | accepted | Remove undefined `E-012` from the amendment and bind identity/support, physical closure, envelope, state-leak, and publication triggers to defined errors. | Surface/Snow/CoupledTime contracts | Error payloads and precedence must be canonical. |
| `CRB-CNFP-005` | agent_b | medium | accepted | Same parent-only publication correction as `CPH-A-001`. | canonical contracts + tests | Direct/rejected candidates must expose no publication. |
| `CRB-CNFP-006` | agent_b | medium | accepted | Add a five-contract readiness/profile matrix with explicit evidence/test bindings. | `readiness-matrix.md` | Lints alone do not establish kernel-profile completeness. |
| `CRB-CNFP-007` | agent_b | medium | accepted | Reconcile registry/contract dates, trailing change logs, and monotonic Snow contract revision while naming it metadata rather than a process-solver successor. | contracts + index | Contract schema and ADR-0044 govern distinct version identities. |
| `CPH-A-002-V1` | verification_agent_a | high | accepted | Assert the exact pre-final, post-validation nonfinal, final-physical, and final-constructor charge/endpoint/constructor/completion matrices. | behavioral expected-red test | Zero publication and rollback alone did not prove result-sensitive counts. |
| `CPH-A-003-V1` | verification_agent_a | high | accepted | Add per-constructor counters, zero native snow-free litter/surface/WB14 calls, retained inactive bytes, all-regime poison coverage, and derived/manual serialization plus `Clone`/`Copy` privacy guards. | behavioral + source expected-red tests | The prior population admitted constructor, native-work, and move-only evasions. |
| `CPH-A-005-V1` | verification_agent_a | high | accepted | Assert exact typed error classes and validation stages, including combined-fault precedence. | behavioral expected-red test | `typed_error.is_some()` did not bind class or precedence. |
| `CRB-CNFP-001-V1` | verification_agent_b | high | accepted | Expose the typed multisecant trial index independently from ordinal; record exact injected failure class/stage and zero alternate-envelope fallback or successful physical-only promotion. | behavioral expected-red test | Generic role text and arbitrary panic acceptance were insufficient. |
| `VB-CNFP-001` | verification_agent_b | high | accepted | Remap role/ordinal to `DirectV11RealConsumerError::AdaptiveRefinement`, partial-owner/rollback defects to `VEGTXN-E-007`, per-map/atomic finalization to `E-013`, restart/replay to `E-014`, and complete shared-owner mismatch to `E-015`. | `SC-VEGETATIONTRANSACTION-001.md` + readiness matrix | Existing semantic error authority must not be redefined by the amendment. |
| `CPH-A-RV2-001` | repeat_verification_agent_a | medium | accepted | Replace tracked-diff hashing with a deterministic ordered full-file manifest over all eight reviewed paths, including the untracked integration test. | `contract_ref.md` | Verification custody must bind every reviewed byte regardless of git index state. |
| `VB-CNFP-002` | repeat_verification_agent_b | high | accepted | Bind `charged_roles.len()` exactly to the charged-map attempt count before checking typed `MultisecantTrial(n)@(n+1)` entries. | behavioral expected-red test | Iterating only the observed slice could admit omitted role records. |

The first verification-scoped diff
`5e1c303754aa7c4ef0ccab43560bebec867d779fad5113f6d10258a581ff440a`
received dual `FAIL` verdicts, recorded in `verification_agent_a.md` and
`verification_agent_b.md`. All verification findings are accepted and amended.
The current deterministic full-file repeat-verification manifest is
`ea628948b21522359c52a788a7e065f4f1d2428f8c2ff59fe1642bd282ee4b44`.
No finding is rejected, deferred, or assigned to follow-up. Repeat dual
verification closed with `PASS` and `PASS-WITH-NOTES`; the sole note,
finding-level traceability for `VB-CNFP-002`, is recorded above. Production
implementation of this amendment is authorized. Runtime conformance remains
unproven until the expected-red population and applicable gates pass.

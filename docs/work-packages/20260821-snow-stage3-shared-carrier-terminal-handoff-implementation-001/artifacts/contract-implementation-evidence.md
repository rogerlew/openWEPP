# Contract Implementation Evidence

Status: `EXECUTED HOLD / TYPED OPT-IN OWNER EXECUTION AND IDENTITY-RESTART PROVED; RELEASE HOLD`

Static: the new implementation binds the following current source locations:

| Contract | Implementation location | Evidence class |
| --- | --- | --- |
| `SC-SNOWENERGY-001@14` | `snow_stage3_terminal_handoff.rs::evaluate_shared_carrier` | Shared-air weighted node, sealed exposure, reciprocal LW, independent snow/liquid/energy checks. |
| `SC-COUPLEDTIME-001@3` | `locate_terminal_event`, `CoupledTimeError::EventBoundaryNoCandidate` | Canonical tick/support/tolerance selection and typed retry error. |
| `SC-LANDSURFACEENERGY-001@7` | `SnowStage3HandoffRuntime::stage` | Half-open remainder, zero-duration custody, minimum post-event support, snow-operand rejection. |
| `SC-VEGETATION-001@26` | `CompleteOwnerSet` | Released seven-owner manifest and byte digest. |
| `SC-VEGETATIONTRANSACTION-001@15` | `SnowStage3HandoffRuntime::stage/commit_pending` | Clone/stage/commit ordering and exact terminal-liquid debit-credit join. |
| `SC-VEGETATION-001` / `SC-VEGETATIONTRANSACTION-001` | `DirectV11SnowStage3OwnerExecutor` and `DirectV11RealConsumerStack` | Actual accepted-slab V11 execution produces the ending V11/LSE/BGC/soil-thermal owner envelopes before handoff commit. |

Static: the direct scheduler hook is
`DirectFrameExecutor::run_publication_stream_with_snow_stage3_terminal_handoff`
and the canonical restart seam is `SnowStage3HandoffRestartV1`.

Static: the event receipt binds parent identity, segment identity, event
ordinal, candidate-set digest, and accepted tie rank. Runtime restart admission
also requires contiguous event ordinals, complete receipt history, matching
receipt-chain digests, and body-consistent receipt digests.

Ran: the independent Python authority oracle completed 17 carrier/event/
conservation cases and 3 restart/rollback cases; all expected accepted/rejected
outcomes and equivalence/no-op results matched. Inside `nix develop`,
`cargo fmt --all -- --check` and `cargo check --workspace` passed. The focused
package integration target latest run passed 6/6 tests. The combined authority-contract
selection passed 17/18 tests; the one failure is an unchanged registry-string
guard requiring a v13 lifecycle phrase in
`docs/specifications/science-contracts/index.md`. The current critical frost
final frost profile ran 391 tests: 390 passed and 1 failed outside this write
set — the unchanged `SC-SNOWENERGY-001.md` `contract_version: 13` guard.

Ran inside `nix develop`: the focused typed endpoint test
`child2c_scheduler_commits_the_concrete_v11_lse_bgc_soil_owner_candidate`
passed, as did the existing V11 full-support test and the package integration
target (6/6). Strict workspace Clippy is clean for the follow-on files; the
workspace `-D warnings` run still stops on unrelated pre-existing test lints in
`surface_liquid_wb14.rs` and authority-contract integration tests.

The typed opt-in endpoint is real, but the ordinary hillslope runner still does
not construct or invoke it. The new participant join and restart identity
guards are bounded increment evidence; physical carrier derivation, terminal
liquid custody into the real surface-liquid owner, durable publication
atomicity, mandatory failure scenarios, and authority-document repairs remain
release blockers. The normal runner lacks the typed owner-input authority
needed for a valid binding.

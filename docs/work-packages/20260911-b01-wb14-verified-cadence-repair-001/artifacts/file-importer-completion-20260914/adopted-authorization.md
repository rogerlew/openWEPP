# B01-WB14-CURRENT-CONTEXT-CAPTURE — file-consumer completion

**DRAFT FOR OWNER ADOPTION. This document grants no execution authority until adopted.**

## One active checkpoint and explicit amendment

**Checkpoint:** continue the SAME `B01-WB14-CURRENT-CONTEXT-CAPTURE` in `docs/work-packages/20260911-b01-wb14-verified-cadence-repair-001/package.md`. Do not create another package. The parent-cadence candidate stays suspended and NOT ACCEPTED. The completed witness and test-only regression remain closed.

**Exact evidence revision:** `6dcb16fed8023e340d8d8f317ac778dfd54926bc` in `rogerlew/openWEPP`.

**Actual source:** `/workdir/openwepp-experiments/b01-wb14-cadence/current-context-capture-20260913`, identified by accepted `baseline-red` plus `artifacts/importer-continuation-20260913/terminal-source.patch` in the owning package. Patch SHA-256: `66c7ab9d3241ce70f1fa90a867a403cc5d12d6cf30326710261e1c445ffc23c0`. Use the five exact file identities and successful reconstruction record in `terminal-source-recovery.json`; do not identify this source by the immutable available145 map alone. Current capture-module SHA-256: `3496d7ebc70620f7ac1bd1ad8d4c7b6c7b156ef6fbc29ea7c89c648856ba5820`. Current test-module SHA-256: `3ffce676c80e312e9b61c01f6b5002383f188d3e04900ca9f0815812ed6d536e`. The earlier passing test cuts are not this source. The rejected cadence patch remains excluded.

**Authorized changes:** finish the existing private diagnostic recorder, complete file consumer, immediately necessary private producer/attachment plumbing, focused tests, and scoped evidence/record. Wire and correct the retained restoration helpers instead of accumulating another unused implementation. Existing public APIs, canonical wire/restart schemas, physics, admission rules, provider decisions, forcing, features, dependency inputs, protected checkpoints, accepted baseline and historical captures remain read-only. Minimal private diagnostic support in the existing runner/test harness remains permitted where needed for the already authorized file probe.

**Acceptance:** a source-bound recorder/importer path that records and canonically reconstructs the actual provider-bound prepared day, consumer, current parent/clock/history, pending custody, and pre-child provisional context; complete required small positive/negative and observation-only tests; source-quality/lint attribution; independent correctness and QA; then the original baseline capture and fresh-process zero-physics probe with their full evidence. Compilation or helper-only success is not importer acceptance. All existing capture requirements remain binding.

**Run allowance:** preserve exactly ONE total baseline capture/collector invocation and ONE total authentic-bundle fresh-process zero-physics probe, both currently `0/1`. No additional runtime allowances, retries, suffix simulations, captured-day reruns, cadence repair, or protected-checkpoint operations. The old witness remains exhausted `1/1`. Existing focused builds/listings/unit tests and matched lint checks remain permitted within the budget. Local pre-capture file-fixture tests do not run the authentic simulation and must not disguise an authentic-bundle probe or model invocation.

**Proposed budget amendment on adoption:** raise the cumulative active-time ceiling from 180 to **240 minutes**, and the cumulative unsuccessful-correction ceiling from nine to **ten**. Carry **180m05s consumed and eight unsuccessful corrections**, including the recorded five-second preservation overrun. This leaves **59m55s and at most two additional unsuccessful corrections**, whichever expires first. Do not erase historical consumption or transfer the balance into another checkpoint. Reading, edits, orchestration, review and preservation count; concurrent wall time counts once. Exclude only recorded pure waits. Budget for final preservation inside the allowance.

**Stopping conditions:** either amended hard limit; owner stop; unexplained source/input drift; necessary unavailable source or authoritative inputs; required public/schema/physics/dependency change; or the first new downstream setup, physical, canonical-restoration, or finalization failure. Return its actual operands and source location rather than repairing another subsystem. Ordinary syntax/type/integration defects within this implementation count as failed corrective verifications and return to Astra before another edit; they do not silently renew the allowance. The known baseline E008 remains the planned capture result.

## Finish the connected path

The concrete deliverable is a file-consuming entrypoint. It may take a file path and explicitly pinned, independently supplied configuration/authority inputs. It must not require a pre-restored live `DirectV10RealConsumerShadow`, prepared day, parent, or clock from the process that produced the file. Reconstruct those objects from the recorded bytes and legitimate configuration inputs through the existing canonical paths.

Work backward from this entrypoint through the existing implementation. Complete this connected flow:

`recorded file -> provider/static configuration -> restored provider day -> all 48 reconstructed supports and canonical bound day -> consumer/current parent/clock/history/custody -> derived current forcing -> provisional constructor on an exact clock clone -> typed restored context and verification result`.

This is a private diagnostic interface, not a new canonical restart format. A typed returned context or equivalent owned fixture object must be usable by the probe; a function that merely compares a row to live caller objects and returns `Ok(())` is insufficient. Process-local capabilities are minted afresh after canonical validation, never serialized or resurrected.

On the producer side, pass the actual validated provider/prepared day through the immediate private path into `PreChildContext`, and emit the operands consumed by the file reader. The current terminal hook only carries one support and does not supply the new day restoration helpers. Reuse the discovered in-scope producer/attachment paths; do not repeat the already-resolved feasibility investigation or require prerecorded successful children 2–18.

A complete prepared day requires exact ordered membership and coverage, not merely 48 entries that validate individually. Include duplicate-valid-support, omitted/replaced-support, reordered-support, and wrong-day controls. Capture actual constructor operands plus canonical projections; do not treat a projection or digest as its own inverse. For manual enum/scalar mappings, prove exact round-trip correspondence with genuine provider-bound objects, including source-defined floating-point bits and ordering.

Derive `canonical_parent_forcing_digest` through the restored provider/prepared-support path. The row's forcing receipt is a comparison operand, not independent authority. Restore the actual consumer from recorded native owners, accepted history, configuration and provider state. Join consumer canonical projections to their corresponding recorded representations, and parent-staged projections to the clock through the source-defined joins. Do not force unlike consumer and clock encodings to be byte-identical. Require exact owner membership, current cursor/segment/prefix relationships and validation of ProducedUnconsumed terminal-parcel custody; do not consume parcels during import.

Retain B01-CONT-015/016 and both current reviewers' detailed findings as the finite unfinished inventory. Do not discard an unmet requirement because one part of the round trip succeeds.

## Known mechanical corrections and validation sequence

The terminal test already uses `let mut missing_lse = row.clone()`. Preserve the historical E0382 failure; verify the final fix rather than repeating an obsolete edit. The fully qualified exact selector was also corrected on earlier cuts; still confirm nonzero selection on the new final source.

**B01-CONT-017 — static compiler defect:** `provider_interval_for_destination` borrows both `provider` and `destination`, but its returned receipt has no explicit lifetime. Bind that returned reference to the provider, not the destination:

```rust
fn provider_interval_for_destination<'a>(
    provider: &'a crate::runtime_inputs::PreparedSnowFreeGsiDayV1,
    support: TimeSupport,
    destination: &(
        openwepp_land_surface_energy::OfeId,
        openwepp_kernel_contract::TileId,
    ),
) -> Result<&'a crate::runtime_inputs::SnowFreeHalfHourIntervalReceipt, String>
```

This is a proposed signature correction, not an executed build result. Retain the body unless a demonstrated defect requires an in-scope correction.

**Replace the previous instruction to postpone every verification until all code is composed.** Cheap syntax/type checks and focused developmental tests are permitted during integration. Preserve each exact cut/result and count every unintended failed corrective verification, including compilation, against the same hard limit. A helper pass is development feedback only; it does not replace final connected-path evidence. Do not repeatedly dispatch the same narrow helper as the deliverable.

Before any authentic capture, run a legitimate small file-backed composed test using the production recorder/consumer path, not a second test-only decoder. Construct valid provider/parent/owner inputs through existing APIs, write the complete bundle to disk, and invoke the actual file consumer without passing the producing process's live consumer/context objects. Preserve canonical expected outputs independently before restoration. Verify the real producer's reduction, ledger, receipt and binding, not values manufactured and checked solely by `reconstruct_provisional_on_clone` on both sides.

Keep the prescribed negative and observation-only checks: provider/forcing/static-context substitutions; exact 48-support membership; owner/parent/prefix/parcel changes; wrong run/topology/parent/child target; missing and duplicate/conflicting capture; disabled/non-target no-work behavior; diagnostic failure isolation; and zero constitutive/ingress/WB14 entry during import. Match each poison to its intended predicate rather than an earlier unrelated parse or checksum failure. Record actual case names and counts. Test fixture runs are not the authentic day-4 evidence.

Use existing Nix, explicit diagnostic manifests/targets, retained feature selections and stack settings. Retain the prospectively selected matched orchestrator and separate runner lint comparisons; do not repeat unchanged baseline lint or accepted baseline/witness tests without a relevant change. All original Clippy failures remain failures. The no-new-relevant-diagnostics policy applies only while independent review confirms observational noncritical scope; no production lint exception is granted.

After final source, focused behavior, formatting, matched lint and both prelaunch scopes are accepted, the original single capture and single fresh-process probe may run within the remaining allowance. Freeze actual source/binary/input/command identities before launch and retain start/end, numeric exit and full logs. Preserve the original B01 case, forcing, timeout and E008 comparison. Any unexpected result is retained without retry. A usable bundle and successful zero-physics probe do not authorize a cadence repair or establish production restart equivalence.

## Execution ownership, review and return

Astra owns integration and source custody; keep one actual writer. Reassign a stalled writer using supported session/slot management if needed, revoking prior write custody first. Do not repeat capacity-failing spawns, create nested agents, or add a planning/reviewer pipeline. Repository role defaults remain applicable. Reuse `/root/capture_importer_correctness` and `/root/capture_importer_qa` for affected fix verification when available. On adoption, one explicitly recorded replacement per genuinely unavailable required scope is permitted, carrying original findings and accepted fixes without claiming conversation continuity. Required correctness and QA conversations remain distinct and independent of author/adviser/implementer.

No additional general design package or full history reread is required. Use the current source, unfinished findings and actual call path. A new necessary scope or authority gap must be demonstrated with the exact caller/API/operand, not inferred from implementation difficulty or a missing convenience decoder.

Keep one maintained `package.md`; record the adopted amendment there. Carry existing scoped commit/push permission on the current branch, preserve unrelated work and all failed cuts, and publish only necessary small source patches/evidence with the already required remote byte verification. Do not duplicate the entire source tree or raw corpus, clean up unique state, or infer remote source recoverability from a hash.

Return separate outcomes for connected importer delivery, focused controls, source quality, independent scopes, authentic capture and fresh-process probe. A completed local implementation milestone is not completion of the capture checkpoint while actual capture/probe remain unperformed. The wider cadence and B01-CONT-005/013/014 obligations stay HOLD. Report exact consumed budget and any overrun without normalization.

## Pinned evidence

Repository: `rogerlew/openWEPP`, revision `6dcb16fed8023e340d8d8f317ac778dfd54926bc`.

- `docs/standards/chatgpt-pro-role.md`, root `AGENTS.md`, `docs/work-packages/AGENTS.md`, and `docs/standards/bounded-agent-execution.md`.
- Owning package `package.md`, section “Importer continuation adopted”.
- `artifacts/importer-continuation-20260913/terminal-source-recovery.json` and `terminal-source.patch`.
- `artifacts/importer-continuation-20260913/correctness-final-review.txt` and `qa-final-review.txt`.
- Applicable unchanged SC-COUPLEDTIME-001, SC-SURFACELIQUID-001 and SC-SNOWENERGY-001 requirements named in the adopted capture scope. In particular, preserve complete chronology, exact owner relationships, fresh live capabilities after restore, and no physical work during import.

Immutable record: https://github.com/rogerlew/openWEPP/blob/6dcb16fed8023e340d8d8f317ac778dfd54926bc/docs/work-packages/20260911-b01-wb14-verified-cadence-repair-001/package.md

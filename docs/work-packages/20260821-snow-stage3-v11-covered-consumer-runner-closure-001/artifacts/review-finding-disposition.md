# Review finding disposition

Static: User review of `8712e272a500ad385051fc2f33d1ae518d4c55ba` is the
first independent checkpoint review. It accepts the corrected keyed candidate
feedback topology while retaining package `EXECUTING / HOLD`.

| Finding | Severity | Decision | Current action/status |
|---|---|---|---|
| Component-authoritative canopy carrier | Critical | accepted | Closed for the covered checkpoint: final LSE output now retains sunlit leaf, shaded leaf, wet canopy, and stem/WAI area, emissive area, conductance, temperature, humidity, sensible flux, and vapor flux. Validation independently reconstructs component flux sums and emissive-area longwave, and the final carrier cross-joins canopy, reference-atmosphere, and Stage 3 snow fluxes. |
| Final receipts do not bind installed V11 and complete snow owners | Critical | accepted | Closed for the covered checkpoint: physical boundary receipts remain noncircular, while `CoveredParentOwnerJoinReceiptV1` is constructed after finalization and canonically binds the actual installed vegetation, complete snow, LSE, hydrology, BGC, soil-thermal, and surface-liquid envelopes plus the Stage 3 physical-state and final boundary/component receipt sets. |
| Stage 3 lane area basis | High | accepted | Option A admitted prospectively in `SC-SNOWENERGY-001@15`. Runtime alternate basis removed; complete OFE-ground tile closure and no-renormalization guards implemented. Mixed open/covered execution now remains fail-closed until the open-snow producer exists. |
| Mixed-unit Stage 3 fixed-point tolerance | High | accepted | Open; replace with per-field units and exact categorical/count comparisons, then obtain independent numerics/science review. |
| Predeclared outcome ledger enters physical solve | High | accepted | Open; separate carrier flux input from postcandidate independently reconstructed outcome ledger before precipitation. |
| 5,608-line active implementation module | Closure blocker | accepted | Closed mechanically in the next increment: covered execution moved to `v11_covered/mod.rs` and owner finalization to `v11_covered/owner_finalization.rs`; every active file is below 3,000 lines. Pre/post orchestrator baselines are recorded in `gate-results.md`. |
| JSON lane receipt identity | Restart blocker | accepted | JSON was removed. v15 now normatively defines the deterministic adopter-specific `OPENWEPP_LANE_STAGE3_BOUNDARY_RECEIPT_V1` wire and explicitly prohibits its use as coupled parent/restart identity. Migration to the canonical framed helper/domain remains mandatory before additive restart. |

Historical checkpoint note: no Rust test or `rustfmt` result was available for
the original `8712e272` amendment. That limitation was superseded by the v15
promotion qualification and the later mechanical-split qualification recorded
in `gate-results.md`. Package disposition remains `EXECUTING / HOLD`.

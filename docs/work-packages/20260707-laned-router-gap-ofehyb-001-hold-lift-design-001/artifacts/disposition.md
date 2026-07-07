# Disposition

Status: **EXECUTED-COMPLETE**. Evidence mode: **Static + Ran**.

## Decision

`GAP-OFEHYB-001` is closed as the Case-4 hybrid ladder subgate.

The accepted rule is the `SC-OFEROUTE-002` rev-3 source-memory cooldown:
source-active bins route explicitly; after a contiguous source-active burst,
the next `2 * burst_duration` source-free bins remain explicit; later
source-free bins are implicit-eligible. Upstream inflow alone still does not
force explicit routing after cooldown because the implicit step books the
interval-mean upstream mass exactly.

## Review Disposition

| Finding | Disposition |
|---|---|
| Case-4 harness duplicated the production cooldown constant. | Fixed by sharing `HYBRID_SOURCE_MEMORY_COOLDOWN_MULTIPLIER` from `cascade.rs` into `dval.rs`. |
| Contract had stale “ignored while failing” wording. | Fixed in Test-Vector, Gap, timing, and revision-history rows. |
| Gate table and disposition artifacts were incomplete. | Fixed by completing review, verification, gate, final disposition, and handoff artifacts. |
| Anti-evasion gates were missing. | Fixed and run. |
| H2637 evidence was not reconciled. | Fixed with a final release-binary H2637 run and updated timing artifact. |
| Copied stale H2637 scratch logs created provenance ambiguity. | Removed stale copied logs; retained current timing log and output hashes. |
| Multi-burst reset semantics lacked a retained vector. | Added `hybrid_source_memory_resets_on_later_source_burst`. |
| Old rev-30/rev-31 wording remained near source-memory paths. | Renamed comments/test wording to rev-33 source-memory language. |

## Non-Promotion Statement

This package does not promote the hybrid selector and does not change default
behavior. `OPENWEPP_LANED_ACTIVE_IMPLICIT=1` remains experimental/unpromoted.
`INV-OFEHYB-008` remains the promotion gate; `GAP-OFEHYB-002` remains open for
solve-cost optimization and broader fidelity/timing ratification.

The package is active.

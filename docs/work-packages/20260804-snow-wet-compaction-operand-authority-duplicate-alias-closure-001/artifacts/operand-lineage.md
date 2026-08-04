# Wet-Compaction Operand Lineage

Status: queued

Evidence mode: not-run

Populate before production edits. For each operand record units, daily/hourly
stage, normalization denominator, whether it is generated/retained/released,
source authority, and authoritative versus diagnostic status.

Required deliberately non-aliasing candidates:

| Candidate | Formula | Status before adjudication |
| --- | --- | --- |
| Current | `snowpack_state_loss + routed_melt` | rejected duplicate candidate |
| Routed-only | `routed_melt` | unresolved incomplete candidate |
| Loss plus all rain | `snowpack_state_loss + rain_retained + rain_released` | unresolved candidate |
| Generated liquid | `sum(max(hourly_generated_melt, 0)) + snow_contact_rain` | primary-source candidate |
| Store | retained-liquid level or delta | rejected as an unproven substitute unless authority says otherwise |

Acceptance must reconstruct the chosen formula from independently produced
hourly components and show every rejected candidate is numerically distinct.

# PL08 Comparator Confidence-Tier Disposition

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Tier policy requires unresolved Tier-A deltas to remain blocking.
- Tier-B semantics are not used to down-classify Tier-A blockers.

Ran:
- Executed single-OFE Tier-A comparator lane for `H5.wat.dat` and `H5.plot.dat`.
- Collected keyed shared-field parity evidence for plant/residue-relevant fields.

## Disposition Records

| tier | surface_id | delta_signature | first_divergence_surface | first_divergence_timestep | investigation_owner | decision | evidence_mode |
|---|---|---|---|---|---|---|---|
| `Tier-A` | `single-ofe.daily-water-balance.H5.wat.dat` | `structure_diff; line_count_mismatch=5; numeric_arity_mismatch_lines=1096; max_abs=360; max_rel=1` | `H5.wat.dat` | `line=18` | `PL08` | `block` | `Ran` |
| `Tier-A` | `single-ofe.daily-water-balance.shared-20-field-keyed-alignment` | `1095/1095 keyed rows identical in shared columns; Ep/Es/Er exact match` | `H5.wat.dat` | `OFE/J/Y keyed rows` | `PL08` | `investigate` | `Ran` |
| `Tier-A` | `single-ofe.daily-water-balance.openwepp-vs-legacy` | `openwepp comparator-ready Tier-A output surface unavailable in current workspace` | `N/A` | `N/A` | `PL08` | `block` | `Static` |

## Blocking Rationale

1. Tier-A strict comparator still reports unresolved structural mismatch on `H5.wat.dat`.
2. Positive shared-field keyed signal is surrogate evidence and not a replacement for full Tier-A comparator closure.
3. OpenWEPP-vs-legacy Tier-A candidate output is not available, so PL08 cannot issue acceptance-direction closure.

## Disposition

`HOLD` (Tier-A blocker remains unresolved).

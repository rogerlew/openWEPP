# Comparator Confidence-Tier Disposition (SR07)

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Tier policy requires unresolved Tier-A deltas to remain blocking.
- No silent down-classification is allowed for Tier-A blockers.

Ran:
- Tier-A comparator lane executed on single-OFE daily water-balance surface `H5.wat.dat` and produced material structural deltas.

## Disposition Records

| tier | surface_id | delta_signature | first_divergence_surface | first_divergence_timestep | investigation_owner | decision | evidence_mode |
|---|---|---|---|---|---|---|---|
| `Tier-A` | `single-ofe.daily-water-balance.H5.wat.dat` | `structure_diff; line_count_mismatch=5; numeric_arity_mismatch_lines=1096; max_abs=360; max_rel=1` | `H5.wat.dat` | `OFE=1, J=1, Y=1` | `SR07` | `block` | `Ran` |
| `Tier-A` | `single-ofe.daily-water-balance.openwepp-vs-legacy` | `openwepp candidate surface unavailable in current workspace` | `N/A` | `N/A` | `SR07` | `block` | `Static` |

## Blocking Rationale

1. Executed lane shows early structural mismatch on a Tier-A surface, so this lane is non-promotable without explicit disposition.
2. SR07 objective requires validating semantic-parity direction after SR06. That validation is incomplete because no openWEPP daily-water-balance candidate output surface is available in this workspace.
3. Per tier policy, unresolved Tier-A comparator blockers keep package disposition in `HOLD`.

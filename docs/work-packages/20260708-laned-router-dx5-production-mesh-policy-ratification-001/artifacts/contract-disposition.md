# Contract Disposition

Status: `EXECUTED-COMPLETE-CONTRACT-REV45`
Evidence mode: Static + Ran.

## Decision

`SC-OFEROUTE-001` rev 45 promotes the active production mesh default from
fixed `10 cells/OFE` to target `dx = 5.0 m`.

The default active policy now derives:

- `raw_cells = ceil(slplen_m / 5.0)`;
- `cells_per_ofe = max(raw_cells, 10)`;
- `max_cells = 4096`;
- `LANED_ACTIVE_SAMPLE_DT_S = 900`;
- `LANED_ACTIVE_MAX_DT_S = 300`.

The diagnostic selector `OPENWEPP_LANED_ACTIVE_MESH_TARGET_DX_M` remains an
explicit override and does not become the production mechanism.

## Authority Basis

The package-local promotion matrix replayed the current evidence under:

- rev 43 coupled space-time mesh-policy authority;
- rev 44 annual pass-sediment material-year plus annual-vector authority.

Result: `artifacts/rev44-promotion-matrix.md` records
`DX5_PRODUCTION_RATIFIED_BY_EVIDENCE`, `21` rows adjudicated, `0` gate-class
blockers, and `0` missing annual replay rows.

The fixed-300 `mn_corn_h4` fine-reference shape row remains report-only
sensitivity evidence under rev 43 because the same-pair shared-`dt75` spatial
adequacy and same-`dx` timestep controls are present and passing.

## Non-Changes

No routed-shape tolerance changed.

No annual sediment threshold changed.

No sediment process physics changed.

No active max-`dt` production default changed.

No hybrid or implicit stepper code is revived.

Shadow mesh policy remains separate and unchanged: `LANED_SHADOW_CELLS = 10`.

## Runtime Proof Hook

`artifacts/default-dx5-evidence.md` proves the contract amendment reached the
production runtime path: for all three selected real-cohort members,
active no-env runs serialize `mesh_policy.mode = target_dx`,
`target_dx_m = 5.0`, `min_cells = 10`, `max_cells = 4096`, and
`max_dt_s = 300.0`.

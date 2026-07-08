# Default Activation Evidence

Status: `COMPLETE`
Evidence mode: Ran.

## Commands

Focused selector integration:

```bash
cargo nextest run --workspace --profile full --test laned_shadow_h2637
```

Result: `8` tests passed, `2` ignored/skipped, wall `38.125s`.

Full H2637 conditional-default acceptance vector:

```bash
cargo nextest run --workspace --profile full --test laned_shadow_h2637 \
  h2637_native_active_owner_routes_and_closes --run-ignored ignored-only
```

Result: `1` ignored acceptance test passed, wall `563.620s`.

## All-Coefficient Default Active

Fixture: H2637 with native cropland `routing_coefficients` inserted for all
scheduled lanes.

Default/no-env output hashes:

| Surface | SHA-256 |
|---|---|
| `H2637.hbp` | `28897e3517c0e5f210c28f3b31c48106338e9cbf24263fbc7c0f44021aeb5dd5` |
| `H2637.pass.parquet` | `bbb0a9d60540d6556711ae1343feeaa7a47242711aa7d3e52f3d8fde93f961cc` |

Explicit-active output hashes:

| Surface | SHA-256 |
|---|---|
| `H2637.hbp` | `28897e3517c0e5f210c28f3b31c48106338e9cbf24263fbc7c0f44021aeb5dd5` |
| `H2637.pass.parquet` | `bbb0a9d60540d6556711ae1343feeaa7a47242711aa7d3e52f3d8fde93f961cc` |

Disposition: default active is byte-identical to explicit active on protected
outputs for the coefficient-complete fixture.

Active manifest counters from default/no-env and explicit-active runs:

| Metric | Value |
|---|---:|
| `mesh_policy.mode` | `target_dx` |
| `mesh_policy.target_dx_m` | `5.0` |
| `mesh_policy.min_cells` | `10` |
| `mesh_policy.max_cells` | `4096` |
| `max_dt_s` | `300.0` |
| `days_seen` | `731` |
| `days_routed` | `610` |
| `days_uniform_shape` | `3` |
| `total_source_m3` | `374423.35262127215` |
| `total_routed_outlet_m3` | `371256.0302455183` |
| `total_end_window_storage_m3` | `3167.322375757055` |
| `total_clamp_m3` | `8.558695924465149e-14` |
| `total_tail_fold_m3` | `36426.08442024077` |
| `total_latqcc_outlet_m3` | `208132.8460294917` |
| `max_supply_reconstruction_rel` | `7.31201193525081e-16` |
| `max_day_cascade_residual_rel` | `2.2762831518726353e-13` |
| `max_day_seam_residual_rel` | `5.0415846159888125e-14` |
| `max_day_identity_residual_rel` | `2.1906143827108124e-13` |
| `lane_days_erosion_source_shape_degenerate` | `1` |

## No-Coefficient Fallback

Fixture: legacy H2637 management with no native `routing_coefficients`.

Default/no-env output hashes from focused integration:

| Surface | SHA-256 |
|---|---|
| `H2637.hbp` | `453e441cf065544fccb41737145ed228625cf9386423671e2a63887ecf0c072f` |
| `H2637.pass.parquet` | `fd47c341b300860de7007164ad0797cec4f70985958869eff829daf75a20966a` |

Manifest disposition: `laned_active` block absent.

## Explicit Disable On Coefficient-Complete Fixture

Fixture: H2637 with all native `routing_coefficients`, plus
`OPENWEPP_LANED_ACTIVE_DISABLE=1`.

Output hashes:

| Surface | SHA-256 |
|---|---|
| `H2637.hbp` | `948faf82c7edc2a60177b9567a92d8e6999f2d95e1d6f13953fda48b492c0467` |
| `H2637.pass.parquet` | `f0d1be11ee9f24b407479a7cdad7e3229981c49b7d2cadb179d4f0a74027e2a3` |

Manifest disposition: `laned_active` block absent.

## Fail-Closed Cases

Covered by focused integration test pass:

- explicit `OPENWEPP_LANED_ACTIVE=1` with missing routing coefficients fails
  closed;
- `OPENWEPP_LANED_ACTIVE=1` plus `OPENWEPP_LANED_SHADOW=1` fails closed;
- `OPENWEPP_LANED_ACTIVE=1` plus `OPENWEPP_LANED_ACTIVE_DISABLE=1` fails
  closed;
- mixed routing-coefficient authority fails closed under default/no-env;
- malformed routing-coefficient authority fails closed before fallback.

The mixed-authority error includes the `conditional Lane D default activation`
surface and present/absent lane counts.

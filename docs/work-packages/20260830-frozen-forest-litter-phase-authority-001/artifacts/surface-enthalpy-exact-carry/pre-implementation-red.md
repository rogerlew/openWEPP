# Version-16 surface-enthalpy exact-carry pre-implementation red

Status: `CONTRACT AUTHORITY PASS — SOURCE EXPECTED RED`

Evidence mode: `Static + Ran`

## Retained failure and evidence limit

The unchanged `p61` consumer reached exact support
`176400000000000..178200000000000 ns` and refused a positive retained-surface
enthalpy credit because checked binary64 addition did not change the resident
high-term bits. The retained evidence did **not** preserve the exact beginning
high bits or the accepted retained OFE/tile credit operands. No numeric oracle,
carry, or closure value is claimed here. Implementation must capture the typed
operands from the unchanged fixture and independently reconstruct them.

## Frozen authority

| Authority | SHA-256 |
|---|---|
| `SC-LANDSURFACEENERGY-001.md` v16 / `INV-LANDSURFACEENERGY-151` | `9036d60afa0e1d280a453879422cb4eca50f7350f433c93d08f722e724829f94` |
| `SC-SURFACELIQUID-001.md` v16 / `INV-SURFACELIQUID-023` | `75cc632b58d0a5cbee79121ebe5075a754d90903cb9185d424363a98a4b9ae86` |
| LSE authority test source | `91918ea0fa9fe6328966a729124e42285abab2888d9d27894e129b2644e77672` |
| Surface-liquid authority test source | `828c1399fa7690495847a32c54ea6d24cd85671930dc99f6daaeb138a5f29911` |

The selected minimal owner is `LseSurfaceEnthalpyOwnerEnvelopeV1`, joined to
unchanged LSE V3 and surface-owner V2 bytes. On the successor path those frozen
surface-enthalpy fields are nonauthoritative bit-identical high mirrors;
authoritative per-tile energy is `U=exact(U_hi)+R_U`. The complete successor is
`SurfaceLiquidCompleteOwnerProjectionV4`. Existing V1/V2/V3 serialization is
not mutated.

## Ran: authority PASS

- `nix develop -c cargo nextest run --test land_surface_energy_balance_authority_contract -E 'test(version_sixteen_binds_exact_lse_surface_enthalpy_carry)'`
  — PASS, nextest run `09d28bd1-faa5-4fb4-83b4-4e6cc5f6f654`.
- `nix develop -c cargo nextest run --test surface_liquid_hydrology_custody_authority_contract -E 'test(version_16_binds_exact_surface_enthalpy_custody_before_production)'`
  — PASS, nextest run `8e87cc6d-fb17-477e-824e-44bbfc3f70e7`.

The passing tests bind version/schema/digest/owner/transaction/support,
canonical exact operands, nearest-even high plus exact dyadic remainder,
immutable high mirrors, receipt/restart/checkpoint/projection custody,
independent vectors, rollback, real consumers, and the unchanged exact
60-second fallback/larger-stable-support obligation.

The complete authority binaries excluding only their named source-obligation
red each passed: LSE 15/15, nextest run
`bfc14a56-395c-47bc-995c-53ce8bb4b221`; surface liquid 16/16, nextest run
`408645e8-f1ed-4a48-955a-6fc60eab4d6d`.

## Ran: unchanged source expected FAIL

- `nix develop -c cargo nextest run --test land_surface_energy_balance_authority_contract -E 'test(version_sixteen_requires_exact_surface_owner_receipt_and_projection_symbols)'`
  — expected FAIL, nextest run `2d18610d-fdc8-4839-9214-2624a3d697b7`.
- `nix develop -c cargo nextest run --test surface_liquid_hydrology_custody_authority_contract -E 'test(version_16_requires_exact_surface_owner_receipt_restart_and_projection_symbols)'`
  — expected FAIL, nextest run `e1aa32ea-465d-431f-891d-83d0fb52cf04`.

Both reds failed first and only at the missing production symbol
`pub struct LseSurfaceEnthalpyOwnerEnvelopeV1`. No production edit was made for
this contract-first increment.

## Exact implementation seams

- canonical dyadic reuse:
  `crates/openwepp-land-surface-energy/src/exact_dyadic_enthalpy.rs`;
- exact owner/receipt and projection:
  `direct_runtime/surface_liquid_owner/v3_exact_enthalpy.rs` and
  `direct_runtime/surface_liquid_owner/v4_projection.rs`;
- retained credit:
  `land_surface_energy_shadow/real_hydrology_execution.rs::credit_retained_receipt_group`;
- authoritative consumer adoption:
  `land_surface_energy_shadow/v3_multitile_adoption.rs` and the package-owned
  frozen-litter real-consumer modules;
- restart/checkpoint:
  `openwepp-persisted-restart-v1/src/frozen_litter_v4_exact_enthalpy.rs`;
- real gates: unchanged `erosion_single_ofe_p61_sediment.rs` and
  `dff_ws1_native_forest_cli.rs` fixtures.

The package and owned-file manifest contain the full exact prospective write
set. Any renamed/new module requires a prospective manifest amendment before
creation.

## Hygiene

- `nix develop -c cargo fmt --all -- --check` — PASS.
- owned-file `git diff --check` — PASS.
- `tools/release/authority-policy/impact-map.json` — unchanged; the existing
  canonical contract/test bindings cover this contract-first increment. A new
  production path must receive an exact-path binding during implementation if
  no current entry covers it.

Disposition: `EXPECTED RED`. Production, `p61`, native-forest, restart,
rollback, and review closure remain open.

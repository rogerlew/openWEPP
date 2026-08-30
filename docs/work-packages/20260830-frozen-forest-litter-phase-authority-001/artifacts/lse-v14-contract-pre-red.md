# LSE v14 contract-first pre-red

Status: `RETAINED — EXPECTED PRODUCTION RED`

Evidence mode: `Ran`

## Contract successor

- predecessor: `SC-LANDSURFACEENERGY-001` v13, SHA-256
  `922917e963788ae10faae699ab8c6eb95180748d53a94b15aa484a34eeadfede`;
- pre-red successor: `SC-LANDSURFACEENERGY-001` v14, SHA-256
  `5cae2929a01aa70d7d3ef37f2e030a9a36b552285588960b294077ee75473969`;
- contract-derived test source SHA-256:
  `520cc94a1878494cf06f21c6aa9c739b0ee362aa787015182df62f3841e21832`.

The v14 amendment preserves the complete v13 authority and adds only the
immutable `OPENWEPP_SNOW_FREE_LSE_V3` successor. It binds the retained R-156
and SURFEX bytes, phase-free V2 solve, separate finalized liquid/ice signed
vapor, bounded `3300 s` kinetic phase, exact equal-mass/fusion-energy closure,
ending-capacity temperature, post-phase ingress and liquid-only WB14,
identities/migration/restart/receipts/rollback, unchanged exact 60-second
fallback, and canonical p61/native real-consumer obligations.

## Retained source hashes

| Retained source | SHA-256 |
|---|---|
| `references/vendorable/gmd-10-1621-2017-isba-meb-litter.pdf` | `2a8c14d912651457bf9205a4a963b78dd12f1aa7f243bccb025e4b81ce99716d` |
| `references/vendorable/surfex-v8/isba_meb.F90.source.html` | `0a300739b5dc660b61d29db144dd92f886e8fdf9934eac8facc022585992087a` |
| `references/vendorable/surfex-v8/isba_fluxes_meb.F90.source.html` | `e0378bc89ee0d52cffe14841aac56de1d8d379edf18ad29f24cfdb9ea0dfdbbc` |
| `references/vendorable/surfex-v8/ini_csts.F90.source.html` | `f39840df4d851efc70044f9e3ad62822371ed743c3c3a8055a4c940e2f86d73a` |
| `references/vendorable/surfex-v8/CeCILL-C_V1-en.html` | `7280115e43fa03917f2f23370519be8c9fb0b57f4c86f8da5f7ac10c070f6aa0` |

## Contract/test validation

Ran:

```text
nix develop --command rustfmt --edition 2024 --check \
  tests/integration/land_surface_energy_balance_authority_contract.rs
```

Result: `PASS`.

Ran:

```text
git diff --check -- \
  docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md \
  tests/integration/land_surface_energy_balance_authority_contract.rs
```

Result: `PASS`.

Ran:

```text
nix develop --command cargo nextest run \
  --test land_surface_energy_balance_authority_contract \
  version_fourteen_binds_frozen_litter_phase_vapor_and_atomic_chronology
```

Result: `PASS`, run ID `7b0d8b4f-21e1-4166-9dda-f29986844b65`, one test
passed and eleven were filtered. This proves the v14 contract-derived
authority assertions are well formed independently of production adoption.

Ran:

```text
nix develop --command cargo nextest run \
  --test land_surface_energy_balance_authority_contract --no-fail-fast
```

Result: expected `FAIL`, run ID
`cc12321a-be28-432a-888f-de0fa13c5dfd`: 12 tests ran, 10 passed and exactly
two production/adoption obligations failed.

```text
version_fourteen_requires_successor_production_identity_and_typed_guards:
unchanged production is missing required V3 binding
OPENWEPP_SNOW_FREE_LSE_V3

version_fourteen_requires_p61_and_native_real_consumer_adoption:
unchanged real consumer tests/integration/erosion_single_ofe_p61_sediment.rs
is missing V3 binding OPENWEPP_SNOW_FREE_LSE_V3
```

No contract-shape, source-hash, formula, chronology, predecessor, or existing
authority assertion failed. The red is therefore the intended missing V3
production and real-consumer adoption gate, not a malformed contract/test.

After this retained run, parent review clarified the preserved v13
`INV-LANDSURFACEENERGY-139` diagnostic representation only: the examined
exponent contributes to the existing cumulative backtracking count and no new
public/persisted field is authorized. Terminal v14 SHA-256 is
`857b49f06fdb675cd91fe2776727388aea72d19fdb999e2e4cd6e248f0e836d1`;
the v14 frozen-litter formulas, identities, chronology, and pre-red obligations
are byte-unchanged by that clarification.

Repository-wide `cargo fmt --all -- --check` was also attempted but was not
accepted as evidence because it reported concurrent formatting changes in
`tests/integration/surface_liquid_hydrology_custody_authority_contract.rs`, an
unowned file. The owned Rust test passed isolated `rustfmt --check` exactly.

## Line-count governance

| File | Lines | Disposition |
|---|---:|---|
| `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md` | 1659 | documentation authority; no Rust threshold |
| `tests/integration/land_surface_energy_balance_authority_contract.rs` | 830 | below Rust `WARN` threshold |

No production path, registry/index row, impact-map binding, or
`SC-SURFACELIQUID-001` byte was edited by this contract slice.

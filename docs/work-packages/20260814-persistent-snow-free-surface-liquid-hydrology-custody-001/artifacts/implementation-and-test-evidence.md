# Implementation And Test Evidence

Evidence class: `Static + Ran`

Status: `implementation complete / focused gates PASS / independent review active`

## Implemented Surfaces

- `DirectSurfaceLiquidConfiguration` binds the hydrology owner, run, explicit
  OFE topology, exact tile/surface/source keys, ingress mode, capacities, area,
  tile fractions and routed destination into deterministic SHA-256 identity.
- `DirectSurfaceLiquidOwnedState` binds one explicit caller-supplied store per
  configured key and one persistent WB14 continuation per OFE. State bytes
  include all masses, day/interval carry, cumulative supply/infiltration and
  accepted transaction lineage.
- Beginning-store withdrawals preserve the complete LSE `GroundWaterKey`, use
  one immutable snapshot, authorize same-store competition proportionally and
  allocate the final floating remainder without exceeding supply.
- The resource candidate validates exact `F <= A <= D`, debits finalized use
  only, credits accepted condensation on OFE-ground basis and emits capacity
  overflow instead of clipping it away.
- `DirectRunFrame` has a default-off optional surface-liquid owner state. The
  normal constructor leaves it `None`; explicit shadow configuration validates
  run identity, configuration and state before attachment.
- The LSE final protocol emits a condensation credit only for an admitted
  tile-local surface/litter source. Its specific liquid enthalpy is derived
  from the accepted surface temperature with the canonical LSE function.
- The production WB14 Green-Ampt interval transition is shared with the new
  persistent continuation rather than copied.
- `execute_surface_liquid_ingress` consumes the post-resource candidate,
  applies open raw precipitation or covered accepted canopy releases according
  to the digest-bound ingress mode, mixes chronological parcels, advances one
  persistent 1800-second WB14 continuation, retains excess by exact tile/source
  identity, and routes runoff with once-only unequal-area conversion.
- `execute_unified_real_hydrology_shadow` partitions one actual typed LSE
  request batch between the production soil-layer owner and the persistent
  surface-liquid owner, restores one ordered authorization batch, validates the
  accepted final protocol, and installs both owner candidates only in the
  cloned default-off frame.
- The beginning-hydrology digest is independently reconstructed from the
  production soil-owner snapshot fingerprint plus the validated surface-liquid
  configuration and state digests. A stale caller digest rejects before the
  fixed-cap constitutive callback can run.
- The real public LSE open-surface potential and fixed-authorization final
  functions are exercised through that unified bridge. The production frame is
  byte-identical after both successful and rejected shadow attempts.

## Focused Commands Run

| Command | Result |
|---|---|
| `cargo nextest run -p openwepp-hillslope-orchestrator surface_liquid --profile quick` | PASS, 14/14 selected; 507 skipped |
| `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings` | PASS after splitting oversized functions and removing strict float comparison |
| `cargo nextest run -p openwepp-land-surface-energy --profile quick` | PASS, 27/27 |
| `cargo clippy -p openwepp-land-surface-energy --all-targets -- -D warnings` | PASS |
| `cargo nextest run --test land_surface_energy_real_hydrology_shadow_contract --profile quick` | PASS, 8/8 |
| `cargo nextest run -p openwepp-hillslope-orchestrator --profile quick` | PASS, 521/521; three retained slow OFE-routing tests |
| `cargo check -p openwepp-hillslope-orchestrator` | PASS |
| `cargo check -p openwepp-land-surface-energy` | PASS |
| `cargo fmt --all -- --check` | PASS on complete implementation bytes |
| `git diff --check` | PASS on complete implementation bytes |
| `cargo nextest run --test surface_liquid_hydrology_custody_authority_contract --profile quick` | PASS, 9/9 |
| `cargo nextest run --test land_surface_energy_balance_authority_contract --profile quick` | PASS, 7/7 |
| `cargo nextest run --test auth11_required_suite_obligation_guards_contract` | PASS, 3/3 |
| `bash tools/release/check_authority_suite_antievasion.sh` | PASS |
| `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md` | PASS |
| `bash tools/release/check_science_contract_admission.sh --base-ref af9a989063aa8751dfadb14c442e1b360653658c --worktree` | PASS, 46 contracts / 11 science surfaces |
| `markdown-doc lint --path <package> --path SC-SURFACELIQUID-001.md` | PASS, 13 files / zero warnings |

## Preserved Failed Evidence

The first owner test run failed five assertions because tests incorrectly
assumed caller insertion order after canonical configuration sorting. The
implementation was not changed to satisfy that assumption; tests now locate
records by typed tile identity.

The first 48-step parity comparison found the stateful cumulative infiltration
bit-identical to the daily production wrapper while the sum of 48 excess
segments differed by one ULP from the daily wrapper's sum of 24 hourly bins.
The comparison now uses the existing admitted scale-aware depth-closure rule;
no production arithmetic, accepted state or infiltration result changed.

## Pending Before Closure

- fresh Rust correctness and hydrology/ownership review;
- disposition and correction of every accepted material finding;
- two exact-byte terminal verifiers;
- prompt archival and truthful terminal disposition.

# Implementation And Test Evidence

Evidence class: `Static + Ran`

Status: `re-review corrections complete / focused gates PASS / final re-review pending`

## Implemented Surfaces

- `DirectSurfaceLiquidConfiguration` additionally binds every OFE to its exact
  production lane index/ID, ordered soil layers, and soil-thermal recipient.
- `DirectSurfaceLiquidOwnedState` binds one explicit caller-supplied store per
  configured key and one persistent WB14 continuation per OFE. State bytes
  include all masses, day/interval carry, cumulative supply/infiltration and
  accepted transaction lineage.
- Beginning-store withdrawals preserve the complete LSE `GroundWaterKey`, use
  one immutable snapshot and authorize same-store competition with one
  symmetric common downward scale without row-specific or canonical-last
  remainder repair.
- The resource candidate validates exact `F <= A <= D`, debits finalized use
  only, credits accepted condensation on OFE-ground basis and emits capacity
  overflow instead of clipping it away.
- `DirectRunFrame` has a default-off optional surface-liquid owner state. The
  normal constructor leaves it `None`; explicit shadow configuration validates
  run identity, configuration and state before attachment.
- The LSE final protocol emits a condensation credit only for an admitted
  tile-local surface/litter source. Its specific liquid enthalpy is derived
  from the accepted surface temperature with the canonical LSE function.
- The complete production WB14 interval transition is shared with the new
  persistent continuation, including remaining storage, thresholds, clamps,
  guards, and cumulative state.
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
- The beginning-hydrology SHA-256 frames the production adapter's complete
  canonical snapshot bytes plus validated surface configuration/state identity;
  it no longer relies on the legacy 64-bit fingerprint.
- Infiltration receipts apply the shared production same-pass transition to the
  exact cloned lane and credit the named soil-thermal layer. Retained enthalpy
  credits the exact LSE tile. Missing or altered receiver/rollback joins return
  no candidate.
- The resource candidate is sealed and independently revalidated. Configuration
  and state persist only through canonical JSON bytes using original field names
  and exact 16-character IEEE-754 bit encodings.
- An external closure module reconstructs `W1 = W0 - F + C - overflow + retained`
  and every parcel mass/enthalpy/routing join from immutable operands; it does
  not consume a producer residual.
- The unified receiver validator freezes and reconstructs every ordered
  production-layer infiltration delta, aggregate soil-water ending, named
  soil-thermal enthalpy credit, and retained LSE tile enthalpy. Aggregate-equal
  wrong-layer distribution and omitted/doubled credits are rejected.
- Arbitration, ingress finalization, and unified candidates are sealed and
  expose read-only operands plus complete validators. Stored authorizations are
  independently re-derived from immutable `W0 + D`; a forged proportional
  allocation cannot reach resource debit.
- Runtime domain/envelope guards emit E004, E007, E008, E009, E010, and E011
  with phase, available typed identities, and beginning/attempted hashes.
- The real public LSE open-surface potential and fixed-authorization final
  functions are exercised through that unified bridge. The production frame is
  byte-identical after both successful and rejected shadow attempts.

## Focused Commands Run

| Command | Result |
|---|---|
| `cargo test -p openwepp-hillslope-orchestrator surface_liquid --no-fail-fast` | PASS, 30/30 selected; 507 filtered |
| `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings` | PASS after splitting oversized functions and removing strict float comparison |
| `cargo nextest run -p openwepp-land-surface-energy --profile quick` | PASS, 27/27 |
| `cargo clippy -p openwepp-land-surface-energy --all-targets -- -D warnings` | PASS |
| `cargo nextest run --test land_surface_energy_real_hydrology_shadow_contract --profile quick` | PASS, 15/15 |
| `cargo nextest run -p openwepp-hillslope-orchestrator --profile quick` | PASS, 537/537; retained slow OFE-routing tests |
| `cargo check -p openwepp-hillslope-orchestrator` | PASS |
| `cargo check -p openwepp-land-surface-energy` | PASS |
| `cargo fmt --all -- --check` | PASS on complete implementation bytes |
| `git diff --check` | PASS on complete implementation bytes |
| `cargo nextest run --test surface_liquid_hydrology_custody_authority_contract --profile quick` | PASS, 9/9 |
| `cargo nextest run --test land_surface_energy_balance_authority_contract --profile quick` | PASS, 7/7 |
| `cargo nextest run --test auth11_required_suite_obligation_guards_contract` | PASS, 3/3 |
| `bash tools/release/check_authority_suite_antievasion.sh` | PASS |
| `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md` | PASS |
| `bash tools/release/check_science_contract_admission.sh --base-ref af9a989063aa8751dfadb14c442e1b360653658c --worktree` | PASS, 46 contracts / 14 science surfaces |
| `cargo nextest run --test advisory_linter_authority_contract --profile quick` | PASS, 7/7 |
| `markdown-doc lint --path <package> --path SC-SURFACELIQUID-001.md` | PASS, 20 files / zero warnings |

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

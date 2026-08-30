# LSE V3 implementation evidence

Status: `IMPLEMENTATION COMPLETE — INTEGRATION PENDING`

Evidence mode: `Ran`

## Implemented boundary

The crate now exposes immutable `OPENWEPP_SNOW_FREE_LSE_V3` identity and
one-way V2-to-V3 configuration/state migration without changing V1/V2 bytes.
The V3 boundary keeps the predecessor nonlinear solve phase-free, publishes
separate liquid/ice vapor under liquid-water saturation, accepts only
separately finalized phase rates, installs vapor once, applies one bounded
`3300 s` phase operator, derives temperature from ending heat capacity, and
returns a non-mutating candidate plus
`OPENWEPP_FOREST_LITTER_PHASE_RECEIPT_V1`.

Receipt replay independently recomputes raw vapor from sealed atmospheric
operands, finalized vapor mass/enthalpy, post-vapor phase state, bounded
freeze/melt, fusion energy, ending state, and every closure field before
checking its canonical digest. It does not accept a producer residual.

The two model artifacts are:

- `crates/openwepp-land-surface-energy/artifacts/openwepp_snow_free_lse_v3_definition.json`,
  SHA-256 `309986036843cd1a5b83ede42655581fc2d2619ab8ab3d6224b812f86bf30ef6`;
- `crates/openwepp-land-surface-energy/artifacts/openwepp_snow_free_lse_v3_phase_vectors.json`,
  SHA-256 `8043ce776a36de780fc19f5ce9d36069dba99dbb1f246c8bfbc68d74e9fe3c08`.

The definition binds current terminal contract bytes:

- `SC-LANDSURFACEENERGY-001@14`:
  `857b49f06fdb675cd91fe2776727388aea72d19fdb999e2e4cd6e248f0e836d1`;
- `SC-SURFACELIQUID-001@14`:
  `bbb165f03c2f3588b32d4e97b41757612a73ef2641c4b4c8ae4d07f4a66df7e8`.

## Focused and crate gates

Ran:

```text
nix develop --command cargo nextest run \
  -p openwepp-land-surface-energy litter_phase --no-fail-fast
```

Result: `PASS`, run `770fe52e-1d20-44f5-adb9-db153fe9f609`, 13/13. The
vectors cover exact empty and nonempty reference temperature, exact 60 seconds,
`dt=tau`, `dt>tau`, all-liquid freezing, all-ice melting, mixed vapor,
condensation/deposition, phase-specific availability, ice-capacity saturation,
wrong sign/old-capacity distinctions, receipt replay/tamper, total-pool-cap
poison, support-floor poison, `rho_i`-capacity poison, and failed-candidate
nonmutation.

Ran:

```text
nix develop --command cargo nextest run \
  -p openwepp-land-surface-energy v3_state --no-fail-fast
```

Result: `PASS`, run `85449219-e0ea-4c7b-9d92-d2b3549007f4`, 3/3. V2 bytes
remain exact across successful migration and rejected mixed-identity migration;
V3 serialization/replay is exact and there is no production downgrade API.

Ran:

```text
nix develop --command cargo nextest run \
  -p openwepp-land-surface-energy --no-fail-fast
```

Result: `PASS`, run `4a7a9753-9737-4aa0-9dd4-a9d844e8ec5e`, 102/102, zero
skipped.

Ran:

```text
nix develop --command cargo clippy \
  -p openwepp-land-surface-energy --all-targets --no-deps -- -D warnings
```

Result: `PASS`.

Ran:

```text
nix develop --command cargo nextest run \
  --test land_surface_energy_balance_authority_contract \
  version_fourteen_requires_successor_production_identity_and_typed_guards
```

Result: `PASS`, run `779c603d-e5c8-4c65-b139-358eeda052a1`, 1/1. The
contract-owned production scan found the V3 model/receipt identities and all
four typed error families in the declared source set.

Ran isolated `rustfmt --edition 2024` on every owned Rust path; result `PASS`.
Ran `git diff --check -- crates/openwepp-land-surface-energy`; result `PASS`.
Production scans found no `.unwrap()`, `.expect()`, print/debug output, or
microstep diagnostic persistence in the six V3 modules.

## Independent closure observation

A dependency-linked `/tmp` probe reconstructed a cold mixed-phase,
phase-specific condensation/deposition case from primitive public API operands.
Its exact reported residuals were:

| Residual | Observed |
|---|---:|
| liquid equal-mass | `0.0 kg m^-2` |
| ice equal-mass | `0.0 kg m^-2` |
| total phase mass | `0.0 kg m^-2` |
| fusion energy | `0.0 J m^-2` |
| `U-L_f*W_i` phase enthalpy | `1.74622982740402222e-10 J m^-2` |
| ending temperature | `0.0 K` |
| liquid vapor energy | `0.0 J m^-2` |
| ice vapor energy | `0.0 J m^-2` |

The same vector froze `1.16123464189391679 kg m^-2` and melted exact zero.
The nonzero enthalpy-coordinate residual is binary floating-point cancellation,
well inside the unchanged `1e-7 + 64*epsilon*scale` closure envelope; no
tolerance was changed.

## Line-count disposition

No new file approaches the 2000-line warning threshold. Existing inherited
files remain unchanged at `solver_covered_evaluation.rs=2100` and
`transaction.rs=2762`; this increment did not edit or grow either file.

## Integration handoff

The surface-owner worker confirmed the API seam is compatible. Integration
must provide exact beginning/candidate surface-owner digests, convert sealed
owner liquid/ice state to `BeginningLitterPhaseState`, pass accepted phase-free
atmospheric operands and separately finalized rates, then install
`AcceptedLitterPhaseCandidate` only through its whole-owner transaction.
`build_v3_ending_state` constructs the matching non-mutating LSE state
candidate. Remaining blockers are outside this worker's write set: real
surface-owner projection/restart/rollback adoption, runner seed/restart, and
unchanged `p61` plus native-forest consumer evidence.

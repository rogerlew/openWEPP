# LSE V3 implementation evidence

Status: `IMPLEMENTATION COMPLETE — INTEGRATION PENDING`

Evidence mode: `Ran`

## Implemented boundary

The crate now exposes immutable `OPENWEPP_SNOW_FREE_LSE_V3` identity and
one-way V2-to-V3 configuration/state migration without changing V1/V2 bytes.
The V3 boundary keeps freeze/melt out of the nonlinear system while evaluating
separate liquid/ice vapor under liquid-water saturation in every V3 current-
trial residual and finite-difference evaluation. It accepts only
separately finalized phase rates, installs vapor once, applies one bounded
`3300 s` phase operator, derives temperature from ending heat capacity, and
returns a non-mutating candidate plus
`OPENWEPP_FOREST_LITTER_PHASE_RECEIPT_V1`.

Receipt replay independently recomputes raw vapor from sealed atmospheric
operands, finalized vapor mass/enthalpy, post-vapor phase state, bounded
freeze/melt, fusion energy, ending state, and every closure field before
checking its canonical digest. The receipt now seals the complete accepted
phase-free surface-energy ledger and independently reconstructs
`U*-U0=dt*(SW_abs+LW_net-H-Q_v,l-Q_v,i-G)`, both named vapor-energy joins,
storage, and the producer/reconstructor delta. It does not accept a producer
residual as closure evidence.

The two model artifacts are:

- `crates/openwepp-land-surface-energy/artifacts/openwepp_snow_free_lse_v3_definition.json`,
  SHA-256 `b8d8886d640f6993e7b6a9f22cc49a5a6d9871caf61a2f82a4041157231117fb`;
- `crates/openwepp-land-surface-energy/artifacts/openwepp_snow_free_lse_v3_phase_vectors.json`,
  SHA-256 `a72500224de82c135a6a76f63764b76f2e0023a2a77b677de9363537977c7c61`.

## Reopened residual correction

Static pre-red: the first V3 slice called the V2 liquid-only covered evaluator
and published phase-specific vapor only after that solve. Its receipt closure
reconstructed phase and per-component vapor enthalpy but did not bind the
complete beginning-to-post-vapor surface-energy ledger. That topology could
not prove INV-LANDSURFACEENERGY-141 or the inherited V2 surface-energy balance.

Positive correction: `evaluate_v3_phase_free_covered_column` replaces the
shared canopy-vapor and litter surface-energy equations at every V3 trial with
separate current-trial liquid/ice mass and sensible-plus-latent energy. The
new `solve_v3_phase_free_covered_column` feeds that residual into every Newton,
Jacobian, and backtracking evaluation; freeze/melt remain absent. Fixed
authorization is applied per named phase as `min(current raw, authorization,
immutable beginning availability)` for outbound mass, while inbound mass stays
exactly constitutive.

Rollback/anti-mutant evidence: focused tests reseal and reject (1) liquid
enthalpy substituted for the ice component while keeping the aggregate ledger
balanced, (2) a vapor-only `U*` identity that omits non-vapor storage, and
(3) producer residual substitution. The existing failed-candidate test retains
exact input bytes after a rejected named-phase availability violation.

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

Result after residual correction: `PASS`, run
`3df50958-80bf-43cc-bfdd-5c06cc67a6b3`, 14/14. The
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

Result after residual correction: `PASS`, run
`ce83e810-0a24-404f-affa-2ada02ded5f8`, 104/104, zero
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

Result after residual correction: `PASS`, run
`623fe7c4-d88c-4713-995d-421dc646ba33`, 1/1. The
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
| phase-free beginning/post-vapor storage | `0.0 W m^-2` |
| complete phase-free surface energy | `-1.11022302462515654e-13 W m^-2` |
| producer/reconstructor energy delta | `1.11022302462515654e-13 W m^-2` |

The same vector froze `1.16123464189391679 kg m^-2` and melted exact zero.
The nonzero enthalpy-coordinate residual is binary floating-point cancellation,
well inside the unchanged `1e-7 + 64*epsilon*scale` closure envelope; no
tolerance was changed.

## Line-count disposition

No new file approaches the 2000-line warning threshold. The owned existing
`solver_covered_evaluation.rs` is now 2342 lines (`WARN`, below the mandatory
3000-line refactor threshold); the V3 correction is isolated in its appended
current-trial evaluator pending the broader covered-evaluator decomposition.
Inherited `transaction.rs` remains unchanged at 2762 lines.

## Integration handoff

The surface-owner worker confirmed the API seam is compatible. Integration
must provide exact beginning/candidate surface-owner digests, convert sealed
owner liquid/ice state to `BeginningLitterPhaseState`, pass accepted phase-free
atmospheric operands, the accepted `V3PhaseFreeSurfaceEnergyLedger`, and
separately finalized rates, then install
`AcceptedLitterPhaseCandidate` only through its whole-owner transaction.
`build_v3_ending_state` constructs the matching non-mutating LSE state
candidate. Remaining blockers are outside this worker's write set: real
surface-owner projection/restart/rollback adoption, runner seed/restart, and
unchanged `p61` plus native-forest consumer evidence.

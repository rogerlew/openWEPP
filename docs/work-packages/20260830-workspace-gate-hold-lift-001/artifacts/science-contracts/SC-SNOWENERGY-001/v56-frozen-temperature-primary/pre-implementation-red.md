# V56 frozen temperature-primary pre-implementation red

Status: `EXPECTED_RED`

Evidence mode: `Static + Ran`

## Authority and retained proof

`SC-SNOWENERGY-001` version 56 binds `REF-SNOWENERGY-WGHL-V56`,
`INV-SNOWENERGY-080`, and `OBL-SNOWENERGY-C-048` before production edits.
The direct blocker is `/tmp/wghl_001d_v55_64m_r144.log`, SHA-256
`161712621295b503da41b065846304ce0e0198a26a9d9b97efa6d4012fa36c65`,
wall `6:46.42`, RSS `442360 KiB`. The exact-floor `2100..2160 s` solve
exhaustively evaluated the complete 21-member V55 Q candidate interval and
found no exact witness. The final candidate had Q bits
`4662593950276069748`, output-Q bits `4662593950276069730`, nonzero RQ bits
`4445615782168100864`, and merit `2.1827872842550278e-5`.

This is direct evidence that a transient V54/V55 cycle or lattice artifact is
not authenticated material/carry authority. It is not an absence proof for a
strictly frozen temperature-primary material state whose exact enthalpy
remainder is retained.

## Contract-first result

Ran:

`nix develop -c cargo nextest run --test snow_terminal_enthalpy_event_numerics_contract v56_`

Nextest run ID: `f4c71aeb-ee0e-40cc-992c-017f005c9fb0`.

Result: `2 tests run: 1 passed, 1 failed, 54 skipped`, exit `100`.

- PASS: `v56_contract_binds_frozen_temperature_primary_compound_owner`.
- EXPECTED RED:
  `v56_frozen_temperature_primary_production_seams_are_required`.

The expected red enumerates absent production seams for strict frozen
eligibility and temperature-primary solve, compound snow material/carry
receipt authority, restart V5, RN-even carry construction, dispatch wiring,
and nine required behavior vectors. No production `.rs` file was edited by
this checkpoint.

Ran the complete source contract after the focused red:

`nix develop -c cargo nextest run --test snow_terminal_enthalpy_event_numerics_contract`

Nextest run ID: `9c21351b-461c-4ce4-bdf2-d8b6c83376d0`.

Result: `56 tests run: 55 passed, 1 expected-red failed, 0 skipped`, exit
`100`. The sole failure is the V56 production-seam obligation above; all
retained contract/source obligations pass against contract version 56.

Ran before that command: `nix develop -c cargo fmt --all -- --check` and
`git diff --check`; both passed.

## Frozen implementation boundary

Production must implement temperature-primary `(W,T_s,rho,Esoil,Tsoil)`,
exact `H=-exact(W)*exact(c_ice)*(exact(273.15)-exact(T_s))`, RN-even high plus
exact dyadic carry, unchanged CN exact-once use, authenticated compound owner
and additive carry receipt, whole-receipt stabilization/replay/finalization,
and V5 committed/pending/in-progress restart custody. It must retain the
shared maximum 96, exact 60-second floor, unchanged physics/ledger tolerances,
events, topology, custody, rollback, and no private publication or persistent
solver diagnostics.

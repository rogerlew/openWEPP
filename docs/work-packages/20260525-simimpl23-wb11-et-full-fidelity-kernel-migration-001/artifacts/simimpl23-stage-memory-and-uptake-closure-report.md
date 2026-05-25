# SIMIMPL23 Stage-Memory and Uptake Closure Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- WB17 ET runtime now supports baseline-authoritative stage-memory symbols
  (`s1`, `s2`, `tu`, `tv`) with explicit all-or-none symbol-family presence.
- Partial stage-memory payloads fail typed at boundary (`MissingRequiredStateSymbol`),
  rather than silently defaulting missing members.
- Stage transition logic now updates `s1/s2/tv` across both stage branches and
  preserves explicit threshold handling around `tu`.
- ET lineage now emits `Etp`, `UPi`, and `Ui` plus stress ratio `Ws` with
  explicit zero-demand guard (`Etp <= 1e-12 => Ws = 1.0`).
- SIMIMPL22 vector family moved from expected-fail to pass by enabling all
  four vectors in default execution.

## Ran
- `cargo test -p openwepp --test wb11_hydrology_kernel_contract`
- `cargo test -p openwepp --test wb17_et_physics_kernel_contract`
- `cargo test --workspace`

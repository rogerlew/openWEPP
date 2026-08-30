# SC-SNOWENERGY-001 v32 pre-implementation contract gate

Evidence mode: `Ran`

Disposition: `PASS — EXPECTED RED ISOLATED TO MISSING V32 PRODUCTION`

## Contract-first source identity

| Surface | SHA-256 |
|---|---|
| canonical `SC-SNOWENERGY-001.md` v32 | `2f2cea494e5950c3516de67d804d15eabdcd9fa03c6533eed6eb340b9d112f6b` |
| science-contract `index.md` | `f69a07c1a52181219332410901e3025a768477aec08389055f5c753ea5e8d596` |
| contract-derived integration test | `9941f0fa966f766d90ee8c44235d15354bb908c0337849dd407dc5c39eb589d7` |
| unchanged `v11_covered/fixed_point.rs` | `7e6dbe0653ee4727e64a93c2448f60ea8f87b675f68a0f88b9e3b9a40c3be7f1` |
| unchanged `v11_covered/open_snow.rs` | `3418232f68b0f472ed488ad4db676f27b1e5f2ed7f91d5986eddbcfa5f715349` |
| unchanged `v11_covered/open_snow_convergence_tests.rs` | `4b87082cf67631b4ce4b66020ed1adb80cb3d6b3e8485613c363a441ccd0a595` |

The three production hashes are byte-identical to the intake freeze in
`source-freeze.md`; no production implementation was edited for this gate.

## Contract lint and diff gates

| Command | Result | Retained log SHA-256 |
|---|---|---|
| `.venv/bin/python tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md` | PASS; 19 binding-exposure rows fully consolidated | `82e5ad40cb48d6dffd59b371a5bb9fb2a4c37a3970226dd8e6981bbd536e0349` |
| `.venv/bin/python tools/release/check_sc_unit_compliance.py --path docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md` | PASS; no findings | `8b7cb5e42506defea3f986af17b397222e6a6c4f7cf9a7957e4f04d7a6770273` |
| `git diff --check -- <exclusive v32 write set>` | PASS; no output | not applicable |
| `nix develop -c rustfmt --edition 2021 --check tests/integration/snow_terminal_enthalpy_event_numerics_contract.rs` | PASS | `43fd0499f49ef6564189b8636fc38973e8f4ee3b489831c01084b89be01f50f7` |

The binding-exposure status values were normalized from the invalid lifecycle
token `in_review` to `active` for the v17, v18, and superseding v32 rows. Their
existing `flagged-binding-addition` review gates remain unchanged; this is
schema/lifecycle normalization, not a review waiver.

## Isolated expected-red execution

Command:

```text
nix develop -c rustc --edition 2021 --test \
  tests/integration/snow_terminal_enthalpy_event_numerics_contract.rs \
  -o /tmp/wghl_v32_contract_test
/tmp/wghl_v32_contract_test --test-threads=1
```

Exit: `101` (expected red)

Retained log: `/tmp/wghl_v32_pre_implementation_red.log`

Log SHA-256:
`3654192242b2d23dcb439b536057991d56b78277882260d24c1fb17bc1ef21d5`

Result: 6 tests executed; 5 passed and exactly 1 failed. The passing tests
include the exact captured deposition/sublimation operands, binary64
`alpha_v=0.04393657257739406`, exact-zero localized vapor, and rejection of
the affine-latent `+45.77845449909091 J m^-2` result. Canonical contract/index
bindings also passed.

The sole failure was
`v32_production_symbols_and_larger_direct_support_behavior_are_required`, at
the first absent production symbol:
`CoveredVaporActiveSetInterfaceV1` in `v11_covered/fixed_point.rs`. That test
also prospectively requires interface and branch-entry functions, the unchanged
60-second minimum-support constant, direct dispatch from `open_snow.rs`, a
same-sign-v31 test, a synthetic-no-publication test, and the explicit
`v32_vapor_active_set_accepts_direct_support_above_exact_floor` production
behavior vector.

No contract, oracle, formatting, unit, binding-exposure, compile, unrelated
test, or existing-production failure occurred. The red is therefore solely the
missing v32 production symbols/behavior required before implementation.

The std-only contract target was compiled directly with the repository Nix
Rust toolchain because unrelated concurrent soil-enthalpy edits temporarily
made Cargo compile additional in-flight workspace crates. Direct `rustc --test`
executes the complete target without bypassing any of its six tests and keeps
this contract-first red independent of those out-of-scope compilation states.

## Protected-boundary result

Static source/diff review confirms no production file, impact map, assurance
lock, package plan, adaptive test, schema, persistence, receipt, rollback,
publication, or diagnostic surface was edited in this contract-first slice.
The v32 contract records no tolerance, cap, constitutive, event, custody,
topology, receipt, rollback, or temporal-policy relaxation.

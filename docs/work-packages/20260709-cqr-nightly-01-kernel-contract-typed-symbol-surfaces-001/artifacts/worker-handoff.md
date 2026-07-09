# Worker Handoff

Status: `COMPLETE`

Package status: `EXECUTED-COMPLETE-CQR-NIGHTLY`

Completion summary:

- Target module rank `1` of `10` was completed.
- Production source changed only in
  `crates/openwepp-kernel-contract/src/lib_mod/core_types/01_typed_symbol_surfaces.rs`.
- Characterization tests changed only in
  `tests/integration/arch22_typed_state_surface_contract.rs`.
- Package-local evidence lives under
  `docs/work-packages/20260709-cqr-nightly-01-kernel-contract-typed-symbol-surfaces-001/artifacts/`.
- Package-local current-run gate logs live under
  `docs/work-packages/20260709-cqr-nightly-01-kernel-contract-typed-symbol-surfaces-001/artifacts/logs/`.

Closure facts:

- CRAP rows above `30`: `0`.
- Max target CRAP: `22.035011574074073`.
- ADR-0021 line coverage: `278 / 284 = 97.88732394366197%`.
- ADR-0021 unique source-region coverage: `332 / 338 =
  98.22485207100591%`.
- Full workspace nextest: `1490 tests run: 1490 passed (4 slow), 3 skipped`.
- Dual second-pass verification: PASS/PASS.

Next package:

- Continue the nightly sequence with rank `2`:
  `crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/chaninp.rs`.

Scratch artifacts:

- Root-level untracked `artifacts/` logs from earlier measurement/final runs are
  intentionally not package-local closure evidence. Do not stage them unless a
  later package explicitly adopts them.

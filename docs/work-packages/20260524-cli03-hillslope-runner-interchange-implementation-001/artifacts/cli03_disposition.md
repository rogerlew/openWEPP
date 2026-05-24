# CLI03 Disposition

Status: completed
Evidence mode: Static + Ran

## Disposition
- Package state: completed
- Scope outcome: implemented
- Required repository gates: passing

## Exit Criteria Check
- [x] CLI03 objective is evidence-backed.
- [x] `.run` contract behaviors are implemented for required/optional inputs
      and outputs.
- [x] `unit_system = "metric"` is explicitly enforced.
- [x] `--legacy-sidecar-discovery` behavior is implemented with discovered
      sidecars authoritative and `.run` override suppression.
- [x] `snow`/`frost` override semantics are preserved (override controls,
      not routine toggles).
- [x] Required outputs (`pass`, `loss`) are enforced with hard-fail behavior.
- [x] Optional parquet outputs are emitted only when configured.
- [x] Dedicated crate `crates/openwepp-hillslope-output/` owns output
      contracts, serializer logic, and output-surface tests.
- [x] Contract-derived tests are implemented and executed.
- [x] Pre-implementation contract gate is recorded before production code
      implementation evidence.
- [x] Required repository gates executed and passing (`fmt`, `clippy`,
      `test --workspace`, `deny`).
- [x] Dual review and dual verification artifacts are completed.
- [x] Kernel-profile compliance checklist artifact is completed.
- [x] Python consumer-boundary alignment to CLI03 runfile/output authority is
      implemented and verified (`tests/python/test_open_wepp_runner_api.py`).

## Governance Notes
- Dedicated security review requirement is satisfied for CLI03 scope.

Disposition decision:
- `GO` for CLI03 package scope closure.

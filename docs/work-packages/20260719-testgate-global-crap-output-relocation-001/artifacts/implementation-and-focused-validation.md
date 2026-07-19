# Implementation And Focused Validation

Evidence class: `Static` and `Ran`

## Correction

The runner now initializes its default output as the safe relative path
`target/adjudicated-crap`. Executor mode validates and relocates that relative
path beneath `OPENWEPP_GATE_ARTIFACT_ROOT`. Standalone mode resolves only the
unmodified default against `ROOT_DIR`; an explicit `--output-dir` retains its
prior behavior.

Direct runner SHA-256 after correction:
`31ea52774fe7ed013794d82c50dc14926163f0e0a50286c532c0fde3daae4491`.
Both `affected-adjudicated-crap-v1` and `adjudicated-crap-v1` bind that exact
digest. No gate command, risk class, prerequisite, output contract, coverage or
CRAP behavior, threshold, or exception changed.

## Focused Evidence

- `bash -n tools/release/run_adjudicated_crap_gate.sh`: PASS.
- `cargo fmt --check`: PASS.
- Direct SHA/JQ equality for both adapter bindings: PASS.
- Exact source assertions plus isolated behavioral probes cover standalone
  default, relative override, absolute override, executor default, executor
  relative override, rejected executor absolute output, and rejected executor
  traversal. Every accepted path creates its fail-envelope beneath the expected
  directory, then exits before acquisition on the deliberately absent scratch
  Python prerequisite: PASS.
- `cargo nextest run --test testgate_ci_executor_contract --test
  testgate_align_authority_contract`: final focused run 13/13 PASS, 0 skipped;
  test execution 0.248 seconds after a 0.55-second incremental build. The
  preceding 12/12 source-contract run remains valid but is superseded by this
  stronger focused result.
- `git diff --check`: PASS.

The changed integration test is 558 lines, below the 2,000-line warning
threshold. No production Rust file changed.

The pre-edit local helper observation was intentionally zero-work and was
rejected with `zero-work increment cannot be admitted`; it is not gate evidence.
The terminal intent/plan is generated only after the package-authorized diff
exists.

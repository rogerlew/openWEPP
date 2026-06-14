# Implementation Test Evidence

Status: W-B executed-hold

Evidence mode: Ran + Static

W-A baseline:

Ran:

- `target/debug/openwepp-cli-watershed --run-dir /tmp/openwepp_wshed01_wa/watershed/run --run-file case.run --output-dir /tmp/openwepp_wshed01_wa/watershed/output --policy compat --legacy-sidecar-discovery`

Observed:

- Exit code `1`.
- `CLIWAT-E-010` wrapping `IMP-E-004` on `pw0.imp` line 2, `jpond=0`.
- `0` output files under `/tmp/openwepp_wshed01_wa/watershed/output`.

Not run:

- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check`. W-A made no Rust edits and
  did not claim production closure.

Static evidence:

- Parser defect before W-B: `watershed_impoundment.rs:581-588`.
- CLI failure wrapper: `openwepp-cli-watershed.rs:239-254`.
- Output writer path not reached: `openwepp-cli-watershed.rs:476-497`.

W-B implementation:

- Added explicit no-impoundment fixture and parser/CLI tests.
- Implemented typed empty impoundment set semantics in
  `watershed_impoundment.rs`: `jpond=0` is accepted only when
  `expected_structural_count == Some(0)`.
- Amended `openwepp-watershed-runfile-contract.md` to pin schema v1 no-pond
  semantics and preserve the required `inputs.pw0_imp` file binding.

Red evidence:

- `cargo test --test infile_watershed_impoundment_parser_contract zero_impoundments`
  failed before the parser edit:
  - strict zero/zero acceptance failed with `IMP-E-004`,
  - compatibility zero/zero acceptance failed with `IMP-E-004`,
  - positive-structure mismatch test observed `DomainError` instead of the
    required `CountMismatch`.

Green evidence:

- `cargo fmt --check`: pass.
- `cargo clippy -p openwepp-input-contract -p openwepp-runner --tests -- -D warnings`:
  pass.
- `cargo test --test infile_watershed_impoundment_parser_contract`: `18`
  passed.
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract watershed_cli_accepts_explicit_zero_impoundment_file_when_structure_has_none`:
  `1` passed.
- `cargo test -p openwepp-watershed-orchestrator watershed_impoundment_runtime_seed`:
  `3` passed.

Arboreal-dendrite W-B gate:

```bash
target/debug/openwepp-cli-watershed \
  --run-dir /tmp/openwepp_wshed01_wa/watershed/run \
  --run-file case.run \
  --output-dir /tmp/openwepp_wshed01_wb/watershed/output \
  --policy compat \
  --legacy-sidecar-discovery
```

Observed:

- Exit code `1`.
- No `CLIWAT-E-010` / `IMP-E-004`.
- Next hard stop:
  `CLIWAT-E-020 watershed dispatch reported failure (message_id=WKERNEL-WS10-CHANNEL-E-003)`.
- Output file count: `0`.

# Verification

Evidence mode: Static/Ran.

## Static Verification

Static:

- Verified the implemented sub-computations are pure typed projection functions
  with `HillslopeWritebackSurface` limited to writer/reader adapters around
  them.
- Verified production direct execution calls
  `DirectProductionSeedAuthority::from_typed_inputs`.
- Verified the snowbench diagnostic replay also uses the typed seed authority.
- Verified no production caller reaches `from_day_zero_seed_surfaces` or
  `direct_publication_day_zero_seed_surface`.
- Verified retained symbol-map seed helpers are transition/test-only and are the
  Phase 3 deletion target.

## Ran Verification

```text
cargo fmt --check
cargo check -p openwepp-runner
cargo nextest run -p openwepp-runner publication_wb11_seed publication_wb19_wb12_wb16
cargo test -p openwepp --test cli03_runner_contract_derived_tests cli03_mofe03_multiofe_runfile_executes_wave2_without_manual_symbol_injection -- --nocapture
cargo clippy --workspace --all-targets -- -D warnings
cargo nextest run --workspace --profile full
cargo deny check
bash tools/release/check_authority_suite_antievasion.sh
cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture
markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260630-typed-day-zero-seed-computation-001 --format json
markdown-doc validate --path docs/work-packages/README.md --path docs/work-packages/20260630-typed-day-zero-seed-computation-001
git diff --check
```

Results:

- `cargo fmt --check`: pass.
- `cargo check -p openwepp-runner`: pass.
- Focused seed tests: `41` run, `41` passed.
- Multi-OFE/Wave-2 focused test: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- H2637 protected output identity: HBP/loss/PASS/WAT/plot byte-identical
  against clean `5b139058`.
- H2637 RSS/time: clean baseline `1:09.02`, `113268 KiB`; current `1:08.62`,
  `91692 KiB`.
- cli01 protected output identity: HBP/loss/WAT/plot byte-identical.
- `cargo nextest run --workspace --profile full`: `1879` passed, `1` skipped,
  `2` slow, `671.206s`.
- `cargo deny check`: pass.
- Authority anti-evasion: pass.
- Required-suite obligation guard: `2` passed.
- `markdown-doc lint`: `10` files scanned, `0` errors, `0` warnings.
- `markdown-doc validate`: `10` files, `0` errors.
- `git diff --check`: no findings.

## Pending

- Phase 3 deletion of transition/test-only symbol-map seed helpers and the
  production compatibility runtime machinery.
- Phase 4 no-compatibility static call-graph proof after deletion.

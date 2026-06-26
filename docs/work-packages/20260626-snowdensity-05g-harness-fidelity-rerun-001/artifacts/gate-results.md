# Gate Results

Evidence class: Ran.

## Focused Development Gates

```text
cargo fmt --check
```

Initial result: failed on rustfmt wrapping in the new CoE contract string.
Disposition: ran `cargo fmt`.

```text
cargo test --test snowdensity05g_harness_fidelity_rerun -- --nocapture
```

Result after registering the test target: pass, `3 passed; 0 failed`;
executable CSS Lab replay confirmed `canopy_cover_fraction = 0.9`.

```text
cargo test --test snowdensity02_contract_adr_guard --test snowdensity05a_melt_contract_guard --test snowdensity05b_shortwave_source_contract --test snowdensity05c_albedo_state_core --test snowdensity05d_opt_in_coe_melt --test snowdensity05e_melt_adjudication --test snowdensity05f_melt_closure_handoff --test snowdensity05g_harness_fidelity_rerun
```

Result: pass. All focused SNOWDENSITY ladder guards passed against
`SC-SNOWFREEZE-001` v83.

## Adjudication

```text
cargo build -q -p openwepp-runner --bin openwepp-snowbench
.venv/bin/python tools/snowfreeze_observed/coe_melt_adjudication.py --observations-dir tests/fixtures/snotel_observed/observations --output-dir target/snowdensity05g_coe_melt_adjudication --snowbench-binary target/debug/openwepp-snowbench
```

Result: pass. Aggregate artifacts copied to
`artifacts/snotel-adjudication.{json,md}`.

Summary:

- `legacy_coe`: robust failures `9`, ordinal score `84`.
- `coe_shortwave_albedo_v1`: robust failures `9`, ordinal score `86`.
- Disposition: `NON-PROMOTION`.
- All site/model summaries report canopy `0.9` and shortwave bridge proof
  `true`.

## Source Scans

```text
rg -n "DEFAULT_CANOPY_COVER_FRACTION|cancov = 0.0|snowdensity05e-coe-melt|snowdensity05g|CoeShortwaveAlbedoV1|--model|snow_melt_model" ...
```

Result: pass. The old `DEFAULT_CANOPY_COVER_FRACTION` path is absent; `--model`
remains confined to `openwepp-snowbench`; production binaries did not gain
activation selectors.

## Final Gates

```text
cargo fmt --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && cargo deny check
```

Result: pass.

- `cargo fmt --check`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo test --workspace`: pass.
- `cargo deny check`: pass, `advisories ok, bans ok, licenses ok, sources ok`.

```text
git diff --check
```

Result: pass.

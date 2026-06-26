# Gate Results

Evidence mode: Ran.

## Required Gates

- `cargo fmt --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `cargo deny check` passed:
  `advisories ok, bans ok, licenses ok, sources ok`.
- `git diff --check` passed.
- `rg -n "qwet|frzftp" crates` returned no matches. The command exited `1`,
  which is the expected `rg` status for an empty result set.

## Focused Gates

- `cargo test --test snowdensity06b_coe_bound_density_replay -- --nocapture`
  passed.
- `cargo test --test snowdensity03_physics_bulk_offline_contract -- --nocapture`
  passed.
- `cargo test -p openwepp-runner snowbench -- --nocapture` passed.
- `cargo build -p openwepp-runner --bin openwepp-snowbench` passed.
- `.venv/bin/python tools/snowfreeze_observed/coe_bound_density_adjudication.py --output-dir target/snowdensity06b_coe_bound_density`
  passed.

## Adjudication Result

Best candidate:
`coe_bound_density_compaction_v1_coe_shortwave_albedo_v1`.

- Whole robust profile vs openWEPP/legacy as-built: failures `9 -> 5`, score
  `84 -> 110`.
- Density-cell profile vs openWEPP/legacy as-built: failures `9 -> 5`, score
  `16 -> 41`.
- Maximum daily CoE SWE identity residual:
  `4.440892098500626e-16 m`.
- Maximum unbounded SWE residual before fixed-boundary normalization:
  `0.1285 m`.

Disposition: `COMPLETE-06B-COE-BOUND-DENSITY-REPLAY`.

This evidence is offline diagnostic evidence only. It does not authorize
runtime/default activation, parser/runfile selector changes, output-schema
publication, mixed/deciduous canopy adjudication, or frost attribution.

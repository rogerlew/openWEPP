# Gate Results

Evidence class: Static/Ran.

## Primary Gate

Ran:

```bash
.venv/bin/python tools/snowfreeze_observed/harder_pomeroy_default_activation.py
```

Result: PASS.

- Prior activated bundle with explicit `legacy_rst`: `17` robust fails / `172`
  robust ordinal score.
- New no-env default with `harder_pomeroy_hourly`: `15` robust fails / `179`
  robust ordinal score.
- Better / worse robust cells vs prior: `9` / `2`.
- Selector trace gate: PASS.
- Partition conservation gate: PASS, max trace residual `5.551115123125783e-17 m`.

## Workspace Gates

Ran:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
bash tools/release/check_authority_suite_antievasion.sh
cargo test --test auth11_required_suite_obligation_guards_contract
```

Results:

- `cargo fmt --check`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo test --workspace`: PASS.
- `cargo deny check`: PASS (`advisories ok, bans ok, licenses ok, sources ok`).
- Authority-suite anti-evasion: PASS.
- `auth11_required_suite_obligation_guards_contract`: PASS.

## Release Notes

- Humid-New-England depth regression remains a non-representative roadmap item
  under the cross-SNOTEL primary gate.
- Cross-SNOTEL density median bias rises to `+23.6234 kg m^-3`; recovery is
  tracked separately.
- No `.run` disable option, fixture edit, public schema change, density-cap
  change, frost change, or parser/runfile/user selector was added.

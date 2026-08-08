# Gate Results

Status: pass

Evidence mode: Ran

| Gate | Result |
|---|---|
| `cargo nextest run --test land_surface_energy_balance_authority_contract` | 5 passed, 0 skipped |
| `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md` | PASS, no findings |
| `cargo nextest run --test assurance_v2_source_contract` | 12 passed, 0 skipped |
| `cargo fmt --all -- --check` | PASS |
| `markdown-doc lint --path .../SC-LANDSURFACEENERGY-001.md` | 1 file, 0 errors/warnings |
| `markdown-doc lint --path docs/work-packages/20260808-land-surface-energy-balance-authority-001` | 34 files, 0 errors/warnings before prompt archive; terminal rerun recorded below |
| `git diff --check` | PASS |
| `cargo nextest run --workspace --profile full` | 2,315 passed, 33 skipped, 0 failed; 2,277.832 s |
| `cargo nextest run --workspace --profile quick` | 2,266 passed, 40 skipped, 0 failed; 2,270.469 s |

The full and quick runs used the frozen reviewed contract/test bytes. An older
quick run begun before final review remediations was intentionally interrupted
and is excluded from closure evidence. Timing logs are ignored under
`target/local-ci-history/`.

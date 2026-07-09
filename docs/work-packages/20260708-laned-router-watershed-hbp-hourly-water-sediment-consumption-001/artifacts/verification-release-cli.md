# Release CLI Verification

Status: `COMPLETE`
Evidence mode: `Ran`

Release build:

- Command: `cargo build --release -p openwepp-runner --bins`
- Result: PASS, completed in 1m23s.

Release run:

- Command:
  `target/release/openwepp-cli-watershed --run-dir tests/fixtures/watershed/p102-sediment-active/runs --run-file case.run --output-dir /tmp/mt3_p102_release --policy compat --jobs 1 --hillslope-binary target/release/openwepp-cli-hill`
- Result: PASS, exit 0.

Binary hashes:

| Binary | Size bytes | mtime epoch | SHA-256 |
| --- | ---: | ---: | --- |
| `target/release/openwepp-cli-hill` | 10673664 | 1783577550 | `37c08e0a8038f208e50e7b4a228fe3cc63c880b5641e1c44d8cb50838518f85a` |
| `target/release/openwepp-cli-watershed` | 9137904 | 1783577555 | `13b826d601e6884ee94680a9bd995bdb736bf173de9a5075e2fbf72e90c40b32` |

Output hashes:

| Path | Size bytes | SHA-256 |
| --- | ---: | --- |
| `/tmp/mt3_p102_release/hillslope-jobs/H1/H1.hbp` | 2342 | `42b4fc68ec749150e8360579295bebf77f373fd65c882e939c4792638a5181d4` |
| `/tmp/mt3_p102_release/interchange/ebe_pw0.parquet` | 6722 | `36b8681a4c235f1e970ed2fc5cbe37c7db36fec8216bb348005e104e147f8288` |
| `/tmp/mt3_p102_release/interchange/totalwatsed3.parquet` | 31795 | `99bbc836e13654752ec225d52b2acb41d4742fbc9855c1feedfdb970bd85e999` |
| `/tmp/mt3_p102_release/interchange/chan.out.parquet` | 4503 | `38f5886d290b5079a2b2d7c2631a2e7aa158704088d0c8052dfe74e24b14561d` |
| `/tmp/mt3_p102_release/interchange/chanwb.parquet` | 6600 | `cfc34faac5a5f2d276a4701bd2a6d1f318eaa9410a53fb068ca9b735b111578b` |

Manifest spot-check:

- `runtime_selection.selected = direct-production-executor`.
- `timestep_policy.selected_lane = hourly`.
- `execution_provenance.multi_ofe_wave1_chained = true`.
- `mofe_hourly_carry.active = true`.
- `mofe_hourly_carry.substep_count = 24`.
- Required HBP hourly arrays are present.

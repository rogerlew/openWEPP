# Run Manifest

Status: complete-after-follow-ons
Evidence mode: Ran

2026-06-11 rerun root:
- `/tmp/frostval01_rerun_20260611T020951Z`

Historical original run root:
- `/tmp/frostval01/full`

Run under validation:
- `/wc1/runs/al/algebraic-radium/wepp/runs`

Target inventory:
- Single-OFE targets: `p1` through `p43` (43 total)
- MOFE excluded from rung-2 execution scope: `pw0` (17 OFEs)
- Additional non-target diagnostic run: `p1_ks0` (used only to sanity-check ksflag-off behavior on a blocked case)

Binary and identity:
- Binary: `/workdir/openWEPP/target/release/openwepp-cli-hill`
- SHA-256: `0d87032deaa62d96a941ad3aaa9d02520369a7e1f457c68a40174eaaab7868dd`
- Version probe: `openwepp-cli-hill --version` is unsupported and returns `CLIHILL-E-001`

2026-06-11 binary/build:
- Ran `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`.
- Binary used: `/home/workdir/openWEPP/target/release/openwepp-cli-hill`.

2026-06-11 invocation shape:
- On path:
  - `openwepp-cli-hill --run-dir /wc1/runs/al/algebraic-radium/wepp/runs --run-file /tmp/frostval01_rerun_20260611T020951Z/runfiles/<prefix>.toml --output-dir /tmp/frostval01_rerun_20260611T020951Z/outputs/<prefix> --policy compat`
- Off path:
  - `openwepp-cli-hill --run-dir /wc1/runs/al/algebraic-radium/wepp/runs --run-file /tmp/frostval01_rerun_20260611T020951Z/off/runfiles/<prefix>_off.toml --output-dir /tmp/frostval01_rerun_20260611T020951Z/off/outputs/<prefix> --policy compat`
  - Off wrappers use explicit `[inputs.frost] wintRed = 0` and leave source input
    files unchanged.

2026-06-11 execution census:
- Single-OFE total attempted: 43
- Single-OFE succeeded: 43
- Single-OFE blocked: 0
- Paired frost-off reruns attempted: 43
- Paired frost-off reruns succeeded: 43
- WAT outputs: 43

2026-06-11 evidence files:
- `/tmp/frostval01_rerun_20260611T020951Z/run_status.tsv`
- `/tmp/frostval01_rerun_20260611T020951Z/off/run_status.tsv`
- `/tmp/frostval01_rerun_20260611T020951Z/reports/activation_summary.csv`
- `/tmp/frostval01_rerun_20260611T020951Z/reports/frost_off_summary.csv`
- `/tmp/frostval01_rerun_20260611T020951Z/reports/frost_on_off_deltas.csv`
- `/tmp/frostval01_rerun_20260611T020951Z/reports/annual_closure_residuals.csv`
- `/tmp/frostval01_rerun_20260611T020951Z/reports/summary.json`

Historical original invocation shape:
- On path (`ksflag` from source soils, expected `1`):
  - `openwepp-cli-hill --run-dir /wc1/runs/al/algebraic-radium/wepp/runs --run-file <wrapper.toml> --output-dir /tmp/frostval01/full/manifests/<prefix> --policy compat`
- Off path (`ksflag` forced to `0` for paired runs on successful prefixes only):
  - `openwepp-cli-hill --run-dir /wc1/runs/al/algebraic-radium/wepp/runs --run-file /tmp/frostval01/full/off_runs/<prefix>.run --output-dir /tmp/frostval01/full/off_manifests/<prefix> --policy compat`

Historical original execution census:
- Single-OFE total attempted: 43
- Single-OFE succeeded: 6 (`p8`, `p13`, `p22`, `p23`, `p26`, `p28`)
- Single-OFE blocked: 37
- Paired ksflag-off reruns attempted: 6
- Paired ksflag-off reruns succeeded: 6

Historical blocked error taxonomy (`run_status.tsv`):
- 19: `HS-RUNTIME-E-062 ... layer 4 ... (1270..2000 mm, covered 530 mm)`
- 10: `HS-RUNTIME-E-062 ... layer 6 ... (1100..2000 mm, covered 700 mm)`
- 6: `HS-RUNTIME-E-062 ... layer 6 ... (1140..2000 mm, covered 660 mm)`
- 2: `HS-RUNTIME-E-062 ... layer 4 ... (760..2000 mm, covered 1040 mm)`

Historical evidence files:
- `/tmp/frostval01/full/run_status.tsv`
- `/tmp/frostval01/full/off_status.tsv`
- `/tmp/frostval01/full/success_prefixes.txt`
- `/tmp/frostval01/full/off_ksflag_checks.txt`
- `/tmp/frostval01/full/reports/activation_summary.csv`
- `/tmp/frostval01/full/reports/closure_yearly.csv`
- `/tmp/frostval01/full/reports/closure_prefix_summary.csv`
- `/tmp/frostval01/full/reports/totalwatsed3_legacy_flag_audit/daily_closure_audit_summary.json`
- `/tmp/frostval01/full/reports/totalwatsed3_subset_audit/daily_closure_audit_summary.json`

# Gate Results

Status: complete
Evidence mode: mixed (Static + Ran)

Gate summary:
- Contract authority gate: pass (diagnostics-only, no contract edits)
- Contract-test gate: pass (no new tests required)
- Pre-implementation gate: pass (typed parser blocker documented; temp input normalization bounded to `/tmp`)
- Candidate execution gate: pass (bounded 60-day lane completed)
- Defect verdict gate: pass (explicit verdict + caveats recorded)

Ran command highlights:
- `cargo run -p openwepp-runner --bin openwepp-cli-hill -- --run-dir /tmp/openwepp_mofe2637_defect_diag/runs --run-file p2637.run --output-dir /tmp/openwepp_mofe2637_defect_diag/output_meta --policy compat` (typed soil parser blocker)
- `cargo run -p openwepp-runner --bin openwepp-cli-hill -- --run-dir /tmp/openwepp_mofe2637_defect_diag/runs --run-file p2637.run --output-dir /tmp/openwepp_mofe2637_defect_diag/output_meta_60d --policy compat` (pass)
- metric extraction scripts (duckdb + Python) for baseline/candidate day-44 diagnostics.

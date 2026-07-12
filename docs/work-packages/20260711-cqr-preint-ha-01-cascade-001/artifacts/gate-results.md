# Gate Results

Evidence class: **Ran — module checkpoint**

| Gate | Result |
| --- | --- |
| Scaffold commit | PASS: `c0a75d8e` predates Rust/test edits |
| Focused tests | PASS: 7/7 twice; full-library coverage 341/341 twice |
| Science coverage / floor / obligations | PASS: production 98.104% line / 98.264% region; lowest floor 91.667%; exact map bound; no exclusion |
| CRAP at most 30 / non-target regression | PASS: target 7; all cascade rows at most 11.001; retry 1 has 66/44 and every non-target key unchanged |
| Numeric, conservation, consumer identity | PASS iteration: extraction preserves expressions/order; routing oracle/conservation tests green; real integral consumer named |
| `git diff --check` and Markdown lint | PASS: diff clean; 37 files scanned, zero errors/warnings |
| `cargo fmt --check` | PASS: exit 0 after final source edit |
| Workspace Clippy | PASS: exit 0, 11.08 s |
| Full workspace nextest | PASS: 1,777/1,777, 4 slow, 3 skipped, run `731479e7-ddd4-4620-a52b-0d5260b0a7b6`, 590.490 s |
| `cargo deny check` | PASS: exit 0, 0.94 s |
| Dual review / finding disposition | PASS: final Review A and B PASS; every finding accepted and corrected |
| Dual verification / non-deferral | NOT REQUIRED per revised module cadence; tranche-final verification owns aggregate non-deferral |

Delegated closure logs/time reports are
`/tmp/openwepp-cqr-preint-ha01-closure-{clippy,full,deny}.{log,time}`. The runner
verified source SHA-256 `574d98ab...b1d3fb` and made no tracked edit.

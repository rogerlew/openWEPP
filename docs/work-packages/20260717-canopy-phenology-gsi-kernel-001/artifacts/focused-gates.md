# Focused Gate Evidence

Evidence class: `Ran`

Terminal-current focused source was checked after all initial review findings
were dispositioned.

| Gate | Result |
| --- | --- |
| `cargo fmt --check` | PASS |
| strict package Clippy | PASS |
| package Nextest, quick profile | PASS, 13/13 across two binaries, run `61c1c47a-3aae-45c1-8f5c-4d997ca81b91` |
| roadmap Markdown lint | PASS, 0 warnings |
| phenology backlog Markdown lint | PASS, 0 warnings |
| `SC-PLANT-001` Markdown lint | PASS, 0 warnings |
| work-package Markdown lint | PASS, 16 files before final-verification artifacts, 0 warnings |
| `git diff --check` | PASS |

Line-count check: the production Rust source is 920 lines, the public restart
integration test is 45 lines, and the amended plant contract is 854 lines.
None reaches the 2,000-line warning threshold.

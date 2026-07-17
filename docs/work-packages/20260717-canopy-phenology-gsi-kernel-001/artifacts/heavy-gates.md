# Heavy Gate Evidence

Evidence class: `Ran`

The final terminal source includes the table-driven error coverage test and the
first-admission/restart vectors required by independent verification.

| Gate | Final result |
| --- | --- |
| `cargo fmt --check` | PASS |
| workspace Clippy with warnings denied | PASS |
| full-profile workspace Nextest | PASS, 2,085/2,085 across 192 binaries, five configured skips, 24 slow, 559.630 seconds, run `3776d7c5-a5b6-4cdd-908c-c3320eeca8cc` |
| `cargo deny check` | PASS: advisories, bans, licenses, sources |
| fresh adjudicated CRAP from frozen base | PASS: raw 2, adjudicated 2, actionable 0, touched actionable 0, untouched actionable 0 |

The initial fresh CRAP run correctly reported `GsiError::fmt` at cyclomatic 15,
coverage 0, and CRAP 240. The accepted correction added a test that formats all
14 typed variants. Focused LLVM coverage then executed the function 14 times
and every match arm at least once. The terminal fresh workspace measurement no
longer reports that function above threshold.

Final adjudicated-CRAP identity:

- production entries: 9,746;
- touched production file: `crates/openwepp-plant-phenology/src/lib.rs`;
- workspace CRAP input JSON SHA-256:
  `93c85e3c8c710e7460f612ee114a53dbf373bfc9bc11df94c0580c38800a01de`;
- adjudicated report JSON SHA-256:
  `5a2f6762dca30c57ba06e720a7d5b77120a6cdcf502a41c25361ff6c855d369a`;
- LCOV SHA-256:
  `694152d3359cd50134c3702603dc945338922176ea31e736498526d541f035cb`;
- production source manifest SHA-256:
  `8b446a40a7eca942f62047961910219b93c2511f2318a7deff571d7d1a2523ac`;
- adjudication registry SHA-256:
  `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`.

Line counts are 920 for the production Rust source, 45 for the public restart
integration test, and 854 for the amended contract; none reaches the 2,000-line
warning threshold.

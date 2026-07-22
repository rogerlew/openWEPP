# Coverage After

Ran: authoritative matching-module evidence at exact clean
`2c0f1b12dc91996189599afc172a189ee5661b12` is retained at
`/tmp/cqr-executor-fix-cSlvEh`.

- production lines: 1,491 / 1,604 = 92.9551%;
- production regions: 2,376 / 2,795 = 85.0089%;
- compiled current-profile functions: 79;
- per-function region floor: 79 / 79 at or above 75%;
- profile omission: only `kill_process_tree` under `cfg(non-unix)`.

Static: cargo-crap's `coverage_percent` column is line coverage and is not the
ADR-0021 per-function region-floor column. The retained TSV's
`region_percent` column is authoritative for the floor; its minimum compiled
value is at least 75%.

Evidence hashes:

- LCOV: `f1ea706cb5912549420726d2542190b13dba9530f974f96ff4e9b86508cc78e2`;
- LLVM JSON: `570308b9ad425a538363d473cf98e47468f6eb687d72a6a2e5d9a9f79245401c`;
- function-region TSV: `7b95a263cc4352aeab6252a86160c15bcf8200944d70dd3292135d276f116171`.

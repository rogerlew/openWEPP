# Gate Results

Evidence: `Ran`

| Gate | Result |
| --- | --- |
| Exact 22-target corrected-binary replay | PASS — `22/22` passed the formerly rejected processing day; 6 suspension and 16 lower-collapse branches; zero forbidden thermal errors |
| Focused EB-04C contract/runtime tests | PASS — `23/23`; run `071389f0-e888-4f20-8bdc-796f0423908e` |
| Native-SWE threshold-side helper | PASS — `1/1`; review run `2f3e6d24-d65b-47cf-a0ea-f6c835e92a8c` |
| Authority suite anti-evasion source guard | PASS |
| Required-suite obligation guard | PASS — `3/3`; run `c24d81b9-5516-4e4f-8bdd-11215c48d68b` |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| Quick workspace profile | PASS — `2119/2119`, 36 skipped; run `01fe4930-2ae5-4e3d-8ca5-17834229e474`; 2,203.877 s |
| Frost workspace profile | PASS — `325/325`, 1,884 skipped; run `739e9fd2-c95c-4370-88e5-57ee76b270ff`; 524.661 s |
| Initial Critical full workspace profile | NOT ACCEPTED — `2167/2168`, 29 skipped; one 720.104 s timeout in `receipt_preparation_is_reused_only_when_bytes_match`; run `e92f87e9-acba-496e-b32e-ce7eb054f43d` |
| Isolated timed-out test under full profile | PASS — `1/1`, 2,196 skipped; run `b3e88ffe-f6ff-4c8c-afd4-15bae21cf0a9`; 239.927 s |
| Canonical Critical full workspace rerun | PASS — `2168/2168`, 29 skipped; run `76f2a6c4-5b6c-42da-8dba-525fb2e5847d`; 2,231.015 s |
| Package and touched-roadmap Markdown lint | PASS — 34 package files plus each touched roadmap/contract/catalog file, zero errors or warnings after closure records |
| Dual independent review | PASS / PASS after all findings were corrected and dispositioned |
| Dual independent terminal verification | PASS / PASS after lifecycle findings were corrected and dispositioned |
| `git diff --check` | PASS after terminal-verification and closure records |

The initial full-profile timeout was retained rather than hidden. The same test
passed both alone under the full profile and inside the immediately following
canonical full rerun. No timeout, profile, test, or production threshold was
changed.

Dependency manifests are unchanged, so `cargo deny` is not increment-applicable.

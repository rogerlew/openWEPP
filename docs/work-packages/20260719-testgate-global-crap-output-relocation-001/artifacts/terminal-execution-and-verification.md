# Terminal Execution And Verification

Evidence class: `Ran`.

## Exact Plan

Committed range:
`c2180deb0075b1bd3b89e97c351b4dd0ada2df4d..668d42d055bb3c993d5b0054b93d8c3bf48bd5a8`.

- Intent plan ID:
  `51ef889c92dec0b34c977fd45d8dd6eada236002f8af288e78a2a8eca24b9482`.
- Terminal plan ID:
  `c2dba4293f106d8fbc66e05c629ff3c8b1fd6f117792114d5ad1df779fd985ad`.
- Two separately invoked terminal plans are byte-identical at SHA-256
  `7f59bf598a931a0a7ca03e78550f202379c9a34df7727ae9b5e998c491b06329`.
- Independent reconciliation: PASS, no added/removed paths and no risk
  escalation.
- Risk: CRITICAL; 12 nodes; 2,188 unique inventory identities.

The first reconciliation invocation omitted the policy-declared Rust
environment bindings and correctly failed reconstruction. Repeating it with the
same `CARGO_HOME`, `RUSTUP_HOME`, and `RUSTUP_TOOLCHAIN` used for planning
passed. No gate ran before successful reconciliation.

## Preserved Host-Failure Attempt

Attempt one emitted truthful FAIL receipt
`2722edb77d31091e8cdc9da5e91c94546b5cda0fa410fb2122f9c427667a17e8`:
10 nodes passed, ordinary full Nextest failed, and global CRAP was blocked.
The full log proves `ENOSPC` while assurance and planner fixtures created
scratch snapshots. The failed run already showed effective publication
concurrency four, but no result was reused.

Two prior generated TESTGATE scratch trees consumed 68 GB. They were removed
after their material findings had been committed to this package. The failed
attempt's receipt, normalized evidence, and logs were retained; only its 14 GB
`.work` scratch tree was removed. The retry used a new external artifact root.

## Passing Attempt

Attempt two receipt:
`b8a44936c154f70bea2221328cc2cedf8ce4e46f8e3b5fe2352f4bb6f325a316`.

- Result/counts: PASS; 12 passed, 0 failed, 0 blocked, 0 skipped, 0 retried.
- Receipt SHA-256:
  `0d277dea616e5377347bfbe8c5dec4c93830b9710d81cf75b1e5557ecaf75755`.
- Ordinary full Nextest: 2,170/2,170 passed, five configured skips,
  `1,644.854s`.
- Fresh LLVM-coverage Nextest: 2,170/2,170 passed, five configured skips,
  `1,905.125s`.
- Both JUnits contain 25/25 publication cases and zero failures/errors.
- Former timeout durations, ordinary/coverage respectively:
  `authority_lifecycle...` `298.731s` / `263.426s` and
  `bootstrap_narrative...` `175.256s` / `153.927s`.
- Ordinary JUnit reconstruction peaks at four concurrent publication cases.
  Coverage JUnit's millisecond-rounded timestamp/duration fields show four
  isolated nominal five-case intervals, each lasting exactly 1 ms; all
  sustained intervals and live process observations peak at four, consistent
  with eight group slots and two required per case.
- Source mutation check: required and unchanged; before/after SHA-256 both
  `d361aa1959d052941f6d191fecd06d23b2a9f85c3239af01f693c87356233dc7`.

Fresh global CRAP reports PASS, threshold `30.0`, 10,722 production entries,
2 raw over threshold, 2 adjudicated, and 0 actionable. `run-status.json` records
PASS, exit 0, fresh acquisition, and detailed-report SHA-256
`57b886a2b5b192b0481a1459f5e8b0d996422adc69bcdb71ee99257701ea7bdf`.
The retained detailed report and receipt-published CRAP artifact both hash to
that exact value.

## Independent Verification

`openwepp-gate-plan verify-receipt` independently reconstructed the plan,
inventory, DAG, artifacts, and receipt and returned PASS for the same receipt
ID and `LOCAL_UNTRUSTED` trust class. Local trust truthfully remains pending
external GitHub attestation and is not represented as hosted trust.

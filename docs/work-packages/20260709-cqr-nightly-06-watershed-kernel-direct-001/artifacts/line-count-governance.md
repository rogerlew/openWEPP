# Line Count Governance

Evidence label: Static/Ran.

Status: `PASS`

Touched Rust files:

- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct_tests.rs`

Baseline line count:

- `1780` lines.

After line count:

- `direct.rs`: `2310` lines.
- `direct_tests.rs`: `1949` lines.
- Net `direct.rs` change from baseline: `+530` lines.
- `git diff --stat -- crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs`:
  `1015 insertions(+), 485 deletions(-)`.

Disposition:

- The 3000+ non-exempt Rust file blocker identified during review is resolved.
- `direct.rs` remains above the 2000-line WARN threshold because this CQR
  package intentionally keeps the production direct-kernel include together
  while decomposing high-CRAP functions into private helpers. It is below the
  3000-line closure blocker.
- Follow-on split intent: the next line-count package for this area should split
  cohesive production direct-kernel concerns, starting with the sediment/WS20
  helper cluster, into ordered include fragments without changing formulas,
  thresholds, publication fields, or direct runtime behavior.
- `direct_tests.rs` is a package-local test include and is below the 2000-line
  WARN threshold.
- No generated/fixture exception is claimed.

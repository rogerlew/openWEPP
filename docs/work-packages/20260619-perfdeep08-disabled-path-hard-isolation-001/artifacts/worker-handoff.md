# Worker Handoff

Status: HOLD.
Evidence mode: Static/Ran.

## Final Disposition

PERFDEEP08 is in `HOLD`.

The package tested disabled diagnostic-hook caching, but H2637 default-disabled
measured `691.93 s`, RSS `229444 KB`. This is slower than the PERFDEEP07
retained `685.85 s` and above the `<= 676.67 s` P0 gate.

## What Changed

No production Rust change was retained.

Temporary changes were tried and reverted:

- cached PERFDEEP02 roundtrip env lookup;
- skipped inactive indexed-shadow thread-local hooks;
- briefly hoisted PERFDEEP02/03 flags in `scheduler.rs`, then reverted before
  timing because of line-count governance.

## Do Not Re-try First

- disabled diagnostic-hook caching, unless new profiling shows a different
  interaction;
- scheduler micro-edits in `scheduler.rs` without a line-count closure plan;
- PERFDEEP07 rejected candidates listed in
  `perfdeep08-rejected-candidates-ledger.md`.

## First Actionable Follow-up

Profile or micro-benchmark the retained default path at the PERFDEEP07 code
state. The next package should identify a concrete remaining cost center before
attempting another hard-isolation patch.

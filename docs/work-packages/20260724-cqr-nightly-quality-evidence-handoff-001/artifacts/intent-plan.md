# Intent Plan

Evidence class: Static.

## Identity And Currency

- Require exact canonical Order-3 publication and Order-4 complete control
  receipt inputs plus an expected evidence ID.
- Reuse the canonical verifier with current-source checks enabled.
- Classify source/head/tree, registry, policy, collector, workflow, toolchain,
  and verifiable profile/control changes as `STALE`. An unsupported profile
  topology that the adopted historical verifier cannot authenticate is
  `INVALID`, never current.
- Classify malformed, incomplete, unsafe, digest-inconsistent, or
  identity-internally-inconsistent evidence as `INVALID`.
- Emit `CURRENT` only after exact verification and independent row
  reconstruction.

## Selection

- Reconstruct raw, adjudicated, and actionable partitions from exact compact
  rows and the current exact-symbol registry.
- Reject summary-only evidence, duplicates, non-production paths, row drift,
  and partition mismatch.
- Rank modules by excess CRAP, unique function count, maximum CRAP, then path.
- Retain the exact candidate rows, module aggregates, source identity,
  evidence ID, and input digests; dual reviews remain required before final
  selection.
- Invoke no test execution, coverage, CRAP, or collection subprocess.
  Exact-current verification may run `cargo nextest list` and the associated
  instrumented inventory compilation through the adopted verifier.

## Recollection

- Accept only a canonical retained `STALE` or `INVALID` intake receipt.
- Require a non-empty explicit operator CQR directive.
- Bind the authorization to the receipt digest, evidence locator identity, and
  typed reasons.
- Absence of evidence remains `INVALID`; it is never silently current.

## Selected Gates

- Python compilation and tool self-test.
- Focused Rust integration contract for current/stale/invalid, selection
  parity, no-recollection, and authorization behavior.
- Existing CQR aggregate-admission contract.
- Rustfmt and warnings-denied Clippy for changed Rust tests.
- Documentation lint and exact terminal diff reconciliation.
- Dual review, finding disposition, and dual terminal verification.

No live, heavy, collection, or CQR-batch execution is selected.

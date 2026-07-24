# Authority Alignment Evidence

Evidence class: Static.

ADR-0041 is accepted and prospectively amends ADR-0021, ADR-0039, and ADR-0040.
The aligned governance now states:

- TESTGATE is the manual blocking correctness-admission workflow on forest1.
- TESTGATE records workspace coverage/CRAP as
  `DEFERRED_TO_QUALITY_CI`; that disposition is neither a pass, skip, waiver,
  nor generic campaign deferral.
- Optional observational QA is non-blocking for increment, campaign, and
  release transitions. Missing, stale, or actionable QA debt does not block
  those transitions.
- Coverage/CRAP thresholds remain binding only for an explicitly declared
  metric-focused CQR or module-test-enhancement closure surface.
- CQR Nightly remains on hold until roadmap Order 5 proves exact
  `quality_evidence_id` intake. Recollection then requires typed
  `STALE`/`INVALID` evidence and an explicit operator directive.
- Retired Omarchy queued records are immutable historical metadata and are
  ignored for forest1 occupancy; they are never awaited or canceled.
- Historical receipts and verdicts remain unchanged. An incompatible retained
  receipt receives a separate `REJECTED_INCOMPATIBLE_RECEIPT` decision.
- A complete forest1 `LOCAL_UNTRUSTED` receipt may close its admitted boundary
  with the required ledger and independent verification. Protected attestation
  is a separate optional publication claim.

Aligned surfaces include root and package governance, the canonical testing
strategy, decision records and indexes, package-authoring guides, CQR
templates, operator documentation, and the roadmap/catalog.

No planner, executor, verifier, schema, workflow, Rust, or test implementation
was changed in Order 1. Executable alignment and proof remain roadmap Order 2.

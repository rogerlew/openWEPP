# Implementation

Static: implementation commit
`dc935c7a7a6ea4a3f89c60cf08b91a54736b013b` performs only
behavior-preserving complexity decomposition:

- co-locates each command handler with its unchanged option allowlist;
- extracts ordered transition preparation, LIGHT execution/persistence, audit
  construction/persistence, and READY/non-READY completion helpers;
- introduces borrowed HEAVY transaction inputs and context, then separates
  STARTED, admission/resume, execution/verification, and CLOSED/FAILED helpers;
- separates pre-HEAVY input collection, ledger preparation, readable/invalid
  LIGHT selection, and ordinary/fallback audit construction;
- removes the obsolete `clippy::too_many_lines` allowance; and
- mechanically updates two source-contract strings for the relocated ledger and
  started-entry bindings.

No option, JSON field, error code, process exit rule, ledger field, timer
boundary, or side-effect order changed.

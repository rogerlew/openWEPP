# Implementation

Status: EXECUTED-HOLD-NO-CODE. Evidence mode: Static.

No production implementation changes landed.

Reason: D16-S2 found a promotion blocker before contract/code edits. The
hybrid explicit opt-in path is faster and closes all active ledgers, but the
plain-vs-hybrid H2637 publication deltas do not have a ratified
default-promotion tolerance. Per the package acceptance criteria, no partial
selector flip was made.

Current runtime semantics remain unchanged:

- `OPENWEPP_LANED_ACTIVE=1` selects the active Lane-D owner.
- `OPENWEPP_LANED_ACTIVE_IMPLICIT=1` selects the hybrid stepper.
- Implicit unset preserves plain rev-27 active behavior.

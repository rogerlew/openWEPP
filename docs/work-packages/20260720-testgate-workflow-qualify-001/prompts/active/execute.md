# Execute TESTGATE Forest1 Workflow Qualification

Scope: execute one ordinary exact-head TESTGATE workflow on forest1. Do not
create a synthetic matrix, edit TESTGATE implementation, or dispatch more than
once.

Task: commit and push the ready documentation increment, prove the TESTGATE
queue idle, dispatch this active package once against the pushed exact head, and
record the forest1 receipt, ledger, artifacts, and result. `LOCAL_UNTRUSTED` is
the normal forest1 local receipt classification, not a failure.

Subagent authorization: this package explicitly authorizes two read-only result
reviewers and two read-only terminal verifiers. No subagent may dispatch, push,
or edit TESTGATE.

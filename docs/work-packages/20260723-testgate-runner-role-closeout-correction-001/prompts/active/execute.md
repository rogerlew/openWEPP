# Execute TESTGATE Runner Role And Closeout Correction

Scope: local openWEPP documentation and policy-digest correction only.

Read and follow `AGENTS.md`, `docs/work-packages/AGENTS.md`,
`docs/standards/AGENTS.md`, `docs/standards/testing-and-gate-strategy.md`, and
this package.

Correct the durable closeout record to state that the retired pre-pivot
Omarchy runner is defunct, forest1 is the active self-hosted TESTGATE HEAVY
runner, and GitHub-hosted jobs perform bounded verification and attestation
without executing HEAVY. Record that automatic run `30002884134` ran on
forest1 and was canceled during content-gate execution. Preserve the local
receipt's `LOCAL_UNTRUSTED` label and distinguish engineering-package closeout
from repository certification.

Do not manually dispatch TESTGATE or rerun an unchanged expensive gate.

Subagent authorization: this prompt explicitly authorizes two independent
read-only reviewers for the corrected documentation claims. Expected output is
concise findings returned to the parent. Reviewers may not edit, commit, push,
dispatch, or execute a gate.

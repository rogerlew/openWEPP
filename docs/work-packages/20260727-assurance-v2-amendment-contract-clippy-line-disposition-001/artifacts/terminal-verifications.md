# Terminal Verifications

Status: `PASS`

Exact subject: `d6465fc7d5b207a021bc4cd518e763052a0c82c5`

## Verifier A

Evidence class: `Static + Ran`

Verdict: `PASS`

No findings. Verifier A proved the assurance Rust diff is exactly the rationale
plus function-scoped allowance, removal reproduces authority-base bytes,
1,046 lines is below both thresholds, later changes are separately authorized,
all retained gates including full 2,361/2,361 pass, no correctness obligation
is deferred, protected paths are absent, diff hygiene passes, and both packages
lint cleanly.

## Verifier B

Evidence class: `Static + retained Ran evidence`

Verdict: `PASS`

No findings. Verifier B independently reconciled all 33 changed paths through
the assurance and bound-ledger package authorities, reproduced the base file
SHA-256 `e43a24d564ee790af69b59a6236b5655de7dbdbeb1dc3c456b6494e21012ece0`
after removing the two lines, confirmed unchanged behavior/reviews/findings,
all focused/supporting/full gates, line-count governance, protected boundaries,
and the legitimate ADR-0041-only quality disposition.

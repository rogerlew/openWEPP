# Receipt Verifications

Status: `PASS`

Evidence subject: repository evidence head
`35d2de0576020df366fe78f9a5d7f909203f4a98`; canonical execution subject
`10d2c2004d7c85a72a5cae7d73c2b571f245960f`.

## Verifier A

Evidence class: `Static + Ran`

Verdict: `PASS`

No findings. Verifier A recomputed every recorded file hash, confirmed the
intent-to-terminal binding, READY package admission, LIGHT PASS, 10/10 audit,
HEAVY PASS, 12-node/2,387-item equality, unchanged source digest, no
retry/fallback, full 2,361/2,361 evidence, and closure-eligible ADR-0041
disposition. It independently recomputed all four ledger entry hashes and
predecessor links.

## Verifier B

Evidence class: `Static + Ran`

Verdict: `PASS`

No findings. Verifier B independently matched all IDs and hashes, parsed and
rehashed the balanced ledger, proved planned/executed inventory lists are
identical and unique, confirmed every node used attempt 1 with no retry reason,
and verified the full-profile log/JUnit and unchanged source mutation digest.
No attempt argument accessed CAL or Harvard.

Both verifiers found `artifacts/canonical-execution.md` truthful.

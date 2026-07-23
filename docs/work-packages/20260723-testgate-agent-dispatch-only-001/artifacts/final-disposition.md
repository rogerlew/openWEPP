# Final Disposition

Evidence classes: Static + Ran.

Disposition: `COMPLETE`.

Static: TESTGATE is now agent-dispatch-only. A push to `main` does not match
the protected workflow. Explicit dispatch requires an exact lowercase
40-character active scaffold commit as `base_ref` and an exact
`intent_package`; both forest1 execution and hosted verification independently
validate the base before using it.

Static: forest1 remains the only HEAVY executor. GitHub-hosted jobs retain
bounded independent verification and attestation roles. Current-main checks,
the permanent concurrency identity, fail-closed aggregate, attestation
identity, and receipt-trust posture are unchanged.

Ran: exact correction commit
`5b287d523408e5a45b5a689326c19e18fc32ad11` passed focused workflow and policy
contracts, YAML and JSON parsing, Markdown lint, formatting, diff hygiene,
canonical package admission, dual implementation review, and dual terminal
verification. `TGD-001` is closed.

Ran: closure-documentation Markdown lint passed 11 files with zero errors or
warnings, and `git diff --check` passed.

Static: no TESTGATE was dispatched and no expensive gate was executed during
the correction. The remaining operational proof is the final push observation:
the push must create no TESTGATE run.

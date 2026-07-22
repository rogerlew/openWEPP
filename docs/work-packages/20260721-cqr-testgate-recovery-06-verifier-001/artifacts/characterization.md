# Characterization

Static: dual eligibility review passed and the exact baseline completed.

Characterization will use the declared split test module and the existing
normalized valid plan/receipt fixture. It must bind the public READY-audit path
in exact order: identity, live execution context, HEAVY admission, then full
receipt verification. It will also directly bind the remaining eligible floor
gaps without changing receipt schemas, errors, or production behavior.

Ran: `verifier::tests::coverage_tests` binds:

- exact successful `ReceiptVerdict` identity, result, and trust accessors;
- identity rejection before context/admission;
- exact live-context error code and message;
- exact missing-HEAVY error code and message;
- downstream source-tree rejection after valid admission;
- retry, prerequisite, HEAVY-audit, envelope-artifact, equality, and public
  verdict-accessor edges required by the baseline floor gaps.

Ran: after dual-review correction, the exact public path uses a TERMINAL plan,
a non-null READY audit bound to the exact plan/package/artifact root, and
`validate_audit_for_execution` immediately before receipt verification. It
compares the complete ten-field `ReceiptVerdict` and exact stable messages for
all four ordered rejection stages. The focused clean-head test passed at
`223b034e` in 358.56 seconds; it and every other library test passed in the
authoritative corrected-head measurement.

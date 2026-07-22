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

Ran: the exact public-path test passed at clean HEAD `9970ac32` in 314.72
seconds. The same test and all other library tests passed in the authoritative
changed-head metric traversal.

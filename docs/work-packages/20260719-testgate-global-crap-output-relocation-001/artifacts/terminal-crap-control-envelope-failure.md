# Terminal CRAP Control-Envelope Failure

Ran: exact committed terminal plan `51e3a084ed5ba2d06811c3cbfabc74d8b3c531e9cbe282c5adb7a74cae39f44d`
from base `0427e78c304582730c991beac054c931e9dccfe3` to head
`db313ce7d97e302618cac6a855b4068ac2398148`.

- Independent double-plan bytes and reconstruction passed before execution.
- The normal full-workspace node completed and wrote its normalized JUnit
  artifact.
- Fresh CRAP acquisition completed. The adapter reported
  `status=PASS raw=2 adjudicated=2 actionable=0` and wrote its complete report,
  LCOV, JUnit, source manifests, and checksum manifest beneath the external
  executor artifact root.
- The executor then failed with `GATE-JSON-INVALID: floating-point JSON is
  outside gate-policy/v1 at line 17 column 21` while reading
  `adjudicated-crap-report.json`; line 17 is `"coverage": 0.0`.
- No terminal receipt was emitted. The failed execution root is retained at
  `/tmp/tgGO-env.tIsU0D` for the current session.

Disposition: reproduced integrated tooling defect
`TESTGATE-CRAP-CONTROL-ENVELOPE-01`. Preserve the detailed numeric report and
move executor PASS validation to a strict status envelope that binds the exact
report SHA-256. Do not resume this stale plan.

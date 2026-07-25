# Gate Evidence

Evidence class: Ran.

Implementation head:
`c87adad3bf37309739352399ddbcf0cdf5e6ed4a`.

| Gate | Result |
| --- | --- |
| Python compilation and both tool self-tests | PASS |
| CQR quality-evidence handoff contract | 4/4 PASS |
| Combined handoff/observatory/TESTGATE/authority bundle | 39/39 PASS |
| CQR aggregate-admission unittest | 17/17 PASS |
| Inherited failed TESTGATE contract after digest repair | PASS |
| TESTGATE authority contract | 11/11 PASS |
| Rustfmt | PASS |
| Warnings-denied Clippy | PASS |
| Package/operator/template Markdown lint | PASS |
| Base-to-head and terminal diff hygiene | PASS |

The real CURRENT/INVALID/STALE fixture contract took about 158 seconds because
it performed two independent inventory enumerations. No test execution,
coverage/CRAP collection, live workflow, CQR batch, or heavy gate ran.

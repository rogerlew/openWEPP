# Planned Versus Executed Gates

Evidence class: `Ran` for executed commands and `Static` for applicability.

| Prospective gate | Terminal status | Executions | Disposition |
| --- | --- | ---: | --- |
| Instruction/path and initial diff hygiene | PASS | 2 | First run retained the seeded failure; only this invalidated gate was rerun after cause repair. |
| Planner library target | FAIL | 1 | 36 passed, 8 failed with `GATE-POLICY-DIGEST-DRIFT`, 18 not run; 20.16 seconds. |
| Three focused TESTGATE integration targets | BLOCKED | 0 | Failed blocking-policy admission made acceptance impossible; policy repair is outside scope. |
| Changed-document Markdown lint | PASS | 1 | Nine executor-authored Markdown files passed with zero errors/warnings. |
| Local committed TESTGATE execution | BLOCKED | 0 | Cannot generate an admitted terminal plan or verified local receipt while policy currency fails. |
| Live normal TESTGATE | NOT RUN | 0 | Parent-owned later boundary; this failed local candidate must not be pushed as acceptance authority. |

Unauthorized gates executed: zero. Successful gates redundantly rerun: zero.
The one hygiene rerun was required by its observed injected failure. Workspace
Nextest, Clippy, coverage, CRAP, cargo-deny, campaign, release, manual dispatch,
and soak gates were not run.

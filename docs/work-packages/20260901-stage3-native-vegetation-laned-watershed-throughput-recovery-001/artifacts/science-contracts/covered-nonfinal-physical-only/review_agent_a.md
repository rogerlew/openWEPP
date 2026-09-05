# Independent correctness review A

Static:

Reviewer: `rust_code_reviewer`

Verdict: `HOLD`

| Finding | Severity | Summary | Required disposition |
|---|---|---|---|
| `CPH-A-001` | high | Final-envelope construction was conflated with parent publication. | Separate private publishable-envelope construction from zero map-level publication and one accepted-parent publication. |
| `CPH-A-002` | high | The `M-1/1` count rule did not define pre-final and final failure paths. | Define separate charge, validated-endpoint, final-attempt, completed-envelope, and parent-publication counters for success and failure. |
| `CPH-A-003` | high | Expected red was an evadable symbol-presence test and omitted `OBL-VEGTRANSACTION-P-005`. | Add behavioral counter, differential, privacy, poison, and rollback tests before production implementation. |
| `CPH-A-004` | medium | LSE “every posture” wording could authorize snow-free WB14 work in the native represented-snow regime. | Make role and regime dispatch orthogonal; native retains inactive litter/WB14 custody without executing it. |
| `CPH-A-005` | medium | New role/custody triggers lacked exact typed-error mapping and precedence; `ERR-CT-021` was semantically wrong. | Add canonical trigger-to-error mappings and deterministic precedence. |

Positive findings: IDs were noncolliding; versions/index/BEI were internally
consistent at review time; intended behavior preserved ADR-0044, the eight-map
ceiling, no fallback, and no process-solver V58.

The full reviewer output was returned in the collaboration record for task
`/root/covered_physical_contract_review_a`.

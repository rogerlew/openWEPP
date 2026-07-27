# Acceptance Matrix

Status: `PROSPECTIVE`

Evidence class: `Static`

| Case | Required result |
|---|---|
| Exact LIGHT receipt and clean external root | READY may be constructed |
| Caller-authored or mutated READY JSON | reject |
| Standalone HEAVY invocation | reject |
| Ledger append after audited head | append balanced failure; reject execution |
| HEAVY preflight failure | one STARTED and one typed terminal record |
| Source checkout mutation | INVALID |
| Symlink, output-root escape, or nonregular output | INVALID |
| Hardlink, attempt-root replacement, directory/file race, or pre-existing nested output | INVALID |
| Undeclared external output | INVALID |
| Output mutation between inventory and hashing | INVALID |
| Partial or interrupted publication | rollback/no destination change; typed failure |
| Publication collision or destination drift | reject before install |
| Missing or changed declared output | FAIL/INVALID by typed cause |
| Prerequisite receipt/output drift | reject dependent node |
| First transition audit reused after verifier handoff | reject |
| Fresh second transition with exact freeze/verifier receipts | holdout may be admitted |
| CAL-04B Harvard token absent before barrier | holdout forbidden |
| Duplicate verifier ID, same actor, executor-produced receipt, forged/replayed receipt, or wrong freeze digest | reject |
| Opening token pre-exists or races exclusive creation | reject without Harvard read |
| BLOCKED/INVALID/audit/pre-spawn failure | no opening token and no Harvard read |
| Failure after `OPENED_ONCE` | retain failure; prohibit retry |

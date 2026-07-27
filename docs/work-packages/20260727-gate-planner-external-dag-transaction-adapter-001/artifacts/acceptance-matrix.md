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
| Undeclared external output | INVALID |
| Missing or changed declared output | FAIL/INVALID by typed cause |
| Prerequisite receipt/output drift | reject dependent node |
| First transition audit reused after verifier handoff | reject |
| Fresh second transition with exact freeze/verifier receipts | holdout may be admitted |
| CAL-04B Harvard token absent before barrier | holdout forbidden |


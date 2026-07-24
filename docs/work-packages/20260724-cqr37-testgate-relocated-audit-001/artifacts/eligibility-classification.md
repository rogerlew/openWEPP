# Eligibility Classification

Status: `PASS`

Static:

| Symbol | Source SHA-256 | Class | Aggregate | Function floor | CRAP |
| --- | --- | --- | --- | --- | --- |
| `validate_relocated_audit` | `bb9799048e5d75c0d28ef8d3859d5ca65df40012f65ea14ed1952f5302456674` | `E-PRODUCTION` | included | required | required |

Rationale: this hand-authored public verifier validates package admission,
sealed artifact identity, policy fields, readiness, embedded LIGHT evidence,
and current inventory. It affects trust-boundary acceptance and is not eligible
for an exception.

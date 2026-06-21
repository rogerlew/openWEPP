# Endpoint RSS Evidence

Status: blocked.
Evidence mode: Static + Ran.

## Required During Completion

Record endpoint and RSS evidence for:

- default-disabled compatibility path;
- R5E accepted direct endpoint baseline;
- R6 direct-publication candidate;
- any shadow or rollback mode used for validation.

Any regression against the accepted R5E baseline must be dispositioned before
closure.

## Current Evidence

Ran:

- `cargo run -p openwepp-runner --bin openwepp-cli-hill -- ... --direct-publication-frame-cutover`

Result: fail-closed before endpoint completion with
`R6-DIRECT-PUBLICATION-PARITY HBP byte identity failed`.

No RSS or timing metric is meaningful for R6 acceptance while the candidate
does not produce valid public outputs.

## Gate

BLOCKED. R6 completion still requires endpoint/RSS evidence after HBP, WAT,
PASS, loss, and manifest cutover gates pass.

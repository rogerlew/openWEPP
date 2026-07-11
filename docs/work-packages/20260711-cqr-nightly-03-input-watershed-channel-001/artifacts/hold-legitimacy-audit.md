# Hold Legitimacy Audit

Status: PASS — legitimate local target hold.

## Blocker

Static: `SC-INFILE-WATERSHED-CHANNEL-001` says `rating_curve_line` is required
iff `icntrl == 4`, assigns missing/extra rating records to `CHN-E-006`, and maps
conditional arity guard `G-CHN-013` to that error. Scaffold source
`a7d07708` consumes no rating row when `icntrl != 4`, then classifies every
remaining non-empty row as `RecordClosure { context: "extra_records" }` /
`CHN-E-002`. The mismatch predates this package.

## In-Envelope Route Considered

The package added characterization and passed glue-tier module percentages, but
decomposed before closing the per-function floor. Review repaired that floor in
the later provisional suite and broadened obligation coverage, which exposed the
extra-row mismatch. No behavior-preserving extraction or test can turn current
`CHN-E-002` into required `CHN-E-006`; that is an observable public error-
contract change explicitly outside the package write/authority envelope.

## Rollback Proof

Required rollback is complete. `git diff --quiet a7d07708 --` on the target and
focused test exits `0`. SHA-256 values are:

- target: `e0b04f88051fd7030446313bb2a853fe4d08100b4c2911c8845bed757f28d154`;
- test: `46b18d420e23a3b83709e3e5107d4d931b559b33d99c05c928380faa2bd91fb8`.

Dual independent reviewers accepted the blocker as target-local and required
rollback. The first actionable defect-closure follow-on is specified in
`worker-handoff.md`.

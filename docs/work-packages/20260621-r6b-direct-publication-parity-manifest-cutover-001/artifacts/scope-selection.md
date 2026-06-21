# Scope Selection

Status: executed-hold.
Evidence mode: Static + Ran.

## Selected Scope

R6B is selected to close all five worker-handoff items in one coherent package:

1. Parity-grade direct frame population.
2. Anti-alias fixtures for HBP/WAT/PASS/loss/manifest provenance.
3. Independent reconstruction for accepted HBP/WAT/PASS/loss operands.
4. Direct manifest production provenance/checksum cutover.
5. Cutover reruns plus default-disabled and endpoint/RSS benchmarks.

## Not Selected

- Default activation of direct publication.
- R7 compatibility-runtime deletion.
- Output schema redesign.
- Process-physics changes.
- Diagnostic-only or producer-only packages.

## Execution Boundary

The selected scope remained current-scope acceptance. It did not shrink after
execution started.

Execution stopped at item 1 because the production candidate lacks the typed
operand bridge required to populate `DirectRunPublicationFrame` with
parity-grade data. Items 2-5 are therefore blocked, not deferred as complete
follow-on work.

# Publication Ledger Promotion Plan

Status: complete for planning-only scope.
Evidence mode: Static.

## Seed Authority

`docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/perfdeep06-publication-operand-ledger.md`
is the seed ledger for future publication projection work.

It is package evidence, not canonical authority. Before R6 cutover or any
package that changes publication operands, the ledger must be promoted into one
of:

- this architecture specification;
- a canonical publication/output contract;
- a new ADR or contract-backed package artifact explicitly referenced by the
  architecture specification.

## Required Promotion Content

The promoted ledger must include:

- output operand;
- units and basis;
- source direct-frame field;
- producer phase;
- legacy symbol alias;
- row/column destination;
- wrong aliases to reject;
- anti-alias fixture;
- metadata/provenance parity;
- independent operand reconstruction requirement;
- closure or magnitude audit requirement when conservation-sensitive.

## Current R1 Disposition

R1 may plan constructors/projections and shadow expected publication operands.
It must not cut over HBP/WAT/PASS/loss/manifest construction to direct-frame
projection until promotion and fixtures are complete.

## Gate

PASS for planning-only scope.

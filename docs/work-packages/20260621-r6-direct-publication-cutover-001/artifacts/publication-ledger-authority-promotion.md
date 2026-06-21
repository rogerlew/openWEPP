# Publication Ledger Authority Promotion

Status: complete.
Evidence mode: Static + Ran.

## Requirement

Production output cutover may not begin until the PERFDEEP06 publication
operand ledger is promoted into canonical architecture or contract authority, or
is superseded by an equivalent canonical ledger.

Seed ledger:
`docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/perfdeep06-publication-operand-ledger.md`.

R0/R1 promotion rule:
`docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/publication-ledger-promotion-plan.md`.

## Required Canonical Fields

The promoted ledger must include, for every in-scope output field:

- output family and field;
- units and basis;
- normalization or denominator;
- area or volume basis;
- source direct-frame field;
- producer phase;
- legacy symbol alias;
- output row/column destination;
- wrong aliases to reject;
- anti-alias fixture;
- metadata/provenance parity requirement;
- independent reconstruction requirement;
- closure or magnitude audit requirement when conservation-sensitive.

## Promoted Authority

R6 promoted the seed ledger into
`docs/architecture/array-native-runtime-specification.md` section
`5.2.1 R6 Canonical Publication Operand Ledger`.

The promoted ledger covers:

- HBP `peakro`, `watdur`, `tdet`, `tdep`, and `sedcon_1..5`;
- WAT `P`, `RM`, `Q`, `QOFE`, `Ep`, `Es`, `Er`, `Dp`, `UpStrmQ`, `SubRIn`,
  `latqcc`, `Tile`, storage/snow/frost/profile/interception fields;
- PASS `runvol`, `sbrunv`, `peakro`, `tdet`, `tdep`, and `sedcon_1..5`;
- loss JSON run/climate/static sidecar fields;
- run manifest checksums, provenance, counters, warnings, output policy, and
  metadata;
- row identity and metadata fields.

The spec also binds the critical R6 rule: constructing a direct-named object
from compatibility WB13 rows or runtime surfaces is an adapter wrapper, not
direct publication cutover.

## Gate

PASS for ledger promotion. Production output cutover remains blocked by the
missing run-bound direct publication frame.

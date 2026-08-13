# V4 State Schema and Migration

Status: `selected`

Evidence mode: `Static`

The exact V4 shared-stratum field set is bound in the V8 contract amendment and
the V4 definition. Every tissue/subpool elemental operand, standing-dead
operand, pending transfer, timer, GSI value, T10, derived area, and transaction
identity enters canonical whole-state serialization and digest.

Displayed leaf C alone owns LAI, and displayed leaf N divided by positive LAI
alone owns FvCB/Atkin leaf-area N. Storage/transfer C/N is non-displayed and
contributes no area or leaf capacity before accepted credit to display.

The V4 field set removes exactly:

- `previous_leaf_offset_flux`;
- `previous_root_offset_flux`.

V3-to-V4 migration validates the complete V3 source, copies every retained
field and occupancy lane exactly, removes only those two fields, verifies V3
area caches against displayed-leaf reconstruction, binds supplied V4 model and
configuration identity, and recomputes the V4 digest. It never averages,
defaults, synthesizes, or partially returns state.

The independent fixture includes a two-stratum/two-occupancy source/output pair and poisons for
retaining either removed field, changing any retained field, summing leaf
subpools, summing leaf N for capacity, repairing bad caches, or partially
returning a multi-stratum invalid migration.

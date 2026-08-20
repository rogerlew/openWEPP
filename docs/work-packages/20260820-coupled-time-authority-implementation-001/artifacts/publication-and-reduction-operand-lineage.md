# Publication And Reduction Operand Lineage.md

Status: authority candidate

Evidence mode: Static

| Operand | Units | Support/source | Admission |
| --- | --- | --- | --- |
| slab diagnostic | contract-declared scalar | exact accepted slab + owner | accepted slabs only |
| peak | same as diagnostic | maximum over all accepted slabs across restart | parent candidate only |
| scheduled output | contract-declared | exact named boundary receipt | exact-once |
| publication buffer | typed records | accepted chronology | invisible before commit |

Rejected aliases: accepted+rejected maximum; parent volume divided by nominal
duration; pre- or post-restart-only peak; duplicate scheduled output; precommit
publication; publication retained after parent rollback. Acceptance requires a
separately written reconstruction plus ordering/rollback proof.

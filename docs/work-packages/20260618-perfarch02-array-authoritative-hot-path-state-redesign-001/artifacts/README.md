# Artifacts

Status: executed 2026-06-18.

Disposition: CONDITIONAL GO to an integrated WB11 array-authoritative pilot.

Deliverables:

- `perfarch02-redesign-shape.md`
- `perfarch02-floor-prototype.md`
- `perfarch02-floor-prototype.tsv`
- `perfarch02-floor-measurement.md`
- `perfarch02-contract-blast-radius.md`
- `perfarch02-staged-migration-plan.md`
- `perfarch02-proposed-adr.md`
- `perfarch02_disposition.md`
- `perfarch02-gate-results.md`

Key result:

- The artifact-local prototype measured the array-authoritative
  writeback/guard surface at about 49.9x faster than the current logical
  writeback/guard path, with exact exported-map identity on the prototyped flow.
- This supports a downstream integrated WB11 pilot and rejects further
  read-mirror/id-table-only work as the next performance lever.
- It does not prove the full H2637 ratio by itself; 5x remains unproven.

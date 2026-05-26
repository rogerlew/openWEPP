# EROD16 Contract Authority Amendment Log

Status: complete
Evidence mode: static+ran
Date: 2026-05-26

## Static
- Contract authority amendments landed in:
  - `docs/specifications/science-contracts/contracts/SC-SED-001.md`
  - `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
  - `docs/specifications/science-contracts/index.md`
- Scope: canonical documentation only; no production kernel/runtime code edits.

## Ran
- `nl -ba /workdir/wepp-forest_260430_baseline/src/contin.for | sed -n '1190,1240p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/route.for | sed -n '1,260p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/route.for | sed -n '260,620p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/xcrit.for | sed -n '1,220p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/depc.for | sed -n '1,220p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/depend.for | sed -n '1,220p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/depos.for | sed -n '1,220p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/erod.for | sed -n '1,260p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/enrich.for | sed -n '1,220p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/rtpart.for | sed -n '1,180p'`
- `rg -n "call\s+rtpart|rtpart\(" /workdir/wepp-forest_260430_baseline/src/grow.for /workdir/wepp-forest_260430_baseline/src/*.for`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/grow.for | sed -n '590,660p'`

## Amendment Summary
1. `SC-SED-001`
- Updated front matter (`contract_version: 12`, `last_reviewed: 2026-05-26`).
- Added baseline anchors for `CONTIN -> ROUTE` lineage and companion routines:
  `xcrit`, `depc`, `depend`, `depos`, `enrich`, plus provenance correction
  anchor for `rtpart`/`grow`.
- Added EROD16 addendum defining canonical route-branch invariants and alias
  continuity requirements.
- Added `GAP-SED-005` as non-promotable until downstream runtime migration.
- Added revision-history row version `12`.

2. `SC-ROUTE-001`
- Updated front matter (`contract_version: 13`, `last_reviewed: 2026-05-26`).
- Clarified out-of-scope ownership: hillslope `CONTIN -> ROUTE` belongs to
  `SC-SED-001`.
- Added provenance anchors for scope partition (`CONTIN/ROUTE`) and
  `rtpart/grow` correction.
- Added EROD16 scope-partition addendum and closed `GAP-ROUTE-007`.
- Added revision-history row version `13`.

3. `science-contracts/index.md`
- Updated `SC-SED-001` and `SC-ROUTE-001` `last_reviewed` fields to
  `2026-05-26`.
- Added EROD16 notes describing scope partition and provenance correction.

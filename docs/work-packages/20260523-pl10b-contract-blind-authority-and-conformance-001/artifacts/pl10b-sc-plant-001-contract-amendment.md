# PL10b SC-PLANT-001 Contract Amendment

Status: `complete`
Evidence mode: `Static`

## Amendment Summary

`SC-PLANT-001` was amended to add contract-authoritative transition-control
runtime-projection semantics required before PL11 implementation.

## Applied Changes

1. Front-matter update:
   - `contract_version: 4`
   - `last_reviewed: 2026-05-23`
2. Added legacy/procedure authority anchors for PL transition controls:
   - `infile.for`, `tilage.for`, `cutgrz.for`, `ptgrp.for`, `ptgra.for`,
     `decomp.for`, `inidat.for`, plus `SC-INFILE-MANAGEMENT-001` domains.
3. Added kernel-profile-required algorithm sections:
   - `Algorithm State Surfaces (PL Transition-Control Runtime Projection)`
   - `Algorithm Specification (PL10b Transition-Control Authority)`
   - `Branch and Guard Table (Transition Controls)`
4. Added transition-control invariant family:
   - `INV-PLANT-011..015`
5. Expanded guard map and boundary disposition for transition-control closures.
6. Expanded symbol alias map to deterministic projected families for:
   - annual extension controls (`jdherb/jdburn/jdslge/jdcut/jdmove`, fractions)
   - perennial indexed arrays (`cutday`, `gday/gend`, grazing payload arrays)
7. Added constants/parameters table and test-vector obligations.
8. Updated `docs/specifications/science-contracts/index.md` registry entry for
   `SC-PLANT-001` with PL10b amendment note and review date.

## Key Contract Anchors

- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md:109`
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md:173`
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md:202`
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md:343`
- `docs/specifications/science-contracts/index.md:43`

## Scope-Compliance Note

This PL10b amendment is authority-first and does not implement PL kinetics or
projection code paths; it defines completion gates and test-vector obligations
for PL11 execution.

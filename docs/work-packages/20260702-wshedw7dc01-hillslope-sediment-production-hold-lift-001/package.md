# WSHED-W7DC01 Hillslope Sediment Production Hold Lift

Status: `queued`

Date opened: `2026-07-02`

Package type: defect-closure, science-contract, hillslope sediment production,
and W7 hold-lift package.

## Objective

Close defect `WSHED-W7-HOLD-001`: current production openWEPP hillslope
execution can enable EROD14 Wave-2 on real multi-OFE source substrates while
still publishing zero HBP sediment (`tdet`, `tdep`, `sedcon_*`) for every
inspected candidate, preventing WSHED-W7 from adopting a sediment-active
watershed fixture.

## Rationale

WSHED-W7 found that committed watershed fixtures remain zero-sediment and that
the local sediment-active legacy substrate
`/wc1/runs/in/insensible-aliquot/wepp` has nonzero legacy soil-loss output but
current openWEPP production HBP pass parquet remains zero even when
`erod14_wave2_enabled = true`. W7 correctly held because it excludes changing
hillslope erosion/sediment physics for convenience.

## Correction Authority Envelope

In scope:

- Diagnose and correct production hillslope sediment emission for real
  multi-OFE openWEPP runs.
- Canonical `SC-SED-*` or existing sediment/erosion contract authority and
  contract-derived tests before production physics changes.
- Direct EROD14/EROD15 producer, publication, HBP pass emission, and manifest
  lineage required to produce nonzero pass sediment when authoritative physics
  produces it.
- A small committed sediment-active hillslope or watershed fixture needed to
  prove nonzero production after the correction.

Out of scope:

- Watershed publication aliasing, channel-balance W8 operands, W9 `NoEvent`
  authority, W10 `chan.inp` authority, and broad watershed CQR maintenance.
- Surrogate, proxy, empirical stand-in, or fixture-only sediment values.

## First Actionable Item

Close defect `WSHED-W7-HOLD-001` by identifying why production direct hillslope
execution emits zero HBP sediment for multi-OFE source substrates where EROD14
is enabled, then land the contract-backed correction or hold for missing
canonical authority with a named boundary.

## Required Evidence

- Contract-first authority for any changed sediment, erosion, or HBP emission
  semantics.
- Before/after pass parquet evidence on at least one real multi-OFE source
  substrate.
- No-surrogate-physics source review.
- Focused hillslope tests and the full Rust closure loop before complete
  disposition.
- Handoff back to WSHED-W7 with a committed sediment-active fixture candidate.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to `rust_code_reviewer`, `rust_qa_reviewer`, `science_contract_reviewer`, and
`comparator_suite_runner` subagents for sediment production review,
verification, and comparator evidence. Expected outputs are compact findings and
artifact paths. Review/verification roles are read-only; implementation remains
in the parent unless a worker is explicitly assigned a bounded write set.

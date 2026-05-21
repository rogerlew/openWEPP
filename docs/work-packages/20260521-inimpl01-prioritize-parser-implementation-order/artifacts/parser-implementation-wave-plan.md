# Parser Implementation Wave Plan

Date: 2026-05-21
Evidence mode: `Static`

## Wave Overview

| Wave | Scope | Goal | Blocker policy |
| --- | --- | --- | --- |
| Wave 1 | Hillslope core parsers (`.slp`, `.sol`, `.cli`, `.man`) | Establish minimum runnable Tier-A parser path. | Any unresolved high-severity correctness gap blocks Wave 2. |
| Wave 2 | Hillslope extension sidecars (`wepp_ui`, `pmetpara`, `snow`, `frost`, irrigation sidecars) | Extend legacy compatibility around Tier-A hillslope flows. | Wave 3 cannot start until Wave-2 is `GO`. |
| Wave 3 | Watershed core parsers (`.str`, `.chn`, `.imp`) | Establish watershed topology and channel namespace readiness. | Wave 4 blocked until Wave-3 GO. |
| Wave 4 | Watershed sidecar extensions (`chan.inp`, `tc`, `tcr`, `lcwb`, `gwcoeff`, `phosphorus`) | Complete watershed-sidecar compatibility envelope. | Promotion requires explicit disposition of remaining `*-GAP-*` risks. |

## Wave 1 (Tier-A MVP)

Ordered sequence:
1. `SC-INFILE-SLOPE-001`
2. `SC-INFILE-SOIL-001`
3. `SC-INFILE-CLIMATE-001`
4. `SC-INFILE-MANAGEMENT-001`

Acceptance checks:
- Parse-time invariants and guard map executed for each surface.
- Cross-file closures hold for `nofe`/profile/section counts.
- A first end-to-end single OFE + daily-water-balance parser ingest can be run
  with typed errors and observability events.

## Wave 2 (Tier-A Extension)

Surfaces:
- `SC-INFILE-WEPPUI-001`
- `SC-INFILE-PMETPARA-001`
- `SC-INFILE-SNOW-001`
- `SC-INFILE-FROST-001`
- `SC-INFILE-IRRIGATION-FIXEDDATE-001`
- `SC-INFILE-IRRIGATION-DEPLETION-001`

Cluster strategy:
- Climate-coupled: `snow`, `frost`
- Management-coupled: `pmetpara`, fixed-date irrigation, depletion irrigation
- Policy/sentinel: `wepp_ui`

Acceptance checks:
- Strict/compatibility mode behavior explicitly tested for each sidecar.
- No silent coercion on invalid required records.
- Tier-A parser ingestion remains stable with sidecars present/absent.

## Wave 3 (Watershed Core)

Ordered sequence:
1. `SC-INFILE-WATERSHED-STRUCTURE-001`
2. `SC-INFILE-WATERSHED-CHANNEL-001`
3. `SC-INFILE-WATERSHED-IMPOUNDMENT-001`

Acceptance checks:
- Topology namespace closure (`nchan`, structure/channel indexing).
- Datver and branch arity handling with typed error taxonomy.
- Watershed parser state can be emitted to observability subsystem surfaces.

## Wave 4 (Watershed Sidecar Extension)

Ordered sequence:
1. `SC-INFILE-CHANINP-001`
2. `SC-INFILE-TC-001`
3. `SC-INFILE-GWCOEFF-001`
4. `SC-INFILE-TCR-001`
5. `SC-INFILE-PHOSPHORUS-001`
6. `SC-INFILE-LCWB-001`

Acceptance checks:
- Channel sidecar parser surfaces align with channel topology and routing flags.
- Conflict and namespace guards (`gwcoeff` vs `chan.inp` semantics) are tested.
- Output/compatibility flags (`lcwb`, `tc`, `tcr`) have explicit strict/compat
  typed behavior and no hidden fallbacks.

## Cross-Wave Governance Actions

1. Track all contract `*-GAP-*` entries in implementation WPs as explicit risk
   checklist items.
2. For each wave, require a fixture-closure pass for highest-risk gap items
   before wave GO.
3. Require observability intent/trace coverage for parser guard failures to
   preserve kernel stimulation ergonomics.

## Minimal First Critical Path

`slp -> sol -> cli -> man` is the first critical path that unlocks
single-OFE/daily parser ingestion and aligns with Tier-A comparator confidence
policy.

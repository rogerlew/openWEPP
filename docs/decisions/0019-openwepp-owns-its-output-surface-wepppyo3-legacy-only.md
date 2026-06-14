# ADR-0019: openWEPP owns its output surface; wepppyo3 interchange stays wepp-legacy-only

**Status:** Accepted (supersedes ADR-0005)
**Date:** 2026-06-14
**Deciders:** Roger Lew, Claude Code

## Context

[ADR-0005](0005-parquet-via-wepppyo3-interchange.md) set the original posture:
openWEPP emits parquet using the **existing wepppy/wepppyo3 interchange
schemas**, schema versions are **co-managed**, openWEPP **does not define new
schemas**, and per-hillslope parquet is **concatenated into watershed-level
outputs by wepppy, not openWEPP**. That was the right low-cost choice at
bootstrap (consumers already existed; no schema authoring needed).

The MOFE01 → WSHED01 work surfaced its cost. MOFE01 made the hillslope WAT
genuinely **per-OFE**, and the WSHED01 watershed/totalwatsed3 work showed that
deriving openWEPP's aggregated outputs through the legacy interchange means
openWEPP inherits legacy interchange **semantics and constraints** it does not
otherwise need (e.g., the totalwatsed3 aggregation is hillslope-only and
unrelated to channel routing, yet was entangled with watershed-CLI legacy
concerns). The `wepppyo3 wepp_interchange` crate is fundamentally a **legacy
WEPP output → parquet converter**; making it also openWEPP's forward output
authority couples openWEPP's surface to a legacy-shaped contract and forces
co-management of two diverging concerns.

The decision point: should openWEPP keep inheriting/co-managing the wepppyo3
interchange surface, or own its own?

## Decision

**openWEPP owns its full output surface end-to-end.** openWEPP authors and
owns its own parquet schemas and produces its own aggregated/derived outputs
(including a native `openwepp-cli-totalwatsed3`), rather than inheriting
schemas from — or routing aggregation through — wepppy/wepppyo3.

**`wepppyo3 wepp_interchange` is frozen as wepp-legacy-only.** It remains the
converter for *legacy* WEPP ASCII output → parquet. It is **not** shared into,
depended on by, or co-managed with openWEPP. openWEPP does not link or reuse
that crate.

Interoperability is preserved by openWEPP matching the **closure semantics**
(the water-balance identities consumers rely on — e.g. the totalwatsed3
`P − (Runoff + Lateral + ET + Perc + Interception) − ΔStorage` identity) where
a consumer contract requires it, **not** by inheriting the legacy schema shape.
openWEPP's schemas are an openWEPP-controlled contract.

## Consequences

- openWEPP **defines and maintains its own output schemas** (reverses
  ADR-0005's "no new schema authoring" / "co-managed versions"). This is a
  deliberate trade: more schema ownership for full control and freedom from
  legacy interchange constraints.
- openWEPP **produces its own aggregated/watershed-level outputs** (reverses
  ADR-0005's "concatenated by wepppy, not openWEPP"). The native totalwatsed3
  CLI is the first instance.
- `wepppyo3 wepp_interchange` carries **no forward-openWEPP burden** — it is
  not extended to track openWEPP's per-OFE / native-schema evolution. Less
  long-term mess: openWEPP does not carry the legacy interchange converters.
- **Preserved from ADR-0005:** openWEPP remains parquet-native and does **not**
  emit legacy ASCII formats; legacy ASCII compatibility stays wepppy's concern
  via its existing parsers.
- Consumers (wepppy `query_engine` etc.) adapt to openWEPP-native schemas, or
  openWEPP publishes the consumer contract for them. Where a closure audit must
  reconcile (totalwatsed3), openWEPP matches the *identity/units semantics*,
  not the legacy column shape.
- Per-package authority: the WSHED01 package
  (`docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/`)
  is the work surface where this surfaced and where the native totalwatsed3
  CLI (T-arc) lands.

## Relationship to other ADRs

- **Supersedes** [ADR-0005](0005-parquet-via-wepppyo3-interchange.md).
- Consistent with [ADR-0011](0011-architecture-first-top-down-science-contracts.md)
  (architecture-first; openWEPP's contracts are authority) and
  [ADR-0007](0007-openwepp-runner-and-release-governance.md) (openWEPP owns its
  runner/release boundary — this extends that ownership to the output surface).

# EROD10 Wave Execution Plan

Status: `completed`
Evidence mode: `Static + Ran`

Static:
- Wave decomposition is derived from PL15 queue posture, WB16 completion, and
  current non-promotable contract gaps across erosion companion domains.

Ran:
- Package and contract artifacts were inspected in-repo to confirm dependency
  order and current completion states.

## Wave Summary

| wave | packages | purpose | state after EROD10 |
|---|---|---|---|
| Wave 0 | `EROD11`, `EROD12` | authority/ownership closure | planned (`HOLD` until executed) |
| Wave 1 | `EROD13` | core hillslope erosion kernel | planned |
| Wave 2 | `EROD14` | OFE routing + enrichment kernel | planned |
| Wave 3 | `EROD15` + `WS10` dependency | routing-boundary production coupling | planned |
| Wave 4 | `EROD16` | comparator/governance closeout | planned |

## Wave 0: Authority and Ownership Closure

Packages:
- `EROD11-alias-and-boundary-ownership-closure-001`
- `EROD12-cross-domain-contract-closure-001`

Entry criteria:
- EROD10 completed.
- WB16 completed (already satisfied).

Exit criteria:
- Non-promotable alias gaps resolved with explicit owner+symbol map.
- Companion cross-domain ownership gaps dispositioned with dual review and
  verification artifacts.
- Wave-0 gate report explicitly marks whether erosion code-authoring is
  released from `HOLD`.

## Wave 1: Core Hillslope Erosion Kernel

Package:
- `EROD13-hillslope-core-erosion-kernel-001`

Entry criteria:
- Wave 0 complete.
- Contract-first prerequisite artifacts committed.

Exit criteria:
- Runtime implementation covers `INV-SED-001..007` families.
- Typed guard surfaces mapped to explicit error families.
- Contract-derived tests and required repo gates pass.

## Wave 2: OFE Routing and Enrichment Kernel

Package:
- `EROD14-multiofe-and-enrichment-kernel-001`

Entry criteria:
- `EROD13` complete.

Exit criteria:
- Runtime implementation covers `INV-SED-008..009`.
- Conservation vectors for class-fraction and class-mass behavior pass.
- No silent fallback/default branches in OFE case transitions.

## Wave 3: Routing Boundary Coupling

Packages:
- `EROD15-routing-boundary-coupling-001`
- upstream dependency: `WS10-channel-impoundment-production-kernels-001`

Entry criteria:
- `EROD14` complete.
- `WS10` no longer `queued`/placeholder-only for production kernel path.

Exit criteria:
- `INV-SED-010` payload export is wired and validated.
- `SC-ROUTE-001` handoff completeness checks pass in integration fixtures.
- Cross-lane typed-seam non-regression evidence produced.

## Wave 4: Comparator and Governance Closeout

Package:
- `EROD16-sediment-closeout-and-comparator-001`

Entry criteria:
- Wave 3 complete.

Exit criteria:
- Tiered comparator evidence produced for erosion-lane claims.
- Remaining gaps explicitly dispositioned by promotability class.
- Closeout language preserves ADR-0011 confidence-tier posture and does not
  over-claim beyond evidence.

## Global Gate Policy

| gate_id | policy | applies_to |
|---|---|---|
| `EROD-GATE-001` | Unresolved high-severity authority/ownership ambiguity keeps package `HOLD`. | Waves 0-4 |
| `EROD-GATE-002` | Contract-first sequencing is mandatory before production code edits. | Waves 1-3 |
| `EROD-GATE-003` | No silent fallback/default behavior for invalid numerical/branch domains. | Waves 1-3 |
| `EROD-GATE-004` | Required repo gates (`fmt`, `clippy`, `test`, `deny`) are mandatory for packages touching production code. | Waves 1-4 |
| `EROD-GATE-005` | Comparator outputs are interpreted by confidence tier; Tier-B deltas trigger investigation, not automatic parity rejection. | Wave 4 |

This plan is the executable wave baseline produced by EROD10.

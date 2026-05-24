# PL09 PL08 Hold-Lift Decision Record

Status: `complete`
Evidence mode: `Static + Ran`
Decision: `RETAIN PL08 HOLD`
Superseded by: `docs/work-packages/20260523-pl15r-tier-a-delta-recloseout-and-hold-lift-rerun-001/artifacts/pl15r-pl08-hold-lift-decision-record.md` (refreshed PL15R decision, 2026-05-23)

Static:
- Decision authority follows ADR-0011 confidence-tier policy and PL08 package
  clear conditions.

Ran:
- Decision criteria were evaluated against PL08 comparator artifacts and PL09
  totality/decomposition discovery outputs.

## Decision Criteria Evaluation

| criterion | target | result |
|---|---|---|
| Tier-A strict comparator closure (`H5.wat.dat`) | no unresolved structure blockers | `fail` |
| Direct openWEPP-vs-legacy Tier-A evidence | available and reproducible | `fail` |
| Root-cause traceability for observed Tier-A deltas | mapped to implementable ownership gaps | `pass` |
| Representation/decomposition completeness for PL hold-lift planning | actionable gap register + queue | `pass` |
| Policy conformance (Tier-A unresolved == block) | enforced | `pass` |

## Rationale

1. The Tier-A blocker pair from PL08 remains unresolved:
   `H5.wat.dat` strict structural divergence and missing direct openWEPP
   candidate output evidence.
2. PL09 discovery confirms unresolved implementation surfaces that directly
   affect branch-faithful PL transition execution.
3. Hold-lift is therefore premature; execution should proceed through a
   dependency-ordered closure queue.

## Hold-Lift Preconditions

PL08 hold may be lifted only after all are satisfied:

1. Active slot/day branch authority replaces first-slot placeholder dispatch.
2. Event-level PL controls required by annual/perennial transitions are
   projected into runtime state surfaces.
3. Production growth/decomposition/residue transition execution exists and is
   validated against contracts/invariants.
4. Direct openWEPP-vs-legacy Tier-A output is emitted and strict comparator
   replay is executed with explicit delta disposition.
5. Canonical naming/alias continuity gaps (`PL09-GAP-007`) are either closed
   or dispositioned by formally approved scoped exception.
6. Any remaining Tier-A deltas are resolved or explicitly risk-accepted under
   policy with documented approval artifact reference.

## Downstream Queue Reference

- See
  `artifacts/pl08-hold-lift-work-package-queue.md` for dependency-ordered
  package proposals and acceptance evidence.

## Evidence Links

- `/home/workdir/openWEPP/docs/work-packages/20260520-arch01-subsystem-map-and-contract-spine/artifacts/comparator-confidence-tier-policy.md:48`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl08-comparator-confidence-tier-review-001/artifacts/pl08_disposition.md:27`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl08-comparator-confidence-tier-review-001/artifacts/comparator-confidence-tier-disposition.md:18`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl08-comparator-confidence-tier-review-001/artifacts/comparator-confidence-tier-disposition.md:20`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/openwepp-totality-implementation-inventory.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/openwepp-vs-baseline-pl-parity-gap-register.md`

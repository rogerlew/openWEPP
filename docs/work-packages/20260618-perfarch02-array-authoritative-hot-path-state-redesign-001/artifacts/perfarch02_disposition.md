# PERFARCH02 Disposition

Evidence class: Static + Ran. Prototype and timing were executed locally on
2026-06-18; no production migration was performed.

## Verdict

CONDITIONAL GO.

Proceed to an integrated WB11 array-authoritative pilot. Do not begin broad
production migration from this package alone.

## Why

The floor prototype answers the narrow gating question it could honestly
answer: an array-authoritative writeback/guard surface can preserve current
finite/domain semantics and exact exported maps while removing the success-path
string/map cost. The candidate path measured about 49.9x faster than the
current logical writeback/guard path for the prototyped surface.

That result rejects more read-mirror/id-table work as the next lever and
supports the array-authoritative architecture. It does not prove the full H2637
ratio because the prototype did not execute complete WB11 daily hydrology,
scheduler state movement, consumer-boundary validation, or publication.

## Target

The realistic next target is <=10x legacy no-UI. PERFARCH01's 89-90% removable
class would imply roughly 7.3-8.0x if the integrated migration realizes that
removal without exposing a new intrinsic floor.

The 5x target remains unproven. It requires about 93.16% total removal from the
PERFIDX06 endpoint, leaving only 45.6 seconds for all legitimate openWEPP work
on H2637. The floor prototype does not justify claiming that yet.

## Acceptance Status

- Floor prototype runs: met.
- Bit-identity on prototyped flow: met for exported state/flux maps.
- Failure-path semantics: met for rejection, message id class, and lazy failure
  subject resolution.
- Per-OFE-day cost and H2637 projection: met in
  `perfarch02-floor-measurement.md`.
- Three-problem design: met in `perfarch02-redesign-shape.md`.
- Contract blast radius: met in `perfarch02-contract-blast-radius.md`.
- Staged migration plan: met in `perfarch02-staged-migration-plan.md`.
- Proposed ADR: met in `perfarch02-proposed-adr.md`.

## Next Package

Proposed next package: `PERFARRAY01` - integrated WB11
array-authoritative pilot.

Minimum scope:

- add non-default array-authoritative contract shell;
- port one representative WB11 daily flow including real guards and scheduler
  apply semantics;
- assert bit-identical logical materialization against the current path;
- measure H2637 no-UI on the same host;
- decide whether the <=10x target remains credible before expanding families.

## Closure

PERFARCH02 closes as a successful feasibility package with a conditional go.
The production write set remains untouched except for the artifact-local
prototype package under this work package.

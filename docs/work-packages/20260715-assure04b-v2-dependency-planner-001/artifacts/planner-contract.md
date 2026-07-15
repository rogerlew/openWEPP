# ASSURE-04B Planner Contract

Status: frozen before implementation

Evidence class: Static

The planner reports build impact; it does not report scientific quality. Its
states are mechanically determined from content identities and graph reachability:

| State | Mechanical meaning | Consequence |
| --- | --- | --- |
| `current` | identity matches and all prerequisites are current | no rebuild selected |
| `stale` | observed bytes differ from declared SHA-256 | the source declaration must be reconciled |
| `blocked` | content is unavailable/unreadable, or a prerequisite is blocked | consumer cannot be safely rebuilt |
| `selected` | node is usable but a prerequisite is stale/selected | later assembly must revisit the consumer |

Cycles, missing logical destinations, and unreachable declarations are graph
contract errors. They fail rather than becoming status rows. A missing local
file is different: its identity and edge are declared, so the planner can
report the source and transitive consumers as blocked.

Every report plan begins at one report target. The manifest binds embedded
record identities; local manuscript, supplement, result, research-object, and
dependency files add their own observed/declared identities; external and
restricted dependencies add immutable identities. Schema and planner contract
identities are shared prerequisites, but whole-catalog bytes are not a report
dependency because one report's catalog hash update must not select unrelated
reports.

Ordering is dependency-first topological order with stable lexical IDs among
ready nodes. JSON and human output are projections of the same typed plan. The
plan reads only; no result here means a manuscript has been rendered, reviewed,
approved, or published.

# Line-Count Governance

Status: `EXECUTED` — one touched file over the 3000-line threshold,
dispositioned `follow-up` (pre-existing oversize; net +4 lines this package).

Evidence class: Ran (`wc -l` current vs `git show 6dbeda40:<path> | wc -l`).

| Touched `.rs` file | Before | After | Δ | Band |
|---|---:|---:|---:|---|
| runner `direct_publication/day_input_and_helpers/00_builders_and_authority.rs` | 4,137 | 4,141 | +4 | **3000+** |
| hydrology `coupling/frost_entry.rs` | 1,967 | 1,959 | −8 | ok |
| hydrology `coupling/frost.rs` | 1,800 | 1,803 | +3 | ok |
| hydrology `infiltration_reconciliation.rs` | 1,915 | 1,915 | 0 | ok |
| hydrology `runoff_reconciliation.rs` | 1,228 | 1,230 | +2 | ok |

(Other touched files — `typed_boundary.rs`, `coupling.rs`, `erosion.rs`,
`evapotranspiration.rs`, `subsurface.rs`, `lib.rs`, test file — are well
under the WARN band.)

Disposition for `00_builders_and_authority.rs` (3000+): the file entered the
package at 4,137 lines — the oversize is pre-existing, not created here. This
package's edit is the F5 authority-field change (+4 net lines) inside the
existing structure; splitting a 4,100-line `include!`-composed runner surface
is a mechanical-refactor package of its own
(`docs/standards/mechanical-refactor-authoring-guide.md`) and sits outside
this package's write-set and behavior-preserving perf scope. Disposition:
**follow-up** — queue the split to the mechanical-refactor backlog; this
package held the WSHED-W3 precedent's bar of not growing an over-band file
materially (+0.1%).

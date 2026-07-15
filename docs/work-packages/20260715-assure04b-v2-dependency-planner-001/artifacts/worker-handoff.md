# ASSURE-04B Worker Handoff

Status: ASSURE-04C is next eligible; not authorized by this handoff

ASSURE-04C should consume `V2Repository::{plan_report, plan_all}` as the typed
input to deterministic, staging-only manuscript and supplement assembly. It
owns value, table, figure, citation, and portable-link resolution plus
report-specific `build` and `check`. It must not add review locks, public
promotion, catalogs, snapshots, release transfer, or vendoring; those remain
ASSURE-04D or later.

Before adding assembly logic, assess a behavior-preserving split of the
2,064-line `v2.rs` source-admission module into schema/source types, structural
admission, and content readers. The WARN is not a 04B closure block, but 04C
must avoid growing the file toward 3,000 lines.

Preserve these inherited boundaries:

- one/all paths share the per-report planner and dependency-first stable order;
- build impact states are mechanical and never scientific grades;
- descriptor-relative no-follow reads remain the only local source path;
- ordinary work stays offline, deterministic, repository-relative, and free of
  agent/wall-clock/hostname dependencies;
- staging cannot write tracked `usersum`, exports, snapshots, or vendor trees;
  and
- the tracked public assurance catalog remains zero-report until genuine review
  and approval in a later package.

The next worker must scaffold a separately authorized ASSURE-04C package,
freeze the terminal 04B APIs and protected identities, and derive assembly
tests from the accepted manuscript/source-build contracts before production
edits.

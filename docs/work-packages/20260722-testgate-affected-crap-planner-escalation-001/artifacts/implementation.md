# Implementation

Static: scaffold commit `cb6eda5e` predates all planner edits.

Static: `CargoGraph` now classifies direct `crates/<member>` production owners.
The planner retains the complete affected/reverse-dependent measurement closure
for ordinary package gates, but passes only production-owning packages to
affected CRAP and its quality-scope claim. A directly changed measurement-only
package raises critical risk with
`MEASUREMENT_ONLY_PACKAGE_REQUIRES_GLOBAL_QUALITY`, selecting global CRAP before
node construction.

Static: no science, subprocess, receipt, retry, or comparator behavior changed.

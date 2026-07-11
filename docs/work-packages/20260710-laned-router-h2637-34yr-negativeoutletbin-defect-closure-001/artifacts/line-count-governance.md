# Line-Count Governance

Status: `EXECUTED-WARN-DISPOSITIONED`

Evidence mode: `Static`

`crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`
contains `2570` lines after review disposition. This is a warning under the
repository's `>= 2000`-line policy and remains below the `>= 3000`-line
execution blocker.

The file currently owns the kinematic-wave data model, solver stepping,
conservative outlet recorder, typed errors, and their unit/contract tests.
LANED-NOB-001 changes only the downstream predictor-face construction and
adds two tightly coupled regressions. Splitting the module inside this defect
closure would expand the semantic diff, complicate pre/post comparator
attribution, and provide no safety benefit to the bounded correction.

Follow-on split intent: openWEPP maintainers should move the large in-module
test body and recorder-specific tests into dedicated child modules, then
consider separating solver state/types from stepping after this package is
closed. That mechanical refactor should have its own work package, preserve
the public module surface and test names, and prove byte/behavior identity.
No decomposition waiver is claimed for future feature work.

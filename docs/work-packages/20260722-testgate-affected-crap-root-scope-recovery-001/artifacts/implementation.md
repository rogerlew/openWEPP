# Implementation

Static: scaffold commit `ddb00d41` predates all tooling/test edits.

Static: the checker now resolves exact workspace-member identities from locked,
offline Cargo metadata. Affected measurement admits only packages whose manifest
is under `crates/`; unknown packages and measurement-only packages such as root
`openwepp` fail closed with an explicit global-quality requirement.

Static: the shell driver invokes this check before tool-version checks,
compilation, coverage cleaning, or instrumented Nextest. Valid scope JSON is
retained as `affected-package-scope.json`. Fresh reports bind both measurement
and resolved production identities in acquisition provenance. All three driver
adapters bind SHA-256
`e801924c5cd04178b6773e09bc57274e61d5606b22e6b0a789d7d499eaf257be`.

Static: planner escalation is outside the immutable write set. RTR-035 remains
open for a fresh prerequisite package that selects global rather than affected
quality for measurement-only packages.

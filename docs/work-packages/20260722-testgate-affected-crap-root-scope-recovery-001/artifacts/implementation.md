# Implementation

Static: scaffold commit `ddb00d41` predates all tooling/test edits.

Static: the checker now resolves exact workspace-member identities from locked,
offline Cargo metadata. Affected measurement admits only direct `crates/*`
members with a real library, binary, or proc-macro target and a non-symlink
source under their own `src/` root.
Unknown, nested, test-only, out-of-tree-target, and measurement-only packages
such as root `openwepp` fail closed with an explicit global-quality requirement.

Static: the shell driver invokes this check before tool-version checks,
compilation, coverage cleaning, or instrumented Nextest. Valid scope JSON is
retained as `affected-package-scope.json`. Final adjudication revalidates those
exact bytes against current locked/offline metadata and binds their SHA-256 in
acquisition provenance. All three driver adapters bind SHA-256
`4d475967b2b3fd35b7ea167bf8c9b60e0160e3329b1467243f1e31ebad557e2b`.

Static: final validation parses and hashes one in-memory byte buffer. The driver
also compares that preflight digest again immediately before PASS publication,
so an acquisition-time rewrite fails closed.

Static: planner escalation is outside the immutable write set. RTR-035 remains
open for a fresh prerequisite package that selects global rather than affected
quality for measurement-only packages.

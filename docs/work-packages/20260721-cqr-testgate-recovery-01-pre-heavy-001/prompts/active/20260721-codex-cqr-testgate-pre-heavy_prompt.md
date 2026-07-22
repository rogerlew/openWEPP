# Active Prompt: CQR TESTGATE Pre-Heavy Admission Complexity

Execution mode: package-end-to-end.

Execute `20260721-cqr-testgate-recovery-01-pre-heavy-001` without changing its
single production-module boundary. Preserve exact audit construction,
validation, canonical JSON, error codes, and fail-closed ordering while reducing
each eligible CRAP row in `crates/openwepp-gate-planner/src/pre_heavy.rs` to
`<= 30`.

Read the package's required-reading list and record it in
`artifacts/required-reading-map.md` before source edits. Characterize behavior
before extracting helpers. Do not change policy, schema, public behavior,
numeric/expression ordering, or error precedence.

Subagent authorization: this prompt explicitly authorizes spawning/delegating
to read-only comparator/closure-runner, review, and verification subagents for
metric checks, selected heavy gates, source/diff review, and behavioral
verification. Outputs are the named package artifacts. Write access is read-only
unless explicitly bounded to this target module or package artifacts.

REQUIRED: delegate every terminal-plan heavy gate to
`comparator_suite_runner` when available. Do not run those heavy gates locally
unless unavailability is recorded with command-level evidence.

Autonomy: complete or truthfully hold this package, commit its disposition, and
then continue only as the CQR ExecPlan permits.

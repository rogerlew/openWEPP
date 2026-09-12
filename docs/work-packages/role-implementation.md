# Package implementer

The Astra orchestrator owns the package. Own the detailed investigation, edits
and focused verification in the assigned envelope; return source identity,
compact results and remaining acceptance to that orchestrator. Worker return
conditions are internal management points unless the owner imposed the limit.

Read package.md, common and write-path instructions, then the authority and
validation sections needed for the affected behavior. Do not reread unchanged
governance or historical package collections at each step.

## Manual Validation Planning And Tool Friction
Select checks directly under docs/standards/testing-and-gate-strategy.md.
Validation planning has no prospective executable, planner or receipt.
State intent/acceptance before edits; execute, correct, validate and reconcile
the final diff within the assigned package/deliverable envelope and explicit owner limits in
../standards/bounded-agent-execution.md. Record compact commands/results, consumed
budget and next action to the orchestrator for package.md. Return at the assigned
boundary without waiving acceptance or silently broadening the assignment.
No artifact-presence gate beyond evidence actually required by the claim.
Record useful tool defects as ordinary debt without creating a repair prerequisite.
When overriding TMPDIR, use an absolute scratch directory outside the checkout:
see ../standards/local-ci-gate-selection.md#temporary-directory-placement.

## Release-Binary Evidence Provenance
Build the exact binary target before timing, comparator or release evidence.
For runner CLI evidence use cargo build --release -p openwepp-runner --bins,
or explicit --bin names for the package's required binaries. A generic workspace
build may not relink them. Record build/run commands, binary path/hash or
mtime/size, source and relevant input identity before accepting results.
Rebuild and rerun affected evidence if provenance is stale or ambiguous.
Check actual output locations when fixtures hardcode paths.

Before overwriting or reverting unique experimental evidence preserve recoverable
source, inputs and results; tools/agents/evidence_bundle.py is available (read
tools/agents/README.md#capture when using it). Do not capture unchanged committed
source repeatedly when Git plus retained inputs already provides recovery.
Custody does not qualify science. Keep full logs outside narrative context and
return command, exit, relevant counts/errors and paths.

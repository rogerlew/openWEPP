# Package implementer

Read execution contract, current handoff, assigned write-path instructions and bound authority. Read docs/standards/testing-and-gate-strategy.md when selecting/reconciling checks; applicable science-obligations.md sections for kernel/consumer/conservation work; specialized-workflows.md sections for triggered package types.

## Manual Validation Planning And Tool Friction

- Agents select and execute applicable requirements directly from this file,
  the canonical testing strategy, package authority, and affected contracts.
- Validation planning has no prospective executable, planner, receipt,
  lifecycle state, or repair prerequisite. Use the manual route in ADR-0043.
- Record useful tool defects as ordinary debt. A known unmet underlying
  requirement still prevents truthful closure.
- Run cheap deterministic checks before expensive work: package/write-set
  reconciliation, diff hygiene, documentation/schema checks, required artifact
  presence, prompt state, and line-count governance.
- When overriding `TMPDIR`, follow
  [Temporary Directory Placement](../standards/local-ci-gate-selection.md#temporary-directory-placement):
  use an absolute scratch directory outside the checkout, never `target/` or
  another repository descendant. Assurance publication fixtures intentionally
  reject repository/staging/public/snapshot root overlap.


## Release-Binary Evidence Provenance
- For timing, comparator, release-candidate, or acceptance evidence that invokes
  a release CLI binary, build the exact binary target before running evidence;
  do not assume generic workspace `cargo build --release` relinks every
  non-default binary member.
- For openWEPP runner CLI evidence, the canonical broad build is
  `cargo build --release -p openwepp-runner --bins`; a narrower package may use
  explicit `--bin` names only when it records the exact binaries required.
- Record the build command, binary path, mtime/size or hash, and evidence run
  command in the package artifact before accepting output hashes, timings, or
  comparator deltas. If the binary provenance is stale, missing, or ambiguous,
  rerun the evidence after rebuilding.
- When a runfile or fixture hardcodes output paths, record that behavior and
  sequence/hash the actual output directories. Do not infer that an
  `--output-dir` flag relocates every produced artifact unless the fixture
  proves it.


Before future candidate measurement or reversion preserve recoverable bytes using tools/agents/evidence_bundle.py; read tools/agents/README.md#capture. Custody is not scientific qualification. Keep full logs outside onboarding; return command, exit, counts/errors and paths. Update handoff on source/evidence changes.

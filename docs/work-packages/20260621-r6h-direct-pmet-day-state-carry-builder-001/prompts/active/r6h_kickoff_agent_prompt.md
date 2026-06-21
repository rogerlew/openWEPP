# R6H Kickoff Agent Prompt

Status: queued.

Execute
`docs/work-packages/20260621-r6h-direct-pmet-day-state-carry-builder-001/package.md`.

First action: close defect
`R6H-DIRECT-PMET-DAY-STATE-CARRY-BUILDER`.

Required posture:

- start from `HOLD-R6G-WAT-PMET-DAY-STATE-CARRY-BUILDER-ABSENT`;
- replace the precomputed multi-day `DirectPublicationDayInput` vector with an
  interleaved builder that executes day `n`, commits direct state, and builds
  day `n+1` PMET operands from direct-carried layer/state;
- keep current-fixture HBP identity green;
- prove WAT row/schema/metadata parity from typed direct projection only, or
  produce a new exact `HOLD-R6H-*` marker after in-envelope corrections are
  attempted;
- handle lane-dimensional direct day inputs, WAT id authority, and allowlisted
  symbol lineage in the current package scope;
- do not read compatibility WB13 rows, compatibility runtime surfaces,
  writeback payloads, writer rows, or output rows as direct authority;
- continue iterating until the R6G blocker is actually cleared or a new
  reviewed out-of-envelope boundary is proven.

Subagent authorization: this package explicitly authorizes
spawning/delegating to rust code-review and QA-verification subagents for R6H
implementation review and verification; expected outputs are
`artifacts/review_agent_a.md`, `artifacts/review_agent_b.md`,
`artifacts/verification_agent_a.md`, and `artifacts/verification_agent_b.md`;
write access is read-only for review and verification agents.

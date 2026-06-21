# R6G Kickoff Agent Prompt

Status: queued.

Execute `docs/work-packages/20260621-r6g-direct-wat-producer-authority-001/package.md`.

First action: close defect `R6G-DIRECT-WAT-PRODUCER-AUTHORITY`.

Required posture:

- start from `HOLD-R6F-WAT-DIRECT-PROCESS-PRODUCER-AUTHORITY-GAP`;
- keep current-fixture HBP identity green;
- bind WAT identity, ET, storage, and profile operands from parsed typed inputs
  and direct runtime state only;
- do not read compatibility WB13 rows, compatibility runtime surfaces,
  writeback payloads, or output rows as direct authority;
- continue iterating until WAT parity passes or a new exact, reviewed `HOLD`
  boundary is proven.

Subagent authorization: this package explicitly authorizes
spawning/delegating to rust code-review and QA-review subagents for R6G
implementation review and verification; expected outputs are
`artifacts/review_agent_a.md`, `artifacts/review_agent_b.md`,
`artifacts/verification_agent_a.md`, and `artifacts/verification_agent_b.md`;
write access is read-only for review agents.

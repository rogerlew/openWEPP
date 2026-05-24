# WS11 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file reads/
edits only; no external connectivity.
Phase: A only.
Files:
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `docs/work-packages/20260524-ws11-channel-routing-physics-equivalence-port-001/artifacts/ws11-contract-implementation-evidence.md`
- `docs/work-packages/20260524-ws11-channel-routing-physics-equivalence-port-001/artifacts/ws11-channel-routing-physics-authority-and-guard-map.md`
Task: implement canonical WS11 channel-routing physics authority amendments and
record contract-implementation/authority-map evidence for Phase A only.
Constraints: contract-first sequencing; canonical SC authority; baseline
provenance (`/workdir/wepp-forest_260430_baseline` at
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`); typed guards; no silent defaults.
Outputs: update listed WS11 artifacts for this phase only.

Execution topology:
- Run WS11 in dedicated worktree branch
  `ws11-channel-routing-physics-equivalence-port-001`.
- WS12 may run concurrently in its own worktree branch.
- WS11 merges to `main` before WS12 merge-back.

Mandatory sequencing constraints:
- Do not modify production kernel code until:
  1. canonical contract amendments are implemented,
  2. contract-derived tests are implemented, and
  3. pre-implementation contract-gate evidence is recorded.
- WS11 migration physics authority must be canonical `SC-*` text, not
  package-local notes.
- Do not introduce silent defaults/clamping for domain violations; use typed
  errors/guards.

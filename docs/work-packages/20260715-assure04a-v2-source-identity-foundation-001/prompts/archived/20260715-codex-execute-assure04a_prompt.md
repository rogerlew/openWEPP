# Execute ASSURE-04A

Scope: local openWEPP repository assurance engineering; flat-file reads, edits,
builds, and tests only; no external connectivity or external-system writes.

Execution mode: package-end-to-end.

Phase plan: execute every phase in `package.md` sequentially through final
disposition unless a proven hard blocker prevents a current-scope gate.

Required reading:

- Core: root `AGENTS.md`, `docs/codex_exec_plans.md`,
  `docs/work-packages/AGENTS.md`, this package, `docs/ROADMAP.md`, the v2
  implementation roadmap, ASSURE-03 handoff, v2 architecture, source/build and
  lifecycle contracts, report standard, and groundwater prototype.
- Conditional: `crates/AGENTS.md`, `tests/AGENTS.md`,
  `docs/standards/AGENTS.md`, prompt wording, and local CI selection for their
  triggered edits/runs.
- On-demand: the fixture claim matrix/current-tree record/pilot decision,
  current assurance code/tests/public guards, ADR-0038, and
  `SC-GWBASEFLOW-001` only for referenced authority identity.

Required-reading budget: 152,057 bytes, `OK` against the 400,000-byte
threshold; map: `artifacts/required-reading-map.md`.

Files: only the declared write set in `package.md`; protected public assurance
and `usersum` files are read-only byte-identity surfaces.

Task: implement the v2 source and identity foundation demonstrated by the
groundwater prototype, prove real one/all CLI validation, and close every
ASSURE-04A gate without absorbing ASSURE-04B through D.

Constraints: typed fail-closed errors; no silent defaults; confined regular
files; content identities; no network/shell/agent/clock dependency during
ordinary validation; no generated interpretation; no public draft route;
preserve exact zero-report outputs. Do not implement dependency planning,
rendering, publication, review approval, release transfer, or vendoring.

Subagent requirement: REQUIRED. Spawn the heavy-gate runner for full workspace,
deny, and fresh CRAP closure. This prompt explicitly authorizes subagent
spawning/delegation to one heavy-gate runner plus two independent read-only
reviewer/verifier agents for package Phases 4 and 5. Outputs are compact metrics
and package artifacts; heavy-runner write access is package-artifact-only and
reviewer/verifier access is read-only. Do not represent coding-agent review as
external scientific peer review.

Autonomy: execute end-to-end and update all evidence without requesting next
steps unless hard-blocked. A phase is complete only with direct current
evidence; do not defer a required A gate to B through D and call A complete.

Outputs: v2 source fixture and executable admission, tests, protected-surface
proof, gate/CRAP records, reviews, disposition, verifications, and handoff.

# SNOW-SURFACE-EB-03B Kickoff

Status: `archived / executed`

Scope: local repository validation-infrastructure defect closure; flat-file
reads/edits and local tests only; no external connectivity.

Execution mode: package-end-to-end.

Phase plan: execute every phase in `package.md` sequentially through
disposition.

Required reading:

- Core: `/home/workdir/openWEPP/AGENTS.md`,
  `docs/codex_exec_plans.md`, `docs/defect_closure_execplans.md`,
  `docs/work-packages/AGENTS.md`, `docs/work-packages/README.md`, and this
  package's `package.md`.
- Conditional: `docs/standards/testing-and-gate-strategy.md`,
  `tests/AGENTS.md`, and `crates/AGENTS.md`.
- On-demand: EB-03A terminal artifacts, quality-observatory tooling, assurance
  publication test/helper sources, and the snow campaign roadmap.

Required-reading budget: 477141 local bytes, `WARN`; the package catalog
accounts for 351813 bytes and remains a governance-required Core read; map:
`artifacts/required-reading-map.md`.

Task: close `EB03B-CQR-001` and `EB03B-ASSURE-001` end-to-end, preserve exact
identity and assurance fail-closed semantics, run the complete required
profiles, and admit EB-04 only on full direct PASS evidence.

Constraints: no snow physics, coefficient, selector, trust-root, approval,
release, publication-authority, timeout-limit, metric-threshold, production
filter, or CQR exception change. Do not delete, ignore, weaken, or silently
reclassify a test or negative case.

HOLD legitimacy audit: do not hold while source reading, implementation,
focused regression, or required validation remains possible inside the
Correction Authority Envelope. A hold must name the out-of-envelope boundary,
direct evidence, considered correction, and why it cannot close now.

Subagent requirement: REQUIRED. Spawn/delegate `comparator_suite_runner` for
complete quick, frost, and full batch runs. This prompt explicitly authorizes
subagent spawning/delegation to two read-only reviewers, two read-only
verifiers, and the heavy-suite runner; expected outputs are compact findings,
metrics, and log paths; write access is read-only except for test temporaries.

Autonomy: execute through final disposition without requesting user direction
unless a proven hard blocker requires new authority.

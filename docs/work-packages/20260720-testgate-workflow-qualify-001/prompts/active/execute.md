# Execute TESTGATE Black-Box Workflow Qualification

Scope: local openWEPP black-box workflow qualification plus at most one ordinary
TESTGATE provider execution. Repository edits are limited to the declared
documentation/evidence write set. Do not edit the frozen subject, create a
branch, or mutate any other external system.

Execution mode: package-end-to-end. Execute every phase in `package.md`
sequentially through truthful disposition unless a declared dependency or
provider boundary blocks progress.

Required reading:

- Core: `AGENTS.md`, `docs/work-packages/AGENTS.md`,
  `docs/standards/testing-and-gate-strategy.md`,
  `docs/standards/prompt-wording-guidance.md`, this package, this prompt,
  `artifacts/qualification-matrix.md`, `artifacts/controller-contract.md`, and
  `artifacts/evidence-contract.md`.
- Conditional: the completed implementation disposition, exact terminal plans,
  pre-heavy audit, receipts, ledger, reviews, and
  `.github/workflows/testgate-shadow.yml` before subject/provider admission.
- On-demand: frozen helper/planner/policy source and prior adversarial evidence
  only when validating anti-fabrication or interpreting failure.

Required-reading budget: 158,337 local bytes, `OK` (`<=400000`); recalculate
after scaffold review and immediately before execution. Map:
`artifacts/required-reading-map.md`.

Files: only `docs/ROADMAP.md`, `docs/work-packages/README.md`, and this package
subtree. Execution output stays in ignored/external fresh roots.

Task: freeze the completed `TESTGATE-CLOSURE-AUDIT-01` implementation; validate
the qualification interface; run `Q01`–`Q15` once through the real TESTGATE
entry point with bounded probes; prove crash/restart, durable re-ingestion,
target-context receipt reuse, and exact process counts; reuse the current real
combined-run receipt or dispatch exactly one ordinary TESTGATE run after proving
the queue idle; complete independent review, verification, and disposition.

Constraints: no subject repair or policy change; no fabricated receipt; no
producer-only closure; no unexpected-result retry; no repeated real heavy run;
no manual dispatch while TESTGATE is queued or active; no provider claim from a
local harness. A failed case opens a correction package and blocks qualification.

Subagent requirement: REQUIRED. This prompt explicitly authorizes subagent
spawning/delegation to one black-box executor with writes limited to ignored or
external fresh roots, two independent read-only reviewers, two independent
read-only terminal verifiers, and `comparator_suite_runner` for the one selected
real heavy/provider run. Outputs: compact case results, commands, process
counts, timings, artifact paths/hashes, findings, and `PASS`/`HOLD`/`FAIL`
verdicts. Only the parent may authorize the exact provider dispatch after the
queue-idle preflight.

Autonomy: execute through final disposition without requesting additional
direction unless dependency completion, credentials, provider availability, or
an unexpected case result creates a declared hard boundary.

Outputs: subject freeze, controller input, per-case reports, process traces,
ledger/recovery reconstruction, provider evidence, dual reviews, finding
disposition, dual verification, worker handoff, archived prompt, and final
disposition.

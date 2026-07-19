# Execute TESTGATE-CUTOVER-01

Scope: local openWEPP repository plus the explicitly named `omarchy` host and
the `rogerlew/openWEPP` GitHub Actions/provider surfaces required by the package.

Execution mode: package-end-to-end (default).

Task: execute
`docs/work-packages/20260718-testgate-accelerated-cutover-001/package.md`
through immediate cutover. Provision the isolated trusted self-hosted runner,
route and bootstrap TESTGATE, close post-change actionable CRAP, conduct the
adversarial patch review, pass the event-driven acceptance matrix, and promote
the accepted aggregate without starting a timed observation period.

Required reading: root `AGENTS.md`, `docs/work-packages/AGENTS.md`, the package
required-reading map, ADR-0039, ADR-0040, the canonical testing/gate strategy,
the local CI standard, current workflows, and nearest instructions for every
write path.

Constraints: never route untrusted public pull-request code to `omarchy`; use a
supported isolated Linux guest and unprivileged repository-scoped runner;
never retain registration credentials; preserve fail-closed selection,
receipts, confinement, coverage/CRAP thresholds, and critical/campaign/release
qualification; run successful heavy commands once; do not create a 14-day,
20-increment, 50%, two-environment, or dual-required cutover gate.

Subagent requirement: REQUIRED for independent review and terminal
verification. This prompt explicitly authorizes subagent spawning/delegation to
one adversarial reviewer, one independent implementation reviewer, two terminal
verifiers, and one heavy closure runner for the exact scopes and write limits in
`package.md`; outputs are compact findings, verdicts, exact commands, timings,
and artifact paths.

Autonomy: execute every phase without asking the user to remember, monitor, or
advance a timer. Ask only when host access, one-time runner registration
authority, or another external credential boundary is actually unavailable.

Outputs: keep package progress and artifacts current, patch every accepted
finding, cut over immediately when event gates pass, archive this prompt at
closure, remove the roadmap item, and record the completed outcome in the
work-package catalog.

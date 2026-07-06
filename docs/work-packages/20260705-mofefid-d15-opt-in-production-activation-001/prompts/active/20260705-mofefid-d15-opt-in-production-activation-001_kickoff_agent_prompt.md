# D15 Kickoff Prompt

Scope: local repository science-contract/kernel activation task; flat-file
reads/edits only; no external connectivity required.

Execution mode: package-end-to-end (default).

Required reading:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/work-packages/20260705-mofefid-d15-opt-in-production-activation-001/package.md`
- `docs/work-packages/20260705-mofefid-d15-opt-in-production-activation-001/artifacts/required-reading-map.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/planning/mofe-fidelity-campaign-strategy.md` §6.1
- D10-D14 disposition and handoff files listed in the required-reading map.

Task: execute D15. Start with the pre-implementation authority gate. If
`SC-OFEROUTE-001` still blocks active routed-water publication via
`GAP-OFEROUTE-005` / `INV-OFEROUTE-011`, close D15 in
`EXECUTED-HOLD-SOURCE-AUTHORITY` and do not edit runtime code. If the hold has
been lifted by a prior commit, implement the opt-in activation package exactly
inside `package.md`.

Constraints: contract-first sequencing; no production/default activation
without source authority; no D10 shock-numerics workaround inside D15; no
surrogate/provisional/proxy/heuristic physics; no output-affecting runtime
change without the full conservation/output acceptance gate.

Subagent authorization: this package explicitly authorizes
spawning/delegating to `rust_code_reviewer`, `rust_qa_reviewer`, `explorer`,
and `comparator_suite_runner` subagents for read-only source/authority audit,
review, verification, fixture inspection, H2637/Lane D evidence, and heavy
gate execution. Outputs: compact findings, metrics, log paths, and
package-local artifact text. Write access: read-only unless the operator
assigns a bounded write set.

Autonomy: execute package phases end to end and update required artifacts
without requesting additional user direction unless hard-blocked.

# Codex Execution Prompt

SUPERSEDED - DO NOT EXECUTE AS WRITTEN. Use
`docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/prompts/active/20260708-codex-execute-tier2-dx-target-mesh-policy-rescope_prompt.md`
instead. ADR-0037 abandoned hybrid work and requires the Tier-2 question to be
re-scoped around plain active target-`dx` mesh policy.

Scope: local repository science-contract/kernel adjudication task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Task: execute `docs/work-packages/20260707-laned-router-tier2-mesh-resolution-adjudication-001/package.md`
through final disposition.

Constraints: contract-first sequencing; canonical `SC-OFEROUTE-001`
authority; no surrogate/provisional/proxy/heuristic physics; no production
mesh-policy change unless named fidelity tolerances pass.

Subagent requirement: REQUIRED for heavy comparator/timing/full closure gates.
This prompt explicitly authorizes subagent spawning/delegation to
`comparator_suite_runner`, `rust_code_reviewer`, `rust_qa_reviewer`, and
`explorer` for the scopes named in `package.md`; write access is read-only
unless separately bounded.

Autonomy: execute phases end-to-end; hold only with a package-local hold
legitimacy audit naming the exact blocker and first follow-on.

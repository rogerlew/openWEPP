# Kernel Work-Package Preparation Procedure

Status: Active

Relocated from `AGENTS.md` by DOCOPT01. This document remains a required, normative root-level procedure before preparing any kernel-affecting work package.

## Procedure
Use this procedure when preparing any kernel-affecting work package (including
runtime projection that controls kernel branches). This is a root-level
requirement, not optional package style guidance.

1. Authorize and name the package
- Confirm the package is authorized by the active queue/decision artifacts.
- Before scaffolding, check that the package is neither too small nor too broad:
  it should contain a coherent closure slice large enough to justify the
  mandatory administrative artifacts and small enough for one agent to execute
  end-to-end without intervention.
- If the proposed package would only produce another narrow observe ledger,
  route label, or single-surface diagnostic, widen it to include the adjacent
  source-line classification, contract/test update, or implementation/evidence
  step when those share the same authority domain and validation surface.
- Split into a smaller package only when adjacent work crosses a distinct
  canonical contract authority, touches unrelated write sets, requires
  materially different validation, or is blocked by evidence that cannot be
  produced in the same autonomous run.
- Use directory format `YYYYMMDD-<slug>-001` under `docs/work-packages/`.
- Add/update the entry in `docs/work-packages/README.md` so intent is
  discoverable.

2. Scaffold required structure
- Create `package.md`, `prompts/active/<id>_kickoff_agent_prompt.md`,
  `prompts/README.md`, `prompts/active/README.md`,
  `prompts/archived/README.md`, and `artifacts/README.md`.
- Pre-create required artifact files as queued placeholders (`Status: queued`,
  `Evidence mode: not-run`) including:
  - contract implementation evidence,
  - contract-test implementation evidence,
  - pre-implementation contract gate,
  - implementation/test evidence,
  - kernel-profile compliance checklist,
  - owned-file manifest,
  - gate results,
  - line-count governance checklist,
  - disposition,
  - worker handoff,
  - dual review artifacts with mandatory finding disposition templates
    (`review_agent_a.md`, `review_agent_b.md`),
  - dual verification artifacts
    (`verification_agent_a.md`, `verification_agent_b.md`).

3. Encode mandatory `package.md` content
- Status (`queued`), objective, rationale, included/excluded scope, explicit
  deliverables, dependencies, intended write set, phase plan, exit criteria,
  and security-impact gate.
- For DC-ExecPlans, encode the Correction Authority Envelope, conversion rule,
  seven-gate bar, `HOLD`-legitimacy boundaries, and defect-shaped handoff.
- Explicitly encode autonomous execution intent: the package must contain enough
  context, sequencing, and validation detail for no-intervention execution.
- Explicitly state contract-first sequence:
  1. contracts,
  2. contract-derived tests,
  3. pre-implementation contract gate,
  4. production code edits.
- Explicitly require dual reviews, finding disposition, and verification that no
  review findings remain undispositioned before final package disposition.
- Explicitly require line-count-governance disposition in review and checklist
  artifacts, including owner/sunset metadata for any approved `3000+`
  generated/fixture exception.
- Require truthfulness labeling in evidence artifacts (`Static:` vs `Ran:`).

4. Make physics authority explicit in canonical contracts
- For legacy migration packages, physics must be implemented in canonical
  `SC-*` science contracts first; package-local notes are not authority.
- Physics/equation authority defaults to
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` unless explicitly justified.
- Do not invent physics: every equation, constant, guard, and invariant must
  trace to canonical contract text plus provenance citations.
- Migration completeness is required for touched process families (for example
  ET, infiltration, snow/frost, runoff, routing): do not merge or disposition
  as complete with placeholder/proxy equations when baseline-authoritative
  routines are known and in scope for migration.
- Preserve variable naming continuity with legacy WEPP symbols; when runtime
  names differ, record explicit alias mappings in the relevant `SC-*` files.

5. Require baseline reference set in Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `docs/work-packages/README.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- Kernel-relevant canonical contracts in
  `docs/specifications/science-contracts/contracts/SC-*.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md` for legacy
  migration/parity packages
- Upstream queue/hold-lift/disposition artifacts that authorize the package

6. Enforce completion gates in the prepared prompt
- Kickoff prompt must prohibit kernel code edits before contract + test + gate
  completion.
- Kickoff prompt must explicitly require canonical `SC-*` physics authority
  updates for migration packages.
- Kickoff prompt must prohibit silent defaults, unbounded clamping, and
  canonicalize-and-proceed behavior for domain violations; require typed
  fail-closed errors/guards unless bounded normalization is explicitly
  contract-authorized.
- Kickoff prompt must instruct autonomous progression through the package phase
  plan and artifact updates through disposition without asking the user for
  "next steps" unless blocked.
- Kickoff prompt must include an explicit end-to-end execution statement (use
  `Autonomy:` line) for the declared scope.
- DC-ExecPlan kickoff prompts must say `Close defect <id> end-to-end`, include
  the Correction Authority Envelope, require conversion to a contract-first fix
  when the seven-gate bar is met, and prohibit relaying intermediate diagnostic
  steps into a new package.
- Kickoff prompt must include a `Required reading` section with explicit path
  references so onboarding/orientation does not require independent discovery.
  At minimum include:
  - `/workdir/openWEPP/AGENTS.md`
  - `/workdir/openWEPP/docs/codex_exec_plans.md`
  - `/workdir/openWEPP/docs/defect_closure_execplans.md` for defect-closure packages
  - `/workdir/openWEPP/docs/work-packages/README.md`
  - the package-local `package.md`
  - `docs/specifications/science-contract-authoring-procedure.md`
  - `docs/specifications/science-contracts/kernel-process-contract-profile.md`
  - `docs/specifications/science-contracts/index.md`
  - all phase-relevant canonical `SC-*` contracts and decision/queue artifacts.

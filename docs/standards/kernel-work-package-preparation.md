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
- For DC-ExecPlans, bias toward the smallest envelope that can actually close
  the defect, not the smallest envelope that can justify a hold. If the likely
  correction surface is adjacent and shares the same authority domain and
  validation surface, include it up front.
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
- Exit criteria must obey the Gate Evidence Non-Deferral Rule (canonical
  statement: `docs/work-packages/AGENTS.md`) and the canonical lifecycle in
  `docs/standards/testing-and-gate-strategy.md`: every increment-scope gate must
  be measurable and evidenced in the current increment. A campaign obligation
  may remain visibly deferred only when the accepted pre-implementation intent
  plan assigned its owner, trigger, rationale, and later boundary in the
  campaign ledger. A terminal discovery cannot be retroactively deferred.
- Require pre-implementation intent planning and exact-diff terminal
  reconciliation. Declare the conservative full path only for critical,
  campaign, release, or explicit rollback boundaries.
- For DC-ExecPlans, encode the Correction Authority Envelope, conversion rule,
  seven-gate bar, `HOLD`-legitimacy boundaries, required hold legitimacy audit,
  and defect-shaped handoff.
- For DC-ExecPlans, explicitly state that `HOLD` is exceptional. Diagnostic
  uncertainty, implementation effort, large edit size, or a partial compatibility
  route do not justify stopping while contract amendment, tests, production
  edits, or validation remain possible inside the envelope.
- Explicitly encode autonomous execution intent: the package must contain enough
  context, sequencing, and validation detail for no-intervention execution.
- Explicitly state contract-first sequence:
  1. contracts,
  2. contract-derived tests,
  3. pre-implementation contract gate,
  4. production code edits.
- Explicitly require dual reviews, finding disposition, and verification that no
  review findings remain undispositioned before final package disposition.
- Explicitly require reviews and verifications to check gate legitimacy, not
  just artifact presence: an unmet required gate cannot be deferred into later
  work while the current package/phase/increment is marked complete.
- For conservation-sensitive output/publication work, explicitly require:
  - an operand-lineage table before production edits, covering units,
    normalization basis, area/volume basis, source authority, and
    authoritative-vs-diagnostic status;
  - anti-tautology fixtures where each plausible wrong operand pairing yields a
    different numeric result from the expected output;
  - explicit rejection of known wrong formulas and adjacent aliases;
  - independent reconstruction from produced outputs plus a real closure or
    magnitude audit on the target fixture/cohort;
  - metadata/schema alignment with the accepted operand lineage.
- If dual review/verification, comparator execution, or other package-required
  work depends on delegated agents, explicitly authorize subagent
  spawning/delegation. Name the role(s), scope, expected compact outputs, and
  read/write limits; `dispatch <role>` alone is not sufficient.
- Explicitly require line-count-governance disposition in review and checklist
  artifacts, including owner/sunset metadata for any approved `3000+`
  generated/fixture exception.
- Require truthfulness labeling in evidence artifacts (`Static:` vs `Ran:`).
- For parameterized science, declare whether the package performs
  implementation, calibration readiness, empirical calibration, independent
  validation, or an explicit combination. Apply ADR-0042 and the
  `Science Implementation And Calibration Readiness` rule in
  `docs/work-packages/AGENTS.md`.
- Distinguish mathematical validity domains, evidence-supported calibration
  bounds, and `ASSUMED_FOR_EXECUTION` demonstration bounds. Never infer
  physiological or probability authority from a finite execution grid.
- If measured data are insufficient, require applicable deterministic
  parameter execution, observation-operator/objective proof,
  sensitivity/identifiability diagnostics, boundary/failure/equifinality
  reporting, and synthetic recovery where structurally meaningful. Do not
  represent synthetic recovery as empirical calibration.
- Require `artifacts/calibration-readiness-matrix.md` or a machine-readable
  equivalent. Disposition every readiness obligation from
  `docs/specifications/science-contract-spec.md` with status, evidence path,
  and applicability rationale; unresolved required rows block completion.
- Freeze measured-data roles as `CALIBRATION`, `INDEPENDENT_VALIDATION`, or
  `DIAGNOSTIC_ONLY` before result-bearing execution. Calibration data cannot
  carry an independent-validation claim.
- Do not encode a hold solely for unavailable or non-identifying data when
  authoritative in-scope science and calibration-readiness work can proceed.

4. Make physics authority explicit in canonical contracts
- For legacy migration packages, physics must be implemented in canonical
  `SC-*` science contracts first; package-local notes are not authority.
- Physics/equation authority defaults to
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` unless explicitly justified.
- Do not invent physics: every equation, constant, guard, and invariant must
  trace to canonical contract text plus provenance citations.
- Do not authorize surrogate, provisional, proxy, empirical stand-in, or
  heuristic process physics in production code. If actual physics authority is
  missing or contradictory, encode a hold-for-authority boundary. If actual
  physics authority is present and the routine is in scope, encode direct
  re-implementation of that physics as the required correction.
- Migration completeness is required for touched process families (for example
  ET, infiltration, snow/frost, runoff, routing): do not merge or disposition
  as complete with placeholder/proxy equations when baseline-authoritative
  routines are known and in scope for migration.
- Preserve variable naming continuity with legacy WEPP symbols; when runtime
  names differ, record explicit alias mappings in the relevant `SC-*` files.

5. Require tiered reference set in Dependencies and prompt required-reading
- Preserve full dependency traceability in `package.md`, but do not force all
  dependencies into mandatory pre-edit reading.
- Author kickoff required-reading in three tiers:
  - Core (always required before edits):
    - `/workdir/openWEPP/AGENTS.md`
    - `/workdir/openWEPP/docs/codex_exec_plans.md`
    - `/workdir/openWEPP/docs/work-packages/AGENTS.md`
    - `docs/work-packages/README.md`
    - package-local `package.md`
  - Conditional (required when applicable):
    - `docs/defect_closure_execplans.md` for defect-closure packages.
    - `docs/specifications/science-contract-authoring-procedure.md`,
      `docs/specifications/science-contracts/kernel-process-contract-profile.md`,
      and `docs/specifications/science-contracts/index.md` when the package
      edits canonical contracts, changes kernel decision logic, or introduces
      new/updated invariant authority.
    - `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md` plus pinned
      baseline source files for legacy migration/parity packages.
  - On-demand references (load only for touched mechanisms/surfaces):
    - Kernel-relevant canonical contracts in
      `docs/specifications/science-contracts/contracts/SC-*.md`
    - `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
    - upstream queue/hold-lift/disposition artifacts.
- Mechanical-only refactor carve-out:
  - For behavior-preserving refactor packages with no intended contract or
    kernel-semantic change, the science-contract authoring procedure/profile
    and full SC corpus may remain conditional/on-demand (not mandatory pre-edit
    reads) unless execution discovers an authority-touch condition.
- Require a package-local authority map artifact:
  - `artifacts/required-reading-map.md` with path, tier (Core/Conditional/
    On-demand), rationale, and applicability trigger.
- Canonical required-reading budget thresholds (single source of truth;
  local-repo files only):
  - `OK`: `<=400000` bytes.
  - `WARN`: `>400000` bytes.
  - `REQUIRES-JUSTIFICATION`: `>800000` bytes.
  - For `REQUIRES-JUSTIFICATION`, include why each heavy file must be pre-read
    and what cannot be deferred to on-demand.

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
- Kickoff prompt must state that a phase/increment may be marked complete only
  when all of its own required gates have direct current evidence. If a required
  gate depends on later work, the agent must stop at `HOLD` / `executed-hold`
  with the blocker named; it may not relabel the missing evidence as future
  scope after execution begins.
- For conservation-sensitive output/publication work, kickoff prompts must
  include a `Conservation/output acceptance:` line requiring operand lineage,
  anti-tautology tests, explicit rejected formulas, independent reconstruction,
  real closure/magnitude audit, and metadata/schema alignment. One-sided bounds
  or exact self-consistency checks may be recorded only as supporting sanity
  evidence, not acceptance.
- Kickoff prompt must include an explicit end-to-end execution statement (use
  `Autonomy:` line) for the declared scope.
- Kickoff prompt must include a `Subagent authorization:` line. Use `none` when
  no delegated work is required. When delegated work is required, the line must
  say the prompt explicitly authorizes subagent spawning/delegation and must
  list authorized role(s), scope, expected outputs, and read/write limits.
- DC-ExecPlan kickoff prompts must say `Close defect <id> end-to-end`, include
  the Correction Authority Envelope, require conversion to a contract-first fix
  when the seven-gate bar is met, prohibit relaying intermediate diagnostic
  steps into a new package, and prohibit `HOLD` while source reading,
  implementation, or validation remains possible inside the envelope.
- DC-ExecPlan kickoff prompts must include a `HOLD legitimacy audit:` line
  requiring the agent to name the boundary, cite the evidence proving it, list
  the in-envelope correction route considered, and explain why that route cannot
  close the defect in the current package.
- Kernel/process-physics kickoff prompts must include a `No surrogate physics:`
  line requiring actual contract-backed or baseline-authoritative physics in
  production code and banning surrogate/provisional/proxy/heuristic stand-ins.
- Direct-path/publication kickoff prompts must include a `Real consumer proof:`
  line requiring the public or downstream consumer to read the corrected path,
  with negative proof that wrappers, adapters, skeletons, shadow paths, and old
  compatibility paths are not carrying the closure claim.
- Kickoff prompt must include a `Required reading` section with explicit path
  references so onboarding/orientation does not require independent discovery.
  Structure this section as `Core`, `Conditional`, and `On-demand` lists.
- At minimum, `Core` must include:
  - `/workdir/openWEPP/AGENTS.md`
  - `/workdir/openWEPP/docs/codex_exec_plans.md`
  - `/workdir/openWEPP/docs/work-packages/AGENTS.md`
  - `/workdir/openWEPP/docs/work-packages/README.md`
  - the package-local `package.md`
- `Conditional` must include, when applicable:
  - `/workdir/openWEPP/docs/defect_closure_execplans.md` for defect-closure
    packages.
  - `docs/specifications/science-contract-authoring-procedure.md`,
    `docs/specifications/science-contracts/kernel-process-contract-profile.md`,
    and `docs/specifications/science-contracts/index.md` for contract/kernel
    authority edits.
- `On-demand` should contain phase-relevant canonical `SC-*` contracts and
  decision/queue artifacts, loaded only for touched mechanisms.
- Kickoff prompt must record required-reading budget metrics and disposition:
  - local bytes total,
  - `OK`/`WARN`/`REQUIRES-JUSTIFICATION` threshold outcome,
  - pointer to `artifacts/required-reading-map.md`.

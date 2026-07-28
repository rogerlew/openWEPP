# Policy Conflict Matrix

Status: `ORDER-1 PATCH PLAN`

`preserve` means the substance remains independently binding. `migrate` means
retain the purpose but remove planner/TESTGATE authority. `supersede` means the
prospective rule ends under ADR-0043. Historical facts are never rewritten.

## ADR-0039

| Decision | Disposition | Order-1 treatment |
| --- | --- | --- |
| 1 | `split` | Preserve one canonical testing strategy; supersede planner, receipt, and CI-lane authority within it |
| 2 | `preserve` | Keep the five validation moments as useful lifecycle guidance; edit-loop remains non-authoritative |
| 3 | `migrate` | Preserve declared deferral, named owner/trigger, truthful status, and terminal reconciliation; use package records, not planner admission or ledger authority |
| 4 | `preserve amended` | Preserve immediate broad correctness regression for critical change and campaign/release exact-head regression; ADR-0041 quality remains observational |
| 5 | `migrate` | Static mapping may advise; agents select from canonical requirements. Unknown or ambiguous production impact receives documented conservative escalation or authority clarification and is never silently narrowed. Remove mechanical planner admission and planner-caused blocking |
| 6 | `preserve` | Preserve ADR-0021/0041 quality model and explicit metric-package authority |
| 7 | `preserve` | Nextest remains an executor, never non-Cargo/science authority |
| 8 | `migrate` | Preserve explicit execution/documentation roots and content-, source-, and relevant-input-bound reuse proportional to the claim; remove receipt authority and planner-owned trust classes |
| 9 | `migrate` | Preserve assurance validity, impact, campaign-transfer, and release-transfer distinctions plus required approval/publication disposition under direct assurance governance; remove only mechanically generated planner pending state and linter authority |
| 10 | `supersede` | Remove planner-enforced age/count backstops and planner-defect recursion; campaign owners choose documented broad checkpoints |
| 11 | `preserve` | Keep deterministic explanation and rejection of learned replacement |
| 12 | `preserve` | Keep A2/A4/A5/A6 as visible investigation unless promoted and A0/A1/A3 as binding |
| 13 | `supersede/migrate` | Remove authenticated receipt requirement; preserve truthful provenance appropriate to the actual claim |
| 14 | `supersede` | Remove protected-namespace certification transaction and CI-app authority |
| 15 | `supersede/migrate` | Remove expected-parent admission, ledger CAS, and blocking cutover; preserve exact terminal diff and measured-friction requirement |

ADR-0039 rejected-alternative language is amended so agent judgment is the
normal decision path. Agents cannot silently narrow, waive, or misrepresent an
applicable rule, but no planner is needed to authorize their judgment.

## ADR-0040

| Decision | Disposition | Order-1 treatment |
| --- | --- | --- |
| 1 | `historical` | Retain the cutover fact; it creates no prospective linter requirement |
| 2 | `historical` | Retain measured result only; target utility is governed by the new cohort |
| 3 | `supersede` | No trusted runner is assigned to the linter |
| 4 | `preserve security` | Public untrusted code must not run on a trusted host; unrelated to linter authority |
| 5 | `supersede` | Remove bootstrap, receipt authentication, hosted verification, and attestation design |
| 6 | `historical/supersede` | Retain historical acceptance facts; remove prospective event gates |
| 7 | `supersede` | Planner/executor aggregate is no longer a normal path |
| 8 | `preserve amended` | No calendar scorecard; ordinary tool defects use stop-loss/manual continuation, not repair prerequisites |

Rejected alternatives remain historical. Direct installation and public-code
execution risks remain rejected; cloud/runner tradeoffs no longer define an
advisory linter.

## ADR-0041

| Decision | Disposition | Order-1 treatment |
| --- | --- | --- |
| 1 | `split` | Preserve correctness versus quality separation; supersede TESTGATE admission authority |
| 2 | `supersede/migrate` | Agents run applicable correctness commands directly; TESTGATE is not blocking authority |
| 3 | `supersede` | Remove planner/receipt `DEFERRED_TO_QUALITY_CI`; state quality posture directly in package evidence when relevant |
| 4 | `preserve` | Quality CI remains optional and observational; it must not depend on linter or TESTGATE identity |
| 5 | `preserve amended` | Campaign/release do not restore a coverage/CRAP gate; direct correctness obligations remain |
| 6 | `preserve` | Keep the quality model |
| 7 | `preserve` | Keep explicit metric-focused package closure |
| 8 | `preserve` | Keep complete-profile measurement semantics for operator-directed quality work |
| 9 | `supersede/migrate` | Remove TESTGATE priority/queue authority; retain the fact that defunct Omarchy records are historical |
| 10 | `preserve` | Preserve immutable historical bytes and original verdicts |

Rejected-alternative language continues to reject making quality debt a
universal correctness gate. References to a blocking TESTGATE are superseded.

## Operative Testing Strategy

| Sections | Conflict | Exact Order-1 amendment |
| --- | --- | --- |
| 1–4 | Planner/executor described as canonical authority | Make the document the requirement authority; describe direct agent selection/execution and optional lint advice |
| 6 | Mechanically generated plan/receipt lifecycle | Replace with concise package intent, exact commands/results, and terminal-diff reconciliation |
| 7 | Risk rules coupled to planner escalation | Keep risk guidance and critical broad regression; make agent disposition explicit |
| 8 | Planner inputs, schema, audit, recursive tooling gates | Move useful mappings to advisory rules; delete permission, readiness, pre-heavy, and tool-repair semantics |
| 9 | Execution architecture | Replace with direct canonical command execution; no linter execution |
| 10 | Receipts, reuse, trust, attestation | Record actual command/source/result; retain provenance proportional to claim; remove receipt authority |
| 11 | Campaign ledger and certification | Use package/campaign records without planner-owned admission or CI certification |
| 12 | Quality | Preserve ADR-0041 optional observation and explicit metric-package rules |
| 13 | Assurance planner | Retain static impact suggestions if useful and human scientific review; remove lifecycle ownership |
| 14 | CI lanes | Delete linter/TESTGATE lanes; document independent optional quality and release commands |
| 15 | Friction | Replace diagnostic budgets with mandatory linter stop-loss and manual continuation |
| 16 | Test failure semantics | Preserve independently of execution mechanism |
| 17 | Anti-evasion | Preserve underlying authority-suite and source-level protections; remove planner/verifier-specific admission |
| 18 | Review economy | Preserve useful review/evidence reuse; remove planner repair blocking |
| 19 | Transition | Mark superseded and point to ADR-0043 roadmap |
| 20 | References | Add ADR-0043 and remove prospective authority implications |

## Other Operative Documents

- Root and work-package `AGENTS.md`: replace authenticated terminal-plan and
  TESTGATE dispatch requirements with direct canonical selection, execution,
  evidence recording, and optional advisory-linter wording.
- `docs/standards/local-ci-gate-selection.md`: present commands and cost
  profiles directly; remove planner authority.
- `tools/local_ci/README.md`: mark TESTGATE controller historical pending
  deletion and document direct command use.
- Work-package templates and prompt guidance: require intent and exact terminal
  reconciliation, not planner permission or receipts.
- Science contracts and the correctness-authority model: no authority change.
- ADR-0042 and calibration-readiness schema: no authority change.

## Exact Order-1 Amendment Specification

Order 1 applies these replacements without re-deciding authority. Line numbers
are deliberately avoided; headings and quoted current phrases are stable patch
anchors.

### Landing Rule

ADR-0043 governs Order 1 from ratification. Order 1 runs direct manual
requirements selected from its exact diff; it does not request or require a
final planner plan, pre-heavy audit, receipt, TESTGATE transition, trusted
runner admission, or planner-certified closeout. Existing impact-map rows that
classify planner/policy changes as `CRITICAL` are migration inputs, not
authority to invoke the frozen system. This does not waive focused governance
guards, documentation checks, exact-diff reconciliation, or an independently
applicable correctness or security test.

### Root `AGENTS.md`

Under **Core Directives**, delete the three bullets beginning `Do not manually
dispatch TESTGATE`, `Exception: permanently queued`, and `Forest1 is the
trusted TESTGATE`. Insert:

> TESTGATE and the gate planner are frozen historical tooling. Do not dispatch,
> repair, or extend them for prospective work. Run applicable canonical
> validation commands directly and record what actually ran. Defunct Omarchy
> and retained forest1 records remain historical evidence and do not occupy a
> live queue.

Under **Validation Gates**, replace the first three bullets through `declared
owned-surface metric gates` with:

> `docs/standards/testing-and-gate-strategy.md` is the canonical authority for
> validation requirements, lifecycle timing, campaign deferral, evidence
> reuse, and escalation. Every package declares implementation intent before
> edits and reconciles the exact terminal diff before disposition. The optional
> advisory linter may report cited suggestions; it never selects, executes, or
> authorizes validation.
>
> Run every applicable increment requirement directly. Unknown or ambiguous
> production impact receives documented conservative escalation or authority
> clarification and is never silently narrowed. Critical changes retain
> immediate campaign-strength full correctness regression; campaign closure
> and release qualification retain exact-head full-workspace correctness.
>
> Coverage/CRAP is optional observational QA under ADR-0041 except for an
> explicitly authorized CQR/module-test-enhancement package's declared metrics.

### `docs/work-packages/AGENTS.md`

Replace **Pre-Heavy Closure Audit And Tooling Correction** in full with:

> ## Advisory Validation Planning And Tool Friction
>
> Agents select and execute applicable requirements directly. The advisory
> linter may help find mappings but creates no permission, hold, lifecycle
> state, receipt, or repair prerequisite. When it is absent, partial, wrong, or
> unavailable, use the manual route in ADR-0043 and continue the originating
> work. Record useful tool defects as ordinary debt. A known unmet underlying
> requirement still prevents truthful closure.

Under **Mechanical Refactor Requirements**, replace the bullet beginning
`Required terminal gates come from` with:

> Required terminal validation follows the declared intent, exact terminal
> diff, and canonical testing strategy. Critical refactors and campaign/release
> boundaries retain full-workspace correctness. Focused, quick, frost, and
> erosion profiles claim only the surfaces they execute.

Under **Observational Quality And Explicit Metric Packages**, delete the bullet
beginning `ADR-0041 requires TESTGATE`. In the preceding bullet, remove the word
`TESTGATE,`.

Under **Validation Checklist**, replace the two bullets beginning `Exact
intent/terminal gate plan` and `Exact TESTGATE quality deferral` with:

> - Declared implementation intent, exact terminal-diff reconciliation, and
>   exact commands/results for every applicable current-scope requirement.
> - Selected full-workspace correctness regression at critical, campaign, and
>   release boundaries; quality remains observational unless explicitly owned.

Every other consumer, conservation, science, calibration, review, verification,
and anti-evasion clause remains unchanged.

### `docs/standards/testing-and-gate-strategy.md`

- In **Authority And Precedence**, delete normative-machine/planner precedence.
  Insert: `This document and applicable contracts/packages establish
  requirements. Agents apply them directly. Advisory-linter findings are
  optional information and never authority or evidence.`
- In **Definitions**, delete definitions for intent plan, terminal plan,
  receipt, pre-heavy audit, ledger transition, tooling-defect lifecycle, and
  certification. Retain `gate` only as `a validation requirement established
  independently of the advisory linter`.
- Keep **Principles** and **Test And Check Families**, replacing `mechanically
  selected` with `mapped from the exact change and documented by the agent`.
- In **Lifecycle Levels**, retain the five levels and their substantive
  correctness obligations. Replace machine plan/receipt/ledger language with
  package-declared intent, direct commands/results, named campaign deferral,
  and exact-terminal-diff reconciliation.
- In **Risk Classification And Escalation**, replace `planner assigns` with
  `the agent assigns and records`. Preserve conservative unknown-impact
  escalation and every actual critical trigger; a linter-only change is not
  critical merely because it is tooling.
- Replace **Mechanical Impact Planning** through **Campaign Gate Ledger** with:

> ## Advisory Impact Analysis And Direct Execution
>
> Agents inspect the canonical Git change set, Cargo and declared non-Cargo
> dependencies, contract/test bindings, external-authority registry, assurance
> dependencies, and package requirements. Unknown or ambiguous production
> impact receives documented conservative escalation or authority
> clarification and is never silently narrowed.
>
> `tools/validation/workplan-lint`, when available, may read the Order-0
> allowlist and emit cited findings or inert suggested argv. It does not execute
> validation, suggested, package-declared, workflow, remote, or user-controlled
> commands; only the frozen literal read-only Git inspection allowlist may run.
> It does not issue permission, write plans/receipts/ledgers, create lifecycle
> state, or certify evidence. Partial or unavailable analysis invokes the manual
> route and cannot block originating work.
>
> Agents run selected commands directly and retain evidence proportional to the
> claim: exact command, working directory, source identity, result, and required
> outputs. Reuse requires identical source, execution/documentation roots, and
> all relevant bound inputs unless those inputs are demonstrably excluded.
> Campaign deferrals and obligations live in ordinary package/campaign records
> with named owners and triggers. Deferred is not passed or waived.

- Under **Observational Coverage And CRAP**, delete **TESTGATE disposition**.
  Preserve optional observation, complete-profile measurement, and explicit
  metric-package closure unchanged.
- Under **Assurance Impact And Deferral**, delete planner-created record/state
  mechanics. Preserve static advisory dependency suggestions and insert:
  `Applicable validity, impact, approval, publication, campaign-transfer, and
  release-transfer dispositions remain direct assurance-governance duties and
  must resolve at their governing boundary.`
- Replace **CI And Operational Lanes** with:

> ## CI And Operational Posture
>
> The advisory linter has no CI workflow, trusted runner, concurrency,
> attestation, or promotion role. Optional quality observation and separately
> authorized release workflows remain independent of the linter.

- Replace **Performance And Friction Budgets** with the definitions, protocol,
  thresholds, and stop-loss in
  `friction-baseline-and-success-metrics.md`. The stop-loss disables only the
  linter.
- Preserve **Failure, Flakiness, And Nondeterminism**, **Anti-evasion And
  Security**, and **Review And Audit Expectations**, deleting only
  planner/receipt-specific coupling. Replace **Transition Requirements** with:
  `ADR-0043 and its ordered roadmap govern removal. Historical evidence keeps
  its original bytes and meaning.`

### `docs/standards/local-ci-gate-selection.md`

Replace the opening paragraph with:

> This standard lists direct local validation commands and cost profiles.
> Lifecycle timing, campaign deferral, evidence reuse, and escalation follow
> `testing-and-gate-strategy.md`. Agents choose and record applicable commands;
> advisory-linter suggestions are optional and execute nothing.

Under **Reporting Rules**, replace the bullet beginning `Record the terminal
plan` with:

> Record the governing requirement and exact command/result for each closure
> check. A focused pass claims only its affected surface.

All command tiers and proportionality rules remain.

### `tools/local_ci/README.md`

Rename **TESTGATE Increment Execution** to **Historical TESTGATE Interface** and
replace its contents through **Assurance Amendment Receipts** with:

> TESTGATE, its controller, planner transitions, receipts, ledgers, recovery,
> and forest1 workflow are frozen historical interfaces pending ordered
> deletion. Do not invoke them for prospective work. Use the direct commands in
> `docs/standards/local-ci-gate-selection.md` and record exact results in the
> owning package. Historical verification is read-only and confers no
> prospective authority.

Keep independent CQR quality-observation and assurance-amendment sections, but
remove any dependency on TESTGATE identity or priority.

### Prompt Guidance And Templates

In `docs/standards/prompt-wording-guidance.md`, replace `gate lifecycle,
boundary assignment, escalation, and receipt currency` with `governing
validation obligations, direct command execution, exact evidence, and
escalation rationale`. Replace the sentence beginning `When a terminal plan`
with:

> When a critical classification, campaign/release boundary, or explicit
> package requirement applies, name it and execute its direct commands; do not
> delegate authority to linter output.

In `docs/work-packages/templates/cqr-nightly-package.md` and
`cqr-nightly-kickoff-prompt.md`, replace `selected by the terminal plan` with
`selected from the declared objective, exact diff, and canonical strategy` and
replace terminal-plan reconciliation with intent/exact-diff reconciliation.
Preserve CQR receipt language only where it identifies the optional
quality-evidence intake intrinsic to that metric-focused workflow; it must not
be described as general increment admission.

### Enforcement Guards, Historical Identity, And Status

Order 1 removes
`tests/integration/testgate_ci_executor_contract.rs` and its `Cargo.toml`
registration because the test asserts execution/pre-heavy source literals.
It migrates `testgate_align_authority_contract.rs` and its registration:
delete blocking planner/schema literal assertions, retain independently
applicable direct-governance and anti-evasion assertions, and defer
advisory-schema behavior to Order 3.

Before editing the live testing strategy, Order 1 records the generation-17
SHA-256 and Git blob in the frozen historical-consumer registry exactly as
specified in `migration-quarantine-deletion-map.md`. Historical verifiers use
that object, never the new live path.

Order 1 applies the exact frozen-package status overlay in the migration map.
No incomplete planner prerequisite remains locally `ACTIVE`; historical
progress and results remain unchanged.

# WSHED-W11B Kickoff Agent Prompt

Scope: local repository science-contract/kernel implementation and defect
closure; flat-file reads/edits and local commands only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute every phase in `package.md` sequentially through final
disposition. Close defects `GAP-ROUTE-014-A`, `GAP-ROUTE-014-B`, and
`WSHED-W11B-DIRECT-001` end-to-end. Do not request a follow-on package for an
intermediate diagnostic step.

## Required Reading

Core:

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260710-wshedw11b-channel-interval-sediment-implementation-001/package.md`

Conditional before kernel/test edits:

- `/workdir/openWEPP/docs/defect_closure_execplans.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/crates/AGENTS.md`
- `/workdir/openWEPP/tests/AGENTS.md`
- `/workdir/openWEPP/docs/standards/local-ci-gate-selection.md`

On-demand:

- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- WSHED-W11A `artifacts/w11-handoff.md`, `codex_reconfirmation2.md`, and
  `codex-review-disposition.md`
- held WSHED-W11 intake/source/support artifacts
- pinned baseline `wshchr`, `chrqin`, `chnrt`, `dcap`, `detach`, `case12`, and
  `case34` sources
- `SC-SYSTEM-001`, `SC-SED-001`, ADR-0036, and unit governance only when the
  touched mechanism requires them

Required-reading budget: `411669` local bytes, disposition `WARN` (`>400000`,
`<=800000`); map: `artifacts/required-reading-map.md`. The large
`docs/work-packages/README.md` remains Core because repository governance makes
the active/held catalog mandatory; mechanism contracts remain On-demand.

Files: only the Intended Write Set in `package.md`.

Task: implement the `SC-ROUTE-001` v53 interval channel-sediment lane, correct
both `GAP-ROUTE-014` terminals, pass all eleven W11A vectors, and prove the real
two-channel production consumer plus independent conservation closure.

Constraints: contract-first sequencing; canonical SC authority; pinned baseline
provenance `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`; typed guards; no silent
defaults; no canonicalize-and-proceed for domain violations. Confirm or amend
contracts, implement contract-derived tests, and complete the pre-implementation
gate before production edits.

DC closure: do not hold while source reading, contract/test work, production
implementation, or validation remains possible inside the Correction Authority
Envelope. Apply the conversion rule and seven-gate bar in `package.md`.

HOLD legitimacy audit: any hold must name the declared boundary, cite proof,
list the considered in-envelope correction route, and explain why it cannot
close now. Effort, uncertainty, or a partial compatibility path is not a hold
boundary.

No surrogate physics: production code must implement actual v53/pinned-baseline
physics. Surrogate, provisional, proxy, heuristic, wrapper, skeleton, or shadow
substitutes are forbidden.

Real consumer proof: the downstream channel and public daily consumer must read
the corrected interval path. Prove adapters, old scalar compatibility state,
event-peak fractions, and event-scalar sediment do not carry the W11B claim.

Conservation/output acceptance: complete operand lineage before production
edits; separate every plausible alias; explicitly reject raw-total `qlat`,
`qlat/lc`, event-peak, active-span, and daily-mass formulas; independently
reconstruct produced water/sediment closure and magnitude; align metadata/schema.
One-sided bounds and exact self-consistency are supporting evidence only.

Subagent requirement: REQUIRED. This prompt explicitly authorizes subagent
spawning/delegation to source-lineage reviewers, two scientific/Rust reviewers,
two verification agents, and `comparator_suite_runner` for heavy full-closure,
comparator, release-build, and production-CLI runs. Expected outputs: compact
findings/metrics, commands, and log paths; write access: read-only except
explicitly assigned package review/verification artifacts. Do not run heavy
batches on the parent model while the runner is available; if unavailable,
record command-level evidence before local execution.

Gate non-deferral: a phase or package is complete only when every required
current-scope gate has direct current evidence. Do not relabel an unmet gate as
future scope after execution begins.

Autonomy: execute end-to-end and update all required artifacts without asking
for additional direction unless a declared hard boundary is proven.

Outputs: completed implementation, tests, package artifacts, review disposition,
dual verification, gate results, final disposition, and defect-shaped handoff.

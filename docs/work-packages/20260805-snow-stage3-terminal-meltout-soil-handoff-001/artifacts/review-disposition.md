# Review Disposition

Status: complete / all findings dispositioned / follow-ups PASS

| Finding | Source | Disposition | Rationale / required change |
| --- | --- | --- | --- |
| Terminal integrator under-specified | Review A finding 1 | Accepted | Require an implicit or error-controlled terminal solve, contract tolerances, event bracketing, convergence failure, and warming/cooling flux reevaluation. Replace `exact` with `localized within contract tolerance`. |
| Receiving surface over-specified | Review A finding 2 | Accepted | Replace fixed `snow_free_wet_soil` with a typed snow-free land-surface regime selected from actual cover, frost, infiltration, and ponding state. Add soil, runoff-partition, evaporation, and pinned baseline authority. |
| Snow-only persistence cannot support seasonal claims | Review A finding 3 | Accepted | Require coupled surface/soil thermal-water-frost shadow state for restart/Snowbird claims; otherwise limit evidence to event-local diagnostics. |
| Terminal liquid and infiltration-first routing omitted | Review A finding 4 | Accepted | Name terminal retained-store release, surface-liquid supply, infiltration, soil storage, ponding/overflow, evaporation, and residual runoff. Distinguish snow drainage from hillslope runoff. |
| Vapor/phase event chronology incomplete | Review A finding 5 | Accepted | Localize the earliest combined melt/sublimation exhaustion event; recompute post-event vapor flux and define simultaneous precipitation/deposition and reappearance chronology. |
| Authority hold occurs after code | Review A finding 6 | Accepted | Make complete receiving-surface and coupled-state authority a Phase-1 go/no-go gate. No production edit may precede its pass; otherwise close `HOLD` or prospectively split an authority successor. |

The reviewer agreed with the proposal's rejection of direct transfer of
snow-computed terminal excess into soil. That decision is retained.

## Phase-1 HOLD Review Findings

| Finding | Source | Disposition | Rationale / correction |
| --- | --- | --- | --- |
| Closeout gate evidence remained queued | QA finding 1 | Accepted | Replaced `gate-results.md` with exact Phase-1/docs-only results and explicit non-applicability of Rust/domain/heavy execution after the authority `NO-GO`; added package validation. |
| Disposition/status wording was premature | QA finding 2 | Accepted | Package and catalog retained review-pending wording until both verifiers passed, then advanced to the final executed-HOLD state. |
| Review disposition omitted execution reviews | QA finding 3 | Accepted / in progress | Added `review_agent_qa.md`; Rust review and dual-verifier outcomes will be appended before final disposition. |
| Event-local work incorrectly depended on coupled seasonal-state authority | Rust review finding 1 | Accepted | Corrected the pre-implementation gate, worker handoff, and snow roadmap: land-surface-energy authority blocks all terminal implementation now; coupled-state authority additionally blocks persistence, restart, Snowbird, and seasonal claims. |
| Pinned WEPP provenance described only by textual anchors | Rust review finding 2 | Accepted | Recorded the differing checkout HEAD, verified the pinned commit object, and used commit-qualified source reads for the named routines. |
| Markdown evidence count stale after adding the HOLD audit | Rust review finding 3 | Accepted | Updated the count to `26`; QA independently confirmed lint and validation at that count. |

QA follow-up and targeted Rust follow-up passed with no remaining findings.

## Terminal Verification Findings

| Finding | Source | Disposition | Rationale / correction |
| --- | --- | --- | --- |
| HOLD audit omitted the two canonical A0 admission routes | Verifier A finding 1 | Accepted | Added explicit ADR-0024 and ADR-0028 applicability/success tests plus scoped owners and next gates to `hold-legitimacy-audit.md`. |

Verifier A follow-up and independent verifier B both passed with no remaining
substantive findings. Their exact checks are recorded in
`verification_agent_a.md` and `verification_agent_b.md`.

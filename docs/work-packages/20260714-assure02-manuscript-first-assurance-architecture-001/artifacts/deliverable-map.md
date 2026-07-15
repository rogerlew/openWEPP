# ASSURE-02 Deliverable Map

Status: agent-executable deliverables and dual verification pass; held for
explicit user or named scientific-steward acceptance

| Closure obligation | Canonical deliverable | Demonstration evidence |
| --- | --- | --- |
| Architecture decision | `docs/decisions/0038-manuscript-first-scientific-assurance-publication.md` | V1 failure and v2 boundary stated as a proposed decision pending human acceptance |
| V&V philosophy | `docs/governance/openwepp-verification-validation-strategy.md` | Verification/corroboration/application asymmetry; public manuscript; prohibited shortcuts |
| Report/supplement/machine/application boundary | `docs/governance/scientific-assurance-v2-architecture.md` | Reader path, documentation cross-references, owners, build boundary |
| Public report standard | `docs/standards/scientific-model-evaluation-report.md` | Conventional manuscript structure, study-type rules, claim envelope, quantitative and review requirements |
| Lifecycle and ownership | `docs/governance/scientific-assurance-dossier-lifecycle.md` | Staging-only drafts/reviews, approval, publication, supersession, release transfer, zero-report state |
| Source/build/dependency contract | `docs/governance/scientific-assurance-v2-source-build-contract.md` | Human-authored source, stable identities, one/all planning, deterministic build, agent procedure, nextest boundary |
| Research basis | Package `research-basis.md` | EPA, Oreskes, Bennett, TRACE, Moriasi, FAIR, AGU, NASA, and USGS synthesis |
| Evidence-led pilot selection | Package `pilot-candidate-inventory.md` and `pilot-selection-decision.md` | Groundwater recurrence selected; competing kernels and scope limits recorded |
| Real-evidence manuscript prototype | Package `prototype-linear-groundwater-reservoir-evaluation.md` | Scientific-paper structure; analytical vector; H2637 reconstruction; consumer path; limitations; reproduction |
| Claim/evidence traceability | Package `groundwater-claim-evidence-matrix.md` and `groundwater-current-tree-confirmation.md` | Claims, complete paths, commands, test output, commits, hashes, units, tolerances, values, and limits |
| V1 retirement | `docs/planning/scientific-assurance-v2-migration-plan.md` | Exact inventory, neutral zero-report target, preservation, link repair, gates, rollback |
| Independently closable implementation | `docs/planning/scientific-assurance-v2-implementation-roadmap.md` | ASSURE-03, ASSURE-04A–D, ASSURE-05, consumers, gates, and rollback |
| Current release conflict and transition safety plan | `docs/governance/openwepp-release-procedure-draft.md` and `docs/planning/scientific-assurance-v2-migration-plan.md` | `ASSURE03-REL-001` truthfully records that the current script/CI still snapshot the v1 candidate; ASSURE-03 must install an executable fail-closed guard and zero-report path |

No canonical v2 code, `assurance/` source, generated/public assurance page,
snow/frost assessment, or WEPPcloud vendor file is changed by ASSURE-02.

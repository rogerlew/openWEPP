# ASSURE-04D Required-Reading Map

Status: current at intake

Evidence class: Ran for byte counts and Static for applicability

Core reading completed before package/source edits totals 128,316 bytes.
Triggered conditional reading adds 20,361 bytes, for 148,677 bytes total and an
`OK` disposition under the canonical 400,000-byte threshold.

| Path | Tier | Bytes | Reason |
| --- | --- | ---: | --- |
| `AGENTS.md` | Core | 10,822 | Repository-wide authority and validation gates |
| `docs/work-packages/AGENTS.md` | Core | 21,107 | Package, review, non-deferral, consumer, CRAP, and line-count rules |
| `docs/codex_exec_plans.md` | Core | 20,708 | Autonomous living-plan requirements |
| `docs/decisions/0038-manuscript-first-scientific-assurance-publication.md` | Core | 4,607 | Accepted publication architecture decision |
| `docs/governance/scientific-assurance-v2-architecture.md` | Core | 9,562 | Record separation, ownership, and build boundary |
| `docs/governance/scientific-assurance-dossier-lifecycle.md` | Core | 13,313 | Lifecycle, approval, independence, transfer, and snapshot authority |
| `docs/governance/scientific-assurance-v2-source-build-contract.md` | Core | 9,576 | Root, lock, publish, snapshot, and dependency contract |
| `docs/standards/scientific-model-evaluation-report.md` | Core | 13,067 | Domain-reader and publication-review requirements |
| `docs/standards/usersum-authoring-style-guide.md` | Core | 13,093 | Public catalog/render/link conventions |
| `docs/planning/scientific-assurance-v2-implementation-roadmap.md` | Core | 8,405 | ASSURE-04D prospective outcome and gates |
| `docs/work-packages/20260715-assure04c-deterministic-manuscript-assembly-001/artifacts/worker-handoff.md` | Core | 1,604 | Frozen predecessor API and boundary handoff |
| `crates/AGENTS.md` | Conditional, triggered | 5,171 | Rust crate edits |
| `tests/AGENTS.md` | Conditional, triggered | 4,534 | Integration-test edits |
| `docs/standards/AGENTS.md` | Conditional, triggered | 3,328 | Prompt/standard routing |
| `docs/standards/prompt-wording-guidance.md` | Conditional, triggered | 9,780 | Required kickoff/delegation wording |

On demand, read only the touched sections of `crates/openwepp-assurance/src/`,
the v2 schemas/source, and adjacent v2 integration suites. This package is not
kernel-affecting, so science-contract authoring/profile and legacy Fortran
sources are not required pre-edit reading.

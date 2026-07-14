# Independent Review A: Scientist-Facing V&V Strategy Inversion

Recommendation: **HOLD** pending correction and disposition of the two findings
below.

Static: I reviewed `AGENTS.md`, `docs/work-packages/AGENTS.md`, the active
package, the revised V&V strategy, the new scientific-assurance-dossier
standard, the four changed documentation index entries, the package evidence,
ADR-0028, and the correctness authority model. I also checked the integrated
validation campaign's terminal package/assessment/disposition because the
strategy characterizes that campaign. I did not read Reviewer B's artifact.

Ran: I executed Markdown lint, local-link resolution, spelling-normalization
preview, diff integrity, changed-file scope, and size/proportionality checks.

## Findings

### High: `VVINV-A-001` — Baseline dossiers are accidentally exempted from independent review

The standard requires scientific review and independent evidence verification
in workflow step 6
(`docs/standards/scientific-assurance-dossier.md:154`), then exempts baseline
dossiers from **steps 4 through 6** when no new verdict-bearing analysis is run
(`docs/standards/scientific-assurance-dossier.md:161`). That exemption includes
review of the dossier's evidence classification, provenance mapping,
limitations, and public summary.

This conflicts with the same standard's required review-and-disposition layer
(`docs/standards/scientific-assurance-dossier.md:90`) and with the strategy's
statement that every dossier reports independent review and that independent
review with finding disposition is part of the minimum audit basis
(`docs/governance/openwepp-verification-validation-strategy.md:141` and
`:160`). It is especially material because baseline dossiers are the first
adoption deliverable (`docs/governance/openwepp-verification-validation-strategy.md:258`).
Historical scientific evidence need not be rerun or retrospectively
pre-registered, but the new public dossier still needs proportional independent
review that it has not strengthened, omitted, or misclassified that evidence.

Required resolution: narrow the baseline exception to prospective campaign
controls that cannot apply retrospectively. Require every published dossier,
including a baseline inventory, to receive proportional scientific review and
independent verification of its evidence mapping, limitations, and disposition.

### Medium: `VVINV-A-002` — The workflow selects an “initial disposition” before examining evidence

Workflow step 1 tells authors to define an initial disposition before step 2
inventories evidence and step 5 derives the summary from results
(`docs/standards/scientific-assurance-dossier.md:144` through `:155`). In a
normative scientific workflow, that ordering can turn a disposition into a
target and encourage confirmation-oriented evidence selection. It is not the
same as the strategy's sound requirement to freeze intended use, evaluation
roles, metrics, and criteria before verdict-bearing execution
(`docs/governance/openwepp-verification-validation-strategy.md:217`).

Required resolution: begin a new dossier at `NOT_ASSESSED`, or define only the
question and claim envelope in step 1. Assign the evidence-derived disposition
after inventory, any verdict-bearing execution, and review. Preserve explicit
`NOT_ASSESSED` or `INSUFFICIENT_EVIDENCE` outcomes when the evidence cannot
support a stronger conclusion.

## Audience, Strategy, And Proportionality Assessment

Static: The inversion otherwise succeeds. The opening explains verification,
validation, and use qualification in language accessible to domain scientists
and practitioners, then asks the practical questions a reader needs answered.
The three-layer dossier structure keeps decision meaning and named scientific
evidence ahead of hashes, commands, and internal governance identifiers.

Static: The current-state section is appropriately bounded. It identifies
verification strengths, explicitly says the integrated campaign is not broad
empirical validation, names the five-climate SNOTEL SWE/depth/density evidence,
and states that this evidence does not establish runoff, erosion, plant growth,
routing, or watershed-scale support. ADR-0028 confirms the named corpus and
bounded rubric, while the integrated campaign's terminal package and assessment
confirm its `PASS-INTEGRATED-VALIDATION` status. No new scientific verdict is
asserted.

Static: The roadmap genuinely puts public evidence and named-data science
before infrastructure: Phase 1 publishes honest baseline dossiers, Phase 2
executes claim-driven scientific plans, and only Phase 3 standardizes recurring
manifest/automation needs. A crate, database, PROV export, or RO-Crate export is
expressly optional and later. The minimum audit kernel can be satisfied with
Markdown and retained manifests.

Ran: The strategy is `367` lines / `2270` words and the standard is `215` lines
/ `1286` words. Their combined `3556` words remain below the prior strategy's
recorded `4930` words while providing progressive disclosure. I found no
unnecessary subsystem, schema, service, or query-engine requirement.

## Exit-Criterion Audit

| Criterion | Status | Reviewer A evidence |
| --- | --- | --- |
| `VVINV-001` | `PASS` | Scientific audiences are named at strategy lines 11-12; the seven transparency questions are lines 63-74. |
| `VVINV-002` | `PASS` | Phase 1 is public baseline publication at lines 258-268, and negative/unknown dispositions are explicit at lines 76-78. |
| `VVINV-003` | `PASS` | Verification is required to interpret empirical evidence at lines 106-116 and is one visible dossier dimension at standard lines 83-97. |
| `VVINV-004` | `PASS` | Current strengths, gaps, SNOTEL basis, and explicit non-transfer limits are stated at strategy lines 235-254 without a new empirical verdict. |
| `VVINV-005` | `PASS` | Claim-driven plans are Phase 2; generalized recurring infrastructure is Phase 3 at lines 270-294. |
| `VVINV-006` | `FAIL` | The structure, visuals, statuses, limitations, and audit kernel are present, but `VVINV-A-001` and `VVINV-A-002` leave the normative authoring/review workflow unsound. |
| `VVINV-007` | `PASS` | Strategy lines 30-34 and standard lines 117-119 explicitly reject generalized tooling as a publication prerequisite. |
| `VVINV-008` | `PASS` | Ran: Markdown lint validated 13 files with 0 errors/warnings; all 59 local links resolved; spelling preview found no change in the strategy, standard, or package files; `git diff --check` passed. |
| `VVINV-009` | `NOT RUN` | Dual review, finding disposition, remediation, and dual verification are still in progress. This correctly blocks `COMPLETE`. |
| `VVINV-010` | `PASS` | Ran/Static: repository status contains documentation only and no `.rs`, executable, fixture, dataset, contract, workflow, or gate change. Reproduction guidance exposes no credential or private-data instruction. |

## Navigation And Source Checks

Ran: All 59 relative links in the strategy, standard, and changed indexes resolve
to existing repository paths. The root documentation index, governance index,
standards index, and work-package catalog all expose the new/revised documents.
The strategy and standard link to each other. External primary-source endpoint
inspection confirmed the NASA, EPA, ASME, W3C, and RO-Crate destinations; some
PDF/DOI endpoints rejected or did not complete automated retrieval, so this
review does not claim a complete external availability audit. Bibliography
entries `R-114` through `R-124` exist in
`references/annotated_bibliography.md`.

## Gate Non-Deferral And Final Recommendation

Static: The package remains `ACTIVE`, its gate artifact records dual review and
verification as `NOT RUN`, and it does not claim `COMPLETE`. That is the correct
non-deferral posture. Documentation-only exemption from Rust closure and the
adjudicated CRAP gate is legitimate; it does not waive the documentation,
review, disposition, or verification criteria.

The scientist-facing strategy itself is strong, proportionate, honest about
current support, and correctly ordered. The two standard-workflow findings are
central to the auditability of the Phase-1 baseline dossiers and must be fixed
and dispositioned before closure. Reviewer A therefore recommends **HOLD**.

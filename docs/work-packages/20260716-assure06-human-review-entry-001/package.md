# ASSURE-06 — Human Accountability And Review Entry

Status: HOLD-INDEPENDENT-HUMAN-REVIEW

This ExecPlan is a living document governed by `docs/codex_exec_plans.md`.

## Purpose

Record Roger Lew's explicit acceptance of the ASSURE-06 report-lead role and
his material participation in the openWEPP snow/frost campaign, then move the
exact snow/frost report from `DRAFT` to `IN_REVIEW` without representing that
acceptance as independent scientific or reproduction/publication approval.

## Authority And Frozen Intake

- Operator statements on 2026-07-16: “I, Roger Lew, reviewed and approve this
  report,” “report lead,” and “I lead the development of the snow and frost
  campaign in openWEPP.”
- Intake report manifest SHA-256:
  `feb093721686875ddf1ef59e1f0c1f8a6981608a8dad2e9e602d17733afe9d3a`.
- Intake manuscript SHA-256:
  `4290978558311fbf14044640cf062b7b6347f314bba1b2344dececf98b769d4f`.
- Intake supplement SHA-256:
  `4ae87bd47a144b310e256f878c0211e0efb2d54723c45fc74994ed44264d5cfd`.
- The active V2 lifecycle, source/build, report, and publication contracts
  remain binding. The operator's report-lead acceptance does not waive their
  independence rules.

## Objective

Register Roger Lew as a production-domain human `report_lead` and
`material_producer`; disclose Codex as draft author, build maintainer, and
material producer; update the manuscript, supplement, and agent-assistance
record; authorize a frozen independent-review charge; calculate and bind the
exact subject and finding-ledger roots; and stop with publication, export,
release transfer, and vendoring prohibited.

## Included Scope

- Human-principal registration from the operator's direct attestation.
- Report-lead and material-producer assignment.
- `DRAFT` to `IN_REVIEW` transition with a pending review record.
- Exact root calculation over deterministic staging.
- Identity cascade, catalog, roadmap, package evidence, and focused gates.
- Independent review of lifecycle truthfulness and root reproducibility.

## Excluded Scope

- Treating Roger Lew as an independent scientific or reproduction/publication
  reviewer for this report.
- Inventing Anurag, Erin, or any other principal, competence declaration,
  finding, disposition, approval, stewardship, or release authority.
- `APPROVED` or public lifecycle state, release transfer, tracked publication,
  export, vendoring, or WEPPcloud changes.
- Scientific claim, method, result, dataset, figure, or model changes.

## Intended Write Set

- `assurance/v2/principals.yaml`
- `assurance/v2/catalog.yaml`
- `assurance/v2/README.md`
- `assurance/v2/reports/snow-and-frozen-soil-process-evaluation/manuscript.md`
- `assurance/v2/reports/snow-and-frozen-soil-process-evaluation/supplement.md`
- `assurance/v2/reports/snow-and-frozen-soil-process-evaluation/evidence/agent-assistance-packet.json`
- `assurance/v2/reports/snow-and-frozen-soil-process-evaluation/report.yaml`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260716-assure06-human-review-entry-001/**`
- `tests/integration/assurance_v2_source_contract.rs`
- `tests/integration/assurance_v2_publication_contract.rs`

Everything else is read-only. The prior ASSURE-06 package and ASSURE-05 source
remain historical evidence and must not be rewritten.

## Progress

- [x] (2026-07-16) Recorded operator role and material-producer attestation.
- [x] (2026-07-16) Scaffolded the bounded review-entry package.
- [x] (2026-07-16) Registered principals and updated human-boundary disclosures.
- [x] (2026-07-16) Entered review and bound exact subject and finding-ledger
  roots.
- [x] (2026-07-16) Ran focused lifecycle, staging, spelling, documentation, and boundary
  gates.
- [x] (2026-07-16) Completed dual independent coding-agent review, disposition,
  and dual terminal verification without actionable findings.

## Review Charge

Independent reviewers must assess the scientific claims, formulation and prior
knowledge, dataset suitability, development/evaluation separation, methods and
metrics, uncertainty and contrary evidence, conclusion support, limitations,
and transfer language. A distinct reproduction/publication reviewer must
independently reconstruct material results and review audience fit,
accessibility, cross-references, and public research-object completeness.

Roger Lew is the report lead and a material producer. He is therefore excluded
from both independent approval roles. Codex is a material producer and build
maintainer and is also excluded from those approval roles.

## Gates

1. Exact result reproduction remains equal at 188 values.
2. Named and all-source validation and planning pass.
3. Two unrelated report builds/checks are byte-identical.
4. Calculated review roots exactly match the bound roots.
5. American-English normalization check passes.
6. Focused assurance editorial and publication/lifecycle contracts pass.
7. Protected public report count remains zero; ASSURE-05 is unchanged.
8. Markdown lint and `git diff --check` pass.
9. Dual independent coding-agent review, disposition, and dual terminal
   verification pass. These checks do not replace required human reviews.

No production Rust changes are authorized, so the adjudicated CRAP gate is
exempt.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to two independent review subagents and two terminal verifier subagents for
read-only review of lifecycle truthfulness, role conflicts, exact-root
reproduction, and protected boundaries. Expected outputs are concise findings
and verification evidence returned to the parent for package artifacts; write
access is read-only. No subagent may create or infer a human identity, review,
approval, release authority, publication, export, or vendoring permission.

## Exit Criteria

- Roger Lew is registered and identified as report lead and material producer.
- The report is `IN_REVIEW` with exact roots and no approval declarations.
- The independent reviewer roles remain unassigned and visibly required.
- No scientific content changes beyond accountability/lifecycle disclosures.
- Public report count remains zero and protected surfaces remain unchanged.
- Technical review and verification close with no undispositioned finding.

## Terminal Disposition

The maximum authorized result is `HOLD-INDEPENDENT-HUMAN-REVIEW`. Advancement
requires distinct qualified humans to complete scientific and
reproduction/publication review, disposition findings against the exact root,
and supply the additional assurance-steward approval required for an approval
lock. Release transfer and publication remain separate later gates.

## Decision Log

- Decision: classify Roger Lew as report lead and material producer, not an
  independent approver. Rationale: Roger explicitly accepted the report-lead
  role and stated that he led the underlying snow/frost development campaign;
  the V2 independence contract makes those roles incompatible with independent
  scientific and reproduction/publication approval. Date/Author: 2026-07-16,
  Roger Lew/Codex.
- Decision: amend the write set for the source-contract integration test.
  Rationale: the focused editorial profile exposed its obsolete assertion that
  both admitted production sources must remain `DRAFT`; the contract supports a
  mixed catalog and the snow/frost source now truthfully enters `IN_REVIEW`.
  The correction is limited to the expected lifecycle and a descriptive test
  name. Date/Author: 2026-07-16, Codex.
- Decision: amend the write set for the publication-contract synthetic fixture.
  Rationale: the full target copied the complete real two-report catalog before
  constructing its own synthetic two-report publication fixture. Once the real
  snow/frost source entered review, publish-all correctly rejected that
  unrelated noncurrent source. The fixture now retains only its intended
  groundwater source before cloning the synthetic peer, matching the existing
  assembly-contract fixture boundary. Date/Author: 2026-07-16, Codex.

## Outcomes And Retrospective

Roger Lew is now registered as the accountable snow/frost report lead and a
material producer. Codex is disclosed as draft author, build maintainer, and a
material producer. Those facts are visible in the governed source and rendered
report, and both principals remain excluded from independent approval roles.

The report entered `IN_REVIEW` at subject root
`11a473da9b26a31d017d1581e194136082e3bc8f79edefb95051546406e5aa4e`
and finding-ledger root
`595f8ead6ada47b1cf7bbcb25bfb1f057b937451bc3ce38b79b8baf4d8b61674`.
All 188 retained values reproduced exactly, two independent staging trees were
byte-identical, focused lifecycle and publication contracts passed, and public
report count remained zero. Dual coding-agent reviews closed all findings;
dual terminal verifiers then passed without actionable findings.

The package stops at `HOLD-INDEPENDENT-HUMAN-REVIEW`. The approval list remains
empty, approval-lock and release-transfer roots remain null, and publication,
export, vendoring, and WEPPcloud transfer remain prohibited. Distinct qualified
human scientific and reproduction/publication review, finding disposition,
and a third distinct assurance-steward approval are required before an approval
lock may be created.

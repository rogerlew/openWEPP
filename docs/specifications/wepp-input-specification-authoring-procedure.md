# WEPP Input Specification Authoring Procedure

Status: Active
Last updated: 2026-05-20
Scope: openWEPP WEPP input-file specifications under
`docs/specifications/wepp-input-files/specs/`

## Purpose

Define a reusable, mandatory workflow for authoring and maintaining a
comprehensive, openWEPP-owned WEPP input specification corpus that is:

1. source-authoritative and citation-backed,
2. complete across hillslope, watershed, and sidecar surfaces,
3. aligned with parser-contract and simulation propagation governance,
4. reviewed and dispositioned with required dual-agent verification.

Principle: correctness over completion. Specification work remains `HOLD` when
coverage or correctness criteria are not satisfied.

This procedure governs specification authoring. Parser contracts remain governed
by:

- `docs/specifications/wepp-input-file-parser-contract-authoring-procedure.md`

## Canonical Locations (Normative)

Canonical WEPP input specification files must be stored in:

- `docs/specifications/wepp-input-files/specs/`

Canonical surface coverage registry must be stored in:

- `docs/specifications/wepp-input-files/input-surface-registry.md`

Canonical parser-contract requirement structure must be stored in:

- `docs/specifications/wepp-input-files/parser-contract-requirements.md`

Work-package artifacts are evidence/workflow records and are not canonical
specification authority.

## Source Authority and Cross-Reference Corpus

Specification claims must be derived from a fixed source hierarchy:

1. WEPP user/documentation references (primary format authority), especially:
   - `/home/workdir/openWEPP/references/vendorable/usersum2024.pdf`
   - `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt`
     (line-addressable extraction aid when present)
2. openWEPP reference corpus for technical context and invariants:
   - `/home/workdir/openWEPP/references/50201000/`
   - `/home/workdir/openWEPP/references/annotated_bibliography.md`
   - `/home/workdir/openWEPP/references/vendorable/`
   - `/home/workdir/openWEPP/references/copyrighted/` (local cache when available)
3. Static legacy WEPP implementation provenance (secondary format behavior
   authority):
   - `/workdir/wepp-forest/src/readin.for`
   - `/workdir/wepp-forest/src/input.for`
   - `/workdir/wepp-forest/src/wshinp.for`
   - `/workdir/wepp-forest/src/wshini.for`
   - `/workdir/wepp-forest/src/verchk.for`
   - `/workdir/wepp-forest/src/irrig.for`
   - `/workdir/wepp-forest/src/depirr.for`
   - `/workdir/wepp-forest/src/snowd.for`
   - `/workdir/wepp-forest/src/frzng.for`
   - `/workdir/wepp-forest/src/frostn.for`
   - `/workdir/wepp-forest/src/chnpar.for`
   - `/workdir/wepp-forest/src/chnvar.for`
   - `/workdir/wepp-forest/src/chnero.for`
   - `/workdir/wepp-forest/src/impyr.for`
   - `/workdir/wepp-forest/src/impflo.for`
   - `/workdir/wepp-forest/src/impday.for`
   - `/workdir/wepp-forest/src/impsvb.f90`
   - `/workdir/wepp-forest/src/impris.f90`
   - `/workdir/wepp-forest/docs/` (legacy process notes and interface behavior context)
4. Existing modern parser/spec implementations (secondary provenance):
   - `/workdir/wepppy/wepppy/weppcloud/routes/usersum/input-file-specifications/`
   - `/workdir/wepppy/wepppy/nodb/core/climate_input_parser.py`
   - `/workdir/wepppy/wepppy/nodb/core/wepp_input_parser.py`
   - `/workdir/wepppyo3/cli_revision/src/`
   - `/workdir/wepppyo3/watershed_abstraction/src/`
   - `/workdir/wepppyo3/wepp_interchange/src/`
   - `/workdir/wepppy/docs/` and `/workdir/wepppyo3/docs/` for supplemental
     context

Rules:

1. `usersum2024.pdf` is the primary format baseline when a surface is
   documented there.
2. `wepp-forest` static behavior is used to resolve ambiguities or undocumented
   branches and must be labeled as legacy-derived provenance.
3. `wepppy` and `wepppyo3` are implementation references, not automatic
   authority, unless their behavior is independently anchored to higher-ranked
   sources.
4. If sources disagree, record the conflict explicitly and keep status `HOLD`
   until disposition rationale is documented.

## Mandatory usersum2024 Extraction Checklist

Each specification cycle must explicitly check `usersum2024` for applicable
tables/sections before falling back to legacy-code provenance.

Minimum checklist:

1. Climate format table(s) (Table 1).
2. Hillslope slope format table(s) (Table 2).
3. Soil format table(s) (Table 3).
4. Plant/management format table(s) (Table 16).
5. Irrigation scheduling sidecar table(s) (Table 19 where applicable).
6. Watershed channel slope/input table(s) (Table 24).
7. Impoundment format table(s) (Table 28).
8. Sidecar sections for:
   - `pmetpara.txt`
   - `frost.txt`
   - `tc.txt`
   - `chan.inp`

If a surface in the registry is not explicitly documented in `usersum2024`, the
spec must:

1. record `usersum2024` coverage gap as `[DIRECT]`, and
2. use ranked fallback sources (legacy code, then modern implementations) with
   explicit provenance labels.

## Comprehensive Coverage Baseline

Coverage is defined by the canonical input-surface registry.

At minimum, the specification corpus must provide explicit coverage for all
`planned` and `active` surfaces in:

- `docs/specifications/wepp-input-files/input-surface-registry.md`

Initial canonical mapping target set:

1. `climate-file.spec.md` for `.cli`
2. `soil-file.spec.md` for `.sol`
3. `plant-file.spec.md` for `.man` (transitional canonical file for
   management; rename/split requires explicit migration disposition)
4. `slope-file.spec.md` for `.slp`
5. `watershed-structure-file.spec.md` for `.str`
6. `watershed-channel-file.spec.md` for `.chn`
7. `watershed-impoundment-file.spec.md` for `.imp`
8. `irrigation-depletion-file.spec.md`
9. `irrigation-fixeddate-file.spec.md`
10. `pmetpara.spec.md`
11. `snow.spec.md`
12. `frost.spec.md`
13. `gwcoeff.spec.md`
14. `phosphorus.spec.md`
15. `wepp-ui.spec.md`
16. `tc.spec.md`
17. `tcr.spec.md`
18. `lcwb.spec.md`
19. `chaninp.spec.md`

`unsupported` surfaces in the registry still require explicit rationale and
successor subsystem linkage in registry/disposition artifacts.

## Required Evidence and Citation Rules

Every non-trivial specification claim must include evidence anchors.

Required:

1. Evidence label per claim:
   - `[DIRECT]` directly observed in source text/code/output
   - `[INFERENCE]` reasoned from evidence
2. Evidence mode label per spec/review artifact:
   - `Static` for read/reasoned work
   - `Ran` for execution-based evidence
3. Citation style:
   - file path + section/table/page when from PDF or markdown references
   - file path + line reference when from source code
4. Provenance tagging for conflict resolution:
   - `usersum2024`, `legacy-code`, `wepppy`, `wepppyo3`, `literature`

## Required Section Set Per Specification File

A specification draft is review-ready only when all sections below exist and
are populated.

1. Header metadata:
   - `spec_id`, `surface_id`, `status`, `owner`, `spec_version`,
     `last_updated_utc`
2. Surface scope and applicability:
   - file surface, run mode/hillslope/watershed applicability
3. Version/datver applicability matrix.
4. Record grammar and line-by-line format definition.
5. Field dictionary table with canonical WEPP symbol names, units, types,
   cardinality, requiredness, and alias mapping for openWEPP boundary names.
6. Conditional branches and optional sections (for example sidecar-present vs
   sidecar-absent behaviors).
7. Cross-file consistency constraints and coupling dependencies.
8. Defaulting and missing-file behavior, including typed error expectations.
9. Example snippets (minimal valid, maximal representative, invalid cases).
10. Gap/conflict register with explicit `HOLD` conditions when unresolved.
11. Parser-contract handoff map referencing target `SC-INFILE-*` contract ID.

Any missing section blocks promotion.

## Work-Package Artifact Layout (Required)

Use a dedicated work package for each spec authoring cycle.

Suggested artifact layout:

`docs/work-packages/<wp>/artifacts/input-specs/<spec_id>/`

Required files:

1. `spec_ref.md` (canonical spec path + commit SHA under review)
2. `review_agent_a.md`
3. `review_agent_b.md`
4. `disposition.md`
5. `verification_agent_a.md`
6. `verification_agent_b.md`

## Required Dual-Agent Review Gate

Two independent reviews are mandatory for each specification revision.

Independence requirements:

1. Agent A and Agent B receive independent prompts.
2. Agent B does not see Agent A findings before first submission.
3. Findings must be severity-ranked with file/line references.

Reviewer output requirements:

1. Evidence header (`Static` or `Ran`).
2. Findings ordered by severity.
3. Per finding:
   - severity,
   - file path + line,
   - issue statement,
   - impact on specification correctness/completeness,
   - proposed disposition (`accept`, `amend`, `reject`).
4. Final recommendation:
   - `GO`, `GO-WITH-AMENDMENTS`, or `HOLD`.

## Disposition and Verification Gate

After review, author must publish `disposition.md` with one row per finding.

Required fields:

1. `finding_id`
2. `source`
3. `severity`
4. `decision`
5. `action_taken`
6. `artifact_ref`
7. `notes`

Verification is a separate hard gate:

1. Agent A verifies accepted/amended finding closure.
2. Agent B verifies rejected-finding rationale and no regressions.
3. Verification verdicts:
   - `PASS`, `PASS-WITH-NOTES`, `FAIL`.

## Promotion Gate Logic

A specification revision is promotable only when all conditions are true:

1. Required section set is complete.
2. Dual independent reviews completed.
3. Disposition has no missing findings.
4. All high-severity findings are closed or explicitly justified.
5. Both verification agents return `PASS` or `PASS-WITH-NOTES`.
6. Surface mapping is consistent with `input-surface-registry.md`.
7. Target `SC-INFILE-*` handoff mapping is declared.
8. No unresolved source conflicts remain undocumented.

If any condition fails, disposition is `HOLD`.

## Relationship to Parser Contracts

This procedure governs the specification layer. Parser contract authoring is a
separate, downstream gate governed by:

- `docs/specifications/wepp-input-file-parser-contract-authoring-procedure.md`

Parser contracts should not enter final review until their governing spec file
has passed this procedure's promotion gate.

## Minimal Prompt Templates

### Reviewer prompt (A/B)

"Review `<spec-file>` for format correctness, datver/version completeness,
field dictionary quality, cross-file constraints, and source-citation
sufficiency versus usersum2024 + legacy provenance. Return severity-ranked
findings with file/line refs and GO / GO-WITH-AMENDMENTS / HOLD."

### Verifier prompt (A/B)

"Given `<spec-file>` and `disposition.md`, verify accepted/amended findings are
resolved, rejected findings are justified, and no completeness regressions were
introduced. Return closure state per finding and PASS / PASS-WITH-NOTES / FAIL."

## Change Management

1. Changes to this procedure must update linked checklists/templates in the same
   commit.
2. Intentional bypass requires explicit risk acceptance in work-package
   disposition artifacts.

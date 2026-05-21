# 20260520-inspec04-author-wepp-input-spec-watershed-impoundment-001

## Status
- `state`: active
- `date`: 2026-05-20
- `timezone`: UTC

## Objective
Author and disposition canonical WEPP input specification `SPEC-INFILE-WATERSHED-IMPOUNDMENT-IMP-001` for
surface `infile-watershed-impoundment-imp` (.imp).

## Why This Package Exists
The current canonical specs corpus does not contain `docs/specifications/wepp-input-files/specs/watershed-impoundment-file.spec.md` and the
surface remains `planned` in the input-surface registry. A comprehensive
input-spec corpus requires this specification before parser-contract closure.

## Scope
### Included
- Author canonical specification file:
  - `docs/specifications/wepp-input-files/specs/watershed-impoundment-file.spec.md`
- Populate required section set defined by:
  - `docs/specifications/wepp-input-specification-authoring-procedure.md`
- Cross-reference and anchor claims against:
  - `/home/workdir/openWEPP/references/vendorable/usersum2024.pdf`
  - `/workdir/wepp-forest/`
  - `/workdir/wepppy/`
  - `/workdir/wepppyo3/`
- Run required dual-agent review, disposition, and dual-agent verification
  workflow for specification promotion readiness.
- Declare parser-contract handoff mapping to `SC-INFILE-WATERSHED-IMPOUNDMENT-001`.

### Explicitly Out of Scope
- Parser contract drafting for `SC-INFILE-WATERSHED-IMPOUNDMENT-001`.
- Rust/Python parser implementation changes.
- Comparator campaign execution.

## Deliverables
1. Canonical specification:
   - `docs/specifications/wepp-input-files/specs/watershed-impoundment-file.spec.md`
2. Spec review/disposition bundle:
   - `artifacts/input-specs/SPEC-INFILE-WATERSHED-IMPOUNDMENT-IMP-001/spec_ref.md`
   - `artifacts/input-specs/SPEC-INFILE-WATERSHED-IMPOUNDMENT-IMP-001/review_agent_a.md`
   - `artifacts/input-specs/SPEC-INFILE-WATERSHED-IMPOUNDMENT-IMP-001/review_agent_b.md`
   - `artifacts/input-specs/SPEC-INFILE-WATERSHED-IMPOUNDMENT-IMP-001/disposition.md`
   - `artifacts/input-specs/SPEC-INFILE-WATERSHED-IMPOUNDMENT-IMP-001/verification_agent_a.md`
   - `artifacts/input-specs/SPEC-INFILE-WATERSHED-IMPOUNDMENT-IMP-001/verification_agent_b.md`
3. Artifact index update:
   - `artifacts/README.md`

## Dependencies
- `docs/specifications/wepp-input-specification-authoring-procedure.md`
- `docs/specifications/wepp-input-file-parser-contract-authoring-procedure.md`
- `docs/specifications/wepp-input-files/input-surface-registry.md`
- `docs/specifications/wepp-input-files/specs/README.md`
- `/home/workdir/openWEPP/references/vendorable/usersum2024.pdf`
- `/workdir/wepp-forest`
- `/workdir/wepppy`
- `/workdir/wepppyo3`

## Phase Plan
### Phase 0 - Source and Coverage Inventory
- Inventory relevant usersum2024 tables/sections and legacy code pathways.
- Capture source conflicts and unresolved gaps as explicit `HOLD` items.

### Phase 1 - Specification Drafting
- Draft `docs/specifications/wepp-input-files/specs/watershed-impoundment-file.spec.md` with complete required section set.
- Build explicit field dictionary and applicability matrix.

### Phase 2 - Dual-Agent Review and Disposition
- Run independent reviewer A and reviewer B passes.
- Disposition every finding with closure evidence.

### Phase 3 - Dual-Agent Verification and Closeout
- Run verification A/B over post-fix draft.
- Set package disposition to `GO`, `GO-WITH-AMENDMENTS`, or `HOLD`.

## Exit Criteria
- `docs/specifications/wepp-input-files/specs/watershed-impoundment-file.spec.md` exists with all required sections populated.
- Dual independent review artifacts are complete.
- Disposition file closes or explicitly justifies every finding.
- Dual verification artifacts are complete with no unresolved high-severity
  findings.
- Handoff mapping to `SC-INFILE-WATERSHED-IMPOUNDMENT-001` is explicit.

## Security Impact and Review Gate
- `security_impact`: `none`
- `dedicated_security_review_required`: `no`
- Rationale: docs/specification package only.

# 20260521-inimpl08-support-peridot-2023-3-slope-format-001

## Status
- `state`: active
- `date`: 2026-05-21
- `timezone`: UTC

## Objective
Add first-class openWEPP support for Peridot-produced `2023.3` hillslope `.slp`
files by updating slope specifications, parser contract semantics, parser
implementation, fixtures/tests, and closeout gate artifacts.

## Why This Package Exists
Peridot currently emits hillslope `.slp` files with `datver=2023.3` and a
3-value OFE metadata row (`azm fwidth elevation`) plus comma-delimited slope
pairs. The current openWEPP slope parser and contract/spec drafts are centered
on legacy/canonical `97.5` grammar and do not yet ratify this first-party
input shape.

## Scope
### Included
- Assess and codify Peridot `2023.3` slope-file grammar and invariants.
- Update slope input specification:
  - `docs/specifications/wepp-input-files/specs/slope-file.spec.md`
- Update slope parser science contract:
  - `docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md`
- Implement parser support in:
  - `crates/openwepp-input-contract/src/parsers/slope.rs`
- Add/expand fixtures and integration tests for `2023.3` acceptance and failure
  behavior:
  - `tests/fixtures/infile/slope/**`
  - `tests/integration/infile_slope_parser_contract.rs`
- Run and record gate evidence (`fmt`, `clippy`, `test`, `deny`).
- Produce review/disposition/verification artifacts for package closeout.

### Explicitly Out of Scope
- Channel bundle `2025.8` parsing (`channels.slp`) and watershed channel parser
  surfaces.
- `.slps` flowpath bundle parsing.
- Non-slope input surfaces.

## Dependencies
- `/workdir/peridot/src/watershed_abstraction/flowpath.rs`
- `/workdir/wepppy/wepppy/topo/watershed_abstraction/slope_file.py`
- `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/slope-file.spec.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md`
- `/home/workdir/openWEPP/docs/work-packages/20260520-inspec01-author-wepp-input-spec-slope-001/`
- `/home/workdir/openWEPP/docs/work-packages/20260520-infile04-author-sc-infile-slope-001/`
- `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl03-implement-sc-infile-slope-parser-001/`

## Intended Write Set
- `docs/specifications/wepp-input-files/specs/slope-file.spec.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md`
- `crates/openwepp-input-contract/src/parsers/slope.rs`
- `tests/integration/infile_slope_parser_contract.rs`
- `tests/fixtures/infile/slope/**`
- package-local artifacts under this work-package directory

## Phase Plan
### Phase 0 - Assessment Freeze
- Produce a Peridot `2023.3` format assessment artifact with direct citations.
- Record compatibility deltas versus existing `97.5` parser/contract behavior.

### Phase 1 - Spec and Contract Update
- Add a normative `2023.3` branch to slope spec and contract applicability
  matrix.
- Define required/optional fields (`elevation` metadata handling) and invariant
  guards for `2023.3` branch.
- Disposition current slope HOLD gap on `2023*` extension acceptance
  (`SLP-GAP-003`) with explicit rationale.

### Phase 2 - Parser Implementation
- Add parser-path support for Peridot `2023.3` grammar:
  - line 3 triple (`azm fwidth elevation`)
  - line 4 (`npts length`)
  - comma-delimited pair token tolerance.
- Preserve existing `97.5` semantics and typed error taxonomy.
- Ensure invariants/guards remain explicit (no silent correction).

### Phase 3 - Tests and Fixtures
- Add positive fixture for valid `2023.3` file from Peridot-compatible shape.
- Add negative fixtures for malformed `2023.3` records (arity/cardinality/
  endpoint/tokenization).
- Extend integration tests to verify strict-mode acceptance policy for
  `2023.3` and typed failures.

### Phase 4 - Gates and Package Closeout
- Run:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Produce dual review/disposition/verification artifacts.

## Deliverables
1. Assessment artifact:
   - `artifacts/peridot-2023-3-slope-format-assessment.md`
2. Updated slope spec and slope parser contract.
3. Parser code + fixtures/tests for `2023.3` support.
4. Gate evidence summary:
   - `artifacts/wave-gate-evidence.md`
5. Review and closure artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/inimpl08_disposition.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Exit Criteria
- `2023.3` slope grammar is explicitly specified and contract-bound.
- Parser accepts valid Peridot `2023.3` slope files with typed output.
- Parser rejects malformed `2023.3` inputs with typed errors.
- Legacy/canonical `97.5` behavior remains covered and non-regressed.
- No unresolved high-severity review findings remain.

## Security Impact and Review Gate
- `security_impact`: `none`
- `dedicated_security_review_required`: `no`
- Rationale: file parser behavior and docs/contracts only; no network/service
  exposure.

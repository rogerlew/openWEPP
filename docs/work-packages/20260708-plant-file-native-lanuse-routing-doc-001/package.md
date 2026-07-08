# Plant File Native Lanuse Routing Documentation

Status: `EXECUTED-COMPLETE-DOC-ALIGNMENT`
Evidence mode: Mixed.
Date: 2026-07-08

## Objective

Update `docs/specifications/wepp-input-files/specs/plant-file.spec.md` so the
public WEPP plant/management-file specification describes the executable
openWEPP-native `ow-lanuse-1` management branch, native forest/native cropland
landuse sentinels, and the extended Lane D `routing_coefficients` plant-record
extension in end-user-legible language.

## Rationale

`SC-INFILE-MANAGEMENT-001`, the management-lanuse authority contract, parser
tests, and fixtures already define the native `ow-lanuse-1` behavior. The public
plant-file spec still reads primarily as the legacy WEPP manual conversion and
contains branch notes that say forest and roads are unsupported without naming
the native carve-outs. End users need a clear rule for when `landuse=3` means
native forest, when `landuse=4` means native cropland, and how the optional
five-value `routing_coefficients` block is authored.

## Scope

In scope:

- Scaffold this package and catalog entry.
- Update only `docs/specifications/wepp-input-files/specs/plant-file.spec.md`
  for the requested native `ow-lanuse-1` and routing-coefficient documentation.
- Make the wording legible to users who write or inspect `.man` files.
- Preserve canonical authority pointers to `SC-INFILE-MANAGEMENT-001` and the
  management-lanuse authority contract.
- Record static provenance, command evidence, review, verification, line-count
  governance, and final disposition.

Out of scope:

- No parser, runtime, test, fixture, `SC-*` contract, or WEPPpy code changes.
- No new coefficient defaults, coefficient calibration, or bridge from legacy
  cropland fields to route coefficients.
- No production routing-policy, Lane D activation, mesh-policy, or consumer-path
  change.
- No broad rewrite of the legacy manual-derived sections beyond the affected
  native carve-outs.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/specifications/wepp-input-files/specs/plant-file.spec.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md`
- `docs/contracts/openwepp-management-lanuse-authority-contract.md`
- This package's `package.md`

On-demand:

- `crates/openwepp-input-contract/src/parsers/management.rs` for parser
  behavior excerpts.
- `tests/integration/infile_management_parser_contract.rs` for executable
  examples.
- `tests/fixtures/infile/management/canonical_forest_nonzero_ow_lanuse_1.man`
  and `tests/fixtures/disturbed_native_route_coefficients/p1.man` for sample
  file layout.
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` for
  routing-consumer authority references.
- Prior D16 route-coefficient source-acquisition package artifacts only if
  provenance of WEPPpy Disturbed producer language is disputed.

## Intended Write Set

- `docs/specifications/wepp-input-files/specs/plant-file.spec.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260708-plant-file-native-lanuse-routing-doc-001/`

## Phase Plan

### Phase A - Scaffold And Source Map

Create package structure, kickoff prompt, and required-reading/source-map
artifacts. Confirm the worktree contains unrelated pre-existing changes and
keep this package separate from them.

### Phase B - Spec Update

Amend the plant-file spec with a concise native `ow-lanuse-1` section near the
parser profile, adjust affected landuse branch notes, and add an example of the
`routing_coefficients` extension. Keep wording user-facing and point authority
claims back to the parser contract.

### Phase C - Checks And Evidence

Run diff hygiene and available markdown lint checks for the touched docs. Record
any skipped or unavailable gates plainly.

### Phase D - Review, Verification, And Disposition

Perform dual local docs reviews and dual local verification passes, disposition
all findings, record line-count governance, final disposition, and worker
handoff.

## Subagent Authorization

Subagent requirement: none.

Subagent authorization: none. This package is small, docs-only, and can be
reviewed and verified locally without delegated work.

## Required Artifacts

- `artifacts/README.md`
- `artifacts/required-reading-map.md`
- `artifacts/implementation.md`
- `artifacts/command-evidence.md`
- `artifacts/gate-results.md`
- `artifacts/review-agent-a.md`
- `artifacts/review-agent-b.md`
- `artifacts/verification-agent-a.md`
- `artifacts/verification-agent-b.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Gates

Required before completion:

- `git status --short --branch` with unrelated dirty files identified.
- Static source cross-check against `SC-INFILE-MANAGEMENT-001`, the
  management-lanuse authority contract, parser tests, and fixtures.
- `git diff --check`.
- Markdown/doc lint for touched docs using available repo tooling.
- Dual review artifacts with finding disposition.
- Dual verification artifacts checking user-legibility, authority alignment,
  gate legitimacy, no code/contract scope creep, and line-count governance.

## Exit Criteria

`EXECUTED-COMPLETE-DOC-ALIGNMENT`:

- The target spec describes `ow-lanuse-1`, native forest `landuse=3`, native
  cropland `landuse=4`, and the optional five-value routing extension in
  end-user-legible language.
- The spec no longer implies that all `landuse=3`/`landuse=4` paths are
  unsupported without naming native-datver carve-outs.
- Authority pointers remain consistent with `SC-INFILE-MANAGEMENT-001` and the
  management-lanuse authority contract.
- Required package gates, review, verification, and disposition are recorded.

`EXECUTED-HOLD-*`:

- Static source review exposes a contradiction with parser/contract authority
  that cannot be resolved by a docs-only spec update.
- Markdown/doc gates fail in a way that requires broader tooling or unrelated
  documentation edits outside this package's write set.

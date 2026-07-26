# Gate Evidence

Evidence class: `Live execution record`

## Admission and source preservation

- Admitted base:
  `f56c3fb541784903bdf6c7df6428fa43f44e42a2`.
- `tools/agents/find-agents --for` resolved repository, package, standards, and
  reference ownership before source installation.
- Commissioned source hashes matched the WEPPcloud ledger before copying and
  matched again at their destination.
- Both reference `SHA256SUMS` manifests pass `shasum -a 256 -c`.
- `source-manifest.json` contains 19 source records; a fresh byte/size
  reconstruction reports `source_manifest_identity=PASS`.
- All imported objects are regular non-executable files; no symlinks or special
  files are present.
- `pdfinfo` reports 10 PDFs, each readable, unencrypted, one or more pages, and
  without embedded JavaScript.

## Ledger and admission validation

- Python standard-library JSON and CSV validation reports:
  `manifest_records=19 identity=PASS target_rows=140 unique=PASS
  return_rows=60 cloud_boundary_rows=12 admission=READY_BOUNDED
  rights=HOLD` at the review snapshot.
- All target IDs are unique, all required fields are populated, and every
  evidence role belongs to the documented seven-value enumeration.
- The CAL-02 JSON emits exactly one allowed verdict and agrees with the
  human-readable record. Its bounded design names the exact physical fixture,
  required files, run period, management sources, allowed transformations,
  arms, and claim limits.
- The 12 WEPPcloud daily-return rows are independently asserted to use the hill
  streamflow/lateral-flow boundary rather than hillslope surface runoff.

## Documentation and packaging

- Pre-verification `markdown-doc lint` scoped to the package, catalog, and
  three authored reference READMEs: 23 files, 0 errors, 0 warnings.
- Pre-verification `markdown-doc validate` over the same scope: 23 files,
  0 errors.
- Source-native vendored synthesis-book QMD is hash evidence and was not
  rewritten or passed through the repository Markdown authoring linter.
- `git check-attr` reports `filter=lfs`, `diff=lfs`, `merge=lfs`, and
  `text=unset` for every imported PDF.
- `git lfs pointer --file` reconstructs valid pointers whose OIDs and sizes
  match the source manifest.
- `git diff --check`: pass.
- Credential-pattern scan over the package and reference subtree: pass.
- Placeholder scan over authored package/reference Markdown: pass.
- Publication state at review:
  `HOLD_RIGHTS_REVIEW_FOR_THIRD_PARTY_MATERIAL`; Bill's commissioned files were
  cleared and no publication occurred before review completion.
- Superseding publication authority: on 2026-07-26 the operator explicitly
  confirmed redistribution permission for the retained third-party set and
  directed commit/push. Final manifest state:
  `CLEARED_BY_OPERATOR_CONFIRMATION_2026-07-26`.

## Selected-gate rationale

No Rust, schema, fixture, executable, or production behavior changed. Rustfmt,
Clippy, Nextest, coverage, CRAP, cargo-deny, and workspace build gates are not
applicable. The selected gates cover source identity, untrusted-document
structure, target typing, documentation, local packaging, and LFS routing.

## Review evidence

- Reviewer A: final `PASS`; all scientific/source-classification findings
  corrected.
- Reviewer B: final `PASS_WITH_PUBLICATION_HOLD`; deterministic admission,
  seasonal-deciduous baseline labeling, constant-cover prohibition,
  hill-streamflow boundary, typed-ledger completeness, LAI correction, and
  publication hold confirmed.
- `finding-disposition.md` disposes every finding. The final gate refresh closes
  Reviewer B's stale-evidence finding.

## Terminal verification and path reconciliation

- Verifier A: `PASS_WITH_PUBLICATION_HOLD`; independently checked source
  identity, ledger structure, return boundaries, deterministic admission,
  rights state, documentation, LFS, and exact path attribution.
- Verifier B: `PASS_WITH_PUBLICATION_HOLD`; independently reconstructed the
  scientific classification and every selected terminal gate with no P1/P2
  finding.
- The verifier snapshot found 47 CAL-01 paths within the declared package,
  reference, and catalog write set. Roadmap/backlog/CAL-02 scaffold paths are
  prior campaign work; `papers/0001-openwepp-architecture/manuscript.md` and
  `code-viz/` are unrelated user changes. No CAL-01 production-code change
  exists.
- The two verifier-receipt files were written after the read-only snapshots;
  the final documentation, structure, credential, placeholder, and diff gates
  were rerun over the completed artifact set: 25 Markdown files with 0 lint
  errors, 0 warnings, and 0 validation errors; 18/18 required artifacts; JSON
  pass; CSV 140/140 unique; credential, placeholder, and diff checks pass.

## Post-closeout source recovery

Operator evidence identified the report-linked runs as
`unassailable-sensuousness/disturbed9002` for Hubbard Brook and
`clean-burning-griddle/disturbed9002` for Santee. Live read-only inspection on
`wepp1` confirmed the coordinates, selected `p1`/`H1` and `p2`/`H2` surfaces,
100-year run controls, exact climate seed `12345`, selected outputs, and
WEPPcloud return-period JSON.

The retained 60 source-native files total `33,389,530` bytes and pass both site
`SHA256SUMS` manifests. Live read-only inspection on BLARHG identifies
`C:\WEPP\wepp\wepp_2012.exe` as `1,597,440` bytes with SHA-256
`6104a3440624ad54aa6c3660794280adfd600d4a11b98559c6205a73cd47fc3f`.

This evidence supersedes the common-forcing substitution with exact
site-specific source forcing while preserving `READY_BOUNDED`: Bill's manually
converted Windows project and Windows outputs remain missing.

Subsequent operator direction established that the Windows project is not a
campaign prerequisite. Inspection confirms the retained `p1.man` and `p2.man`
files disable growth, senescence, and decomposition and are the exact
site-specific WEPPcloud constant-cover inputs Bill describes transcribing.
They are admitted as analytical comparators; the missing Windows artifacts now
limit only historical byte/output-equivalence claims.

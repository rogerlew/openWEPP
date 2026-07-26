# CANOPY-CAL-01 Intent Plan

Evidence class: `Admitted execution record`

Admitted base: `f56c3fb541784903bdf6c7df6428fa43f44e42a2`

Admitted on: `2026-07-26`

## Intended outcome

Preserve the commissioned Elliot evidence and supporting literature under
stable identities, classify every load-bearing canopy, biomass, residue,
runoff, and sediment target, and issue a fail-closed CAL-02 reproduction
admission verdict. No model behavior, fixture, science contract, or production
default is changed.

## Declared terminal write set

- `docs/work-packages/README.md`
- `docs/work-packages/20260726-canopy-cal-01-source-target-ledger-001/**`
- `references/canopy_phenology/**`

The pre-existing edits to `papers/0001-openwepp-architecture/manuscript.md` and
the untracked `code-viz/` tree are operator-owned and protected from this
package. The already-scaffolded campaign roadmap, backlog, and CAL-02 package
are context only and are not part of this execution's terminal diff.

## Source actions

- Copy the three commissioned Elliot files byte-for-byte from the WEPPcloud
  reference archive after verifying their published SHA-256 identities.
- Move the seven operator-supplied PDFs and the WEPP user-requirements Markdown
  transcription from `~/Downloads` into the literature subtree with normalized
  bibliographic filenames; ignore `.DS_Store` and the unrelated
  `wepp-figures/` image directory.
- Copy source-native Hubbard Brook synthesis-book material from the clean clone
  at commit `3bb5b43e1429172b8d002e4b002d6a31db694ad1`.
- Acquire missing cited literature only from a publisher, DOI landing page,
  institutional repository, or author-hosted primary source. Retain unavailable
  items in the missing-source bundle rather than substituting secondary prose.

## Gate plan

1. Verify regular-file type, path confinement, media type, absence of embedded
   JavaScript, source/destination hash identity, and Git LFS routing.
2. Visually inspect and text-extract the report and retained papers used by
   load-bearing rows.
3. Validate the source manifest and CAL-02 admission JSON; validate the target
   CSV against its documented enumerations and required fields.
4. Check Markdown structure and local links, scan the package/reference diff
   for placeholders and credential-like material, and run `git diff --check`.
5. Reconcile every changed path against this intent, disposition two
   independent reviews, then obtain two independent terminal verifications.

Rust, Clippy, Nextest, coverage, and CRAP gates are not applicable because this
package changes no Rust, schema, fixture, or executable behavior.

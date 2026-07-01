# Disposition

Status: `UPDATED`

Final disposition: `EXECUTED-COMPLETE-DRAFT-SPEC-REV2-DUAL-REVIEW-DISPOSITIONED`.

Ran:

- `git diff --check -- docs/architecture docs/work-packages/20260701-wshedarch01-watershed-runtime-architecture-specification-001 docs/work-packages/README.md`
- `wctl doc-lint --path docs/architecture/watershed-runtime-architecture-specification.md`

Result:

- No whitespace errors.
- `wctl doc-lint` exited `0`, but reported `0 files validated, 0 errors, 0
  warnings`; this is recorded as a scoped tooling smoke check rather than a full
  markdown-doc lint pass.

Not run:

- Rust gates. This package is docs-only and does not edit production Rust.
- Full repository markdown-doc lint.

Residual risk:

- WSHEDPERF01 full openWEPP end-to-end evidence now has three stability repeats
  plus one profile run, but legacy/openWEPP timing remains cross-scope until a
  legacy-equivalent openWEPP surface is defined. The draft spec uses the data to
  choose direction and set an engineering budget, not to claim speedup/parity.

Revision 1 note:

- User-directed lesson from hillslope performance work is now encoded: watershed
  performance work is a ground-up runtime rewrite with full deletion of the
  existing runtime and obsolete old-surface tests after replacement, not an
  incremental compatibility-runtime optimization program.

Revision 2 note:

- Dual-review findings are accepted and dispositioned in
  `artifacts/dual-review-disposition.md`.
- The spec now requires fail-closed latest-event payload semantics, current
  WSHEDPERF repeat evidence, cross-scope legacy comparison wording, ADR-owned
  `--jobs` default policy, consumer-path proof, protected-coverage restoration
  for deletion packages, and required Rust closure gates for W2-W5.

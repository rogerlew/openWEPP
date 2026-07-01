# Disposition

Status: `UPDATED`

Final disposition: `EXECUTED-COMPLETE-DRAFT-SPEC-REV4-CLAUDE-STATIC-VERIFICATION-DISPOSITIONED`.

Ran:

- `git diff --check -- docs/architecture docs/ROADMAP.md docs/work-packages/20260701-wshedarch01-watershed-runtime-architecture-specification-001 docs/work-packages/README.md`
- `test -z "$(rg -n '[ \t]+$' docs/work-packages/20260701-wshedarch01-watershed-runtime-architecture-specification-001/artifacts/claude-static-verification-disposition.md)"`
- `wctl doc-lint --path docs/architecture/watershed-runtime-architecture-specification.md`
- `wctl doc-lint --path docs/ROADMAP.md`

Result:

- No whitespace errors.
- No trailing whitespace found in the new Claude disposition artifact.
- `wctl doc-lint` exited `0` for both scoped paths, but each reported `0 files
  validated, 0 errors, 0 warnings`; this is recorded as a scoped tooling smoke
  check rather than a full markdown-doc lint pass.

Not run:

- Rust gates. This package is docs-only and does not edit production Rust.
- Full repository markdown-doc lint.

Residual risk:

- WSHEDPERF01 full openWEPP end-to-end evidence now has three stability repeats
  plus one profile run, but legacy/openWEPP timing remains cross-scope until a
  legacy-equivalent openWEPP surface is defined. The draft spec uses the data to
  choose direction and set an engineering budget, not to claim speedup/parity.
- WSHEDPERF01 full-chain evidence used `--legacy-sidecar-discovery`; Revision 4
  records that discovery-on/off runs are different benchmark surfaces unless
  explicitly labeled and justified.

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

Revision 3 note:

- The spec now defines the watershed fixture ladder and auditability rule:
  arboreal-dendrite is smoke/baseline only, carnivorous-adobo is the preferred
  near-term 32-hillslope development fixture, larger 1,000+ hillslope fixtures
  are required after runtime progress, and all adopted fixtures must be
  committed to the repository with provenance before they can close persistent
  gates.

Revision 4 note:

- Claude static verification is dispositioned in
  `artifacts/claude-static-verification-disposition.md`.
- The spec now distinguishes `legacy-sidecar-discovery-on`,
  `canonical-sidecar-discovery-off`, and `strict-committed-fixture`
  measurement modes, labels WSHEDPERF01 as discovery-on, adds roadmap queue
  activation, annotates `chan_out`, and makes latest-event `NoEvent`
  classification a contract-first follow-up.

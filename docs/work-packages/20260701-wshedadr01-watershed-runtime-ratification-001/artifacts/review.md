# WSHEDADR01 Review

Status: `UPDATED`

Static:

- WSHEDARCH01 Revision 4 leaves three explicit ADR-owned decisions open:
  public entrypoint, `--jobs` default, and canonical benchmark mode.
- ADR-0004 keeps subprocess-per-hillslope as the watershed orchestration
  boundary.
- ADR-0006/ADR-0020 keep `openwepp-cli-watershed` in the simulation binary tier
  and `openwepp-cli-totalwatsed3` in the output-aggregation tier.
- WSHEDPERF01 full-chain evidence used `--legacy-sidecar-discovery`, so future
  canonical benchmark claims need a discovery-off committed-fixture surface.

Decision basis:

- Keep `openwepp-cli-watershed` as the public watershed runtime entrypoint to
  avoid an unnecessary new production binary and preserve the existing binary
  taxonomy.
- Default `--jobs` to `1` for deterministic, host-independent behavior; require
  explicit positive `--jobs N` for CPU scaling.
- Make `strict-committed-fixture` the canonical benchmark and ratification mode
  so persistent gates are auditable and do not depend on `/wc1` or legacy
  sidecar discovery.

Dual-review follow-up:

- Primary correctness review (`rust_code_reviewer`
  `019f1ee2-c9c5-73d0-bb75-481443ead4ab`) completed with two findings:
  premature completion claim relative to pending package artifacts and stale
  ROADMAP pre-ratification wording.
- Secondary QA review (`rust_qa_reviewer`
  `019f1ee2-e8f8-7e13-a8aa-b8dfb7edd952`) independently reported the same two
  findings.
- Both findings are accepted and fixed in
  `artifacts/review-disposition.md`.

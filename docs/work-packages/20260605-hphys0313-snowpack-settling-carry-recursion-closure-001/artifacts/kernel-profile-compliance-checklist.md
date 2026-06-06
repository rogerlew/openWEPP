# Kernel profile compliance checklist

Status: complete

Evidence mode: static

Static:

- Contract-first sequencing: complete; `SC-SNOWFREEZE-001#INV-SNOWFREEZE-038`
  and `SC-WATBAL-001#INV-WATBAL-086` precede diagnostics and disposition.
- Contract-derived tests: complete; focused HPHYS0313 contract tests are
  registered and passing.
- Canonical authority: complete; package-local observations are evidence only,
  with pinned-baseline source lines cited separately.
- Baseline provenance: complete; fixed comparator commit
  `47ac4c32faeea81bb99081f955a14c38b815ef4d` and
  `/workdir/wepp-forest_260430_baseline/src/snowd.for` are recorded.
- Typed/fail-closed posture: complete for the diagnostic runner; missing
  required source lines and paired evidence fail closed.
- Production edit gate: complete; no production edit was authorized or made.
- Disposition posture: `HOLD`; branch-gated hourly snowfall input lineage and
  recursive earlier-year carry lineage remain follow-up work.

Ran:

- See `gate-results.md` for validation commands.

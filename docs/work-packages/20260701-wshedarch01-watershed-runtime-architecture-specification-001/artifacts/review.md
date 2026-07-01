# WSHEDARCH01 Review

Status: `UPDATED-REV4`

Static:

- WSHEDPERF01 records pinned legacy full watershed wall `0:07.86`, openWEPP
  routed-stage wall `0:00.07-0:00.08`, and three stable openWEPP full
  command-chain repeats at `1:02.38`, `1:01.41`, and `1:01.06` (`avg
  1:01.62`) plus one full-chain profile run at `1:02.07`.
- Current watershed routing still seeds and executes through
  `WatershedWritebackSurface` maps keyed by `BoundarySymbol`.
- ADR-0004 keeps subprocess-per-hillslope as the accepted process boundary.

Conclusion:

- The draft spec should select bounded subprocess fanout as the first
  architecture lever.
- The draft spec should avoid the hillslope perf mistake of under-aggressive
  refactor scope: watershed work is a ground-up runtime rewrite, not
  old-runtime hardening.
- Typed network-frame replacement and full deletion of the symbol/writeback
  runtime remain necessary. The worker-pool/supervisor work is first because of
  walltime evidence, but deletion is a current architecture requirement, not an
  optional cleanup.
- Tests tied only to obsolete internal surfaces may be deleted with a manifest
  during the full-deletion package; protected science, topology, output, and
  user-facing assertions must be backfilled against the new runtime.
- The spec must remain draft until ADR ratification of public entrypoint/default
  `--jobs` policy and implementation packages prove the required consumer-path,
  determinism, deletion, and closure gates.

Dual-review follow-up:

- Primary and secondary review findings are dispositioned in
  `artifacts/dual-review-disposition.md`.
- Revision 2 accepts and fixes the identified payload-validation,
  benchmark-truthfulness, cross-scope comparison, default-policy, consumer-path,
  deletion-coverage, and closure-gate gaps.

Fixture-strategy follow-up:

- Revision 3 adds a fixture ladder:
  - arboreal-dendrite remains tiny smoke/baseline evidence;
  - carnivorous-adobo (`/wc1/runs/ca/carnivorous-adobo/wepp`) is the preferred
    next development fixture because it is small but has 32 hillslopes;
  - larger 1,000+ hillslope fixtures are required after worker-pool/runtime
    progress;
  - fixtures adopted for gates, recurring benchmarks, ratification, regression,
    or release-readiness must be committed to the repository with provenance.

Claude static verification follow-up:

- Claude's static verification review is dispositioned in
  `artifacts/claude-static-verification-disposition.md`.
- Revision 4 accepts and fixes the sidecar-discovery benchmark-scope gap,
  ROADMAP queue gap, and `chan_out` naming note.
- Revision 4 keeps the latest-event `NoEvent` question open intentionally, but
  moves it into explicit contract-first W2/open-question language.

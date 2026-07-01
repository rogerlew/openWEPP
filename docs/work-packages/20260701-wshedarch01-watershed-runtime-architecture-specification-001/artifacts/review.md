# WSHEDARCH01 Review

Status: `UPDATED-REV2`

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

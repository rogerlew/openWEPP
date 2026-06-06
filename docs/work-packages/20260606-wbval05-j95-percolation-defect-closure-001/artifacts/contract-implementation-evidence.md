# Contract Implementation Evidence

Status: complete

Evidence mode: static

Purpose: record canonical `SC-*` amendments or explicit no-change authority
findings before any production correction.

Static:

- Amended `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
  to contract version 29.
- Added WBVAL05 language to the WB18 algorithm: an already-published finite
  non-negative `wb12_infiltration` surface is authoritative for same-pass
  percolation ingress; WB18 reconstructs WB14/WB12 liquid partition only when
  the published surface is absent.
- The amendment preserves snow/runoff fail-closed ownership. Invalid projected
  snow state is not canonicalized by WB18; after correction the four target
  runs fail at WB14 runoff with `snow.runtime_swe < 0`.

Ran:

- Not applicable; contract implementation evidence is static file evidence.

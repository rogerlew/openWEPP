# PERFDEEP09 Zero-Cost Disabled Proof

Status: complete.
Evidence class: Static + Ran.

PERFDEEP09 did not add or activate any direct-frame, dense-island, trace,
audit, indexed-shadow, or publication cutover machinery. The retained change is
on the always-on compatibility/default path and removes unnecessary guard scan
work from that path.

Default-disabled runtime evidence:

- Final H2637 command explicitly unset:
  `OPENWEPP_PERFDEEP02_FRAME_ISLAND`,
  `OPENWEPP_PERFDEEP03_LANE_DENSE_STATE`,
  `OPENWEPP_PERFDEEP02_FRAME_ROUNDTRIP_PATH`,
  `OPENWEPP_INDEXED_SHADOW_REPORT_PATH`,
  `OPENWEPP_SYMBOL_REGISTRY_AUDIT_PATH`, and
  `OPENWEPP_HPHYS0245_TRACE_PATH`.
- Final median was `635.65 s`, below the `676.67 s` gate and below the no-edit
  control `682.65 s`.

Static proof:

- Candidate 1 touched opt-in-adjacent registry lookup and was reverted.
- Retained code changes only
  `ensure_no_overflow_indexed_symbol_roots_for_decomposition`, replacing
  repeated root scans with a local stack array and one pass over matching
  slot/crop symbols.
- No dense/direct-frame structures are constructed, refreshed, flushed, or
  published by the retained patch.
- No PERFDEEP environment variable default changed.

Conclusion: the known opt-in plumbing remains fail-closed when disabled, and
PERFDEEP09 removes an always-on default-path guard cost rather than adding new
disabled-path work.

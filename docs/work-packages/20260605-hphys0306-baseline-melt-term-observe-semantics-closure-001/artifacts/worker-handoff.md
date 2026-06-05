# Worker Handoff

Status: complete

Evidence mode: ran

Static:

- HPHYS0306 executed the HPHYS0305 required continuation.
- Production physics edits remain unauthorized.

Ran:

- Added canonical branch-active observe semantics in `SC-WATBAL-001`.
- Reclassified all nine H1/H7/H39 target windows from HPHYS0305 evidence.
- Eight windows route to `melt-call-mask` /
  `branch-active-mask-hold`.
- H39 first-2013 routes to `same-hour-multi-source:cmelt,snodpt` /
  `same-hour-multi-source-hold`.
- No row authorizes downstream compensation.

## Required Continuation

The next work package should prioritize the eight-window melt-call mask
divergence before numeric melt-term correction. Scope:

- Compare fixed-baseline `winter.for` conditions that call `melt.for` against
  openWEPP `snow_hourly_melt_branch_active` publication logic.
- Add contract tests for branch activation inputs and state predicates.
- Correct openWEPP branch activation only if baseline-authoritative source-line
  provenance identifies an openWEPP defect.
- Preserve H39 first-2013 same-hour `cmelt`/`snodpt` as a separate
  source-ordering lane if the branch activation package does not cover it.
- Keep WB13/WB17/WB18/WB19/WB12 compensation prohibited.
